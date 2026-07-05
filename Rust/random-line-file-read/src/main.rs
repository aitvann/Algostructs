use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::PathBuf;
use std::range;
use std::range::Range;
use std::range::Range;

use anyhow::Context;
use clap::Parser;
use clap::Subcommand;
use itertools::Itertools;
use lipsum::MarkovChain;
use rand::Rng;
use rand::seq::IteratorRandom;

/// Range of line size in bytes
//const LINE_SIZE_RANGE: std::ops::Range<usize> = 8usize..1024;
// const LINE_SIZE_RANGE: std::ops::Range<usize> = 8usize..256;
const LINE_SIZE_RANGE: std::ops::Range<usize> = 8usize..100 * 2usize.pow(20); // up to 100mb

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Write {
        /// Path to file
        path: PathBuf,

        /// Size of file in bytes
        #[clap(long)]
        #[clap(default_value = "1048576")]
        size: usize,
    },

    Read {
        /// Path to file
        path: PathBuf,
    },
}

fn line_generator<'a>(dictionary: &'a MarkovChain<'_>, size: usize) -> impl fmt::Display + 'a {
    let mut line_remaining = size;
    dictionary
        .iter()
        .take_while(move |word| {
            // Taking space character into account
            let Some(new_line_remaining) = line_remaining.checked_sub(word.len() + 1) else {
                return false;
            };
            line_remaining = new_line_remaining;

            true
        })
        .format(" ")
}

#[derive(Debug)]
struct ByteCounter<W> {
    inner: W,
    counter: usize,
}

impl<W> ByteCounter<W> {
    fn wrap(writer: W) -> Self {
        Self {
            inner: writer,
            counter: 0,
        }
    }

    fn reset(&mut self) {
        self.counter = 0;
    }
}

impl<W: fmt::Write> fmt::Write for ByteCounter<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.counter += s.len();
        self.inner.write_str(s)
    }
}

impl<W: io::Write> io::Write for ByteCounter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let res = self.inner.write(buf);
        if let Ok(bytes) = &res {
            self.counter += bytes;
        }

        res
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

/// Problems with this approach:
/// 1. Unnecessary `String` allocations for every line
/// 2. If and entire file is just one string then we will read the entire file
#[allow(dead_code)]
fn read_random_line1(reader: &mut impl BufRead, rng: &mut impl Rng) -> io::Result<Option<String>> {
    reader.lines().choose(rng).transpose()
}

/// Problems with this approach:
/// 1. If and entire file is just one string then we will read the entire file
#[allow(dead_code)]
#[allow(unused_variables)]
fn read_random_line2(reader: &mut impl BufRead, rng: &mut impl Rng) -> io::Result<Option<String>> {
    let mut buf = vec![];
    reader.read_until(b'\n', &mut buf)?;
    let result = (!buf.is_empty()).then_some(buf);

    // TODO: loop over strings and randomly switch to new ones

    // TODO: convert utf8 error to io::Error
    result
        .map(String::from_utf8)
        .transpose()
        .map_err(io::Error::other)
}

fn copy_range(
    src: &mut (impl BufRead + Seek),
    dst: &mut impl Write,
    range: Range<u64>,
) -> io::Result<()> {
    src.seek(SeekFrom::Start(range.start))?;

    // `ExactSizeIterator` is not implemented for `RangeIter<u64>` for some reason
    // let mut left_to_copy = range.iter().len();
    let mut left_to_copy = range.end - range.start;
    loop {
        let content = src.fill_buf()?;

        // End Of File
        if content.is_empty() || left_to_copy == 0 {
            break;
        }

        dst.write_all(&content[..left_to_copy.min(content.len() as u64) as usize])?;
        let consumed = content.len();
        left_to_copy = left_to_copy.saturating_sub(consumed as u64);
        src.consume(consumed);
    }

    Ok(())
}

#[derive(Debug)]
struct LineRanges<B> {
    buf: B,
    current_line_start: u64,
}

impl<B: BufRead> Iterator for LineRanges<B> {
    type Item = io::Result<Range<u64>>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.buf.skip_until(b'\n');
        let range_size = match res {
            Ok(0) => return None,
            Ok(n) => n,
            Err(e) => return Some(Err(e)),
        };

        let range_start = self.current_line_start;
        self.current_line_start += range_size as u64;
        let line_range = (range_start..self.current_line_start).into();
        Some(Ok(line_range))
    }
}

trait LineRangesExt: Sized {
    fn line_ranges(self) -> LineRanges<Self>;
}

impl<B: BufRead> LineRangesExt for B {
    fn line_ranges(self) -> LineRanges<Self> {
        LineRanges {
            buf: self,
            current_line_start: 0,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.command {
        Command::Write { path, size } => {
            let file = File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)
                .context("open file")?;

            let writer = BufWriter::new(file);
            // let writer = std::io::stdout();
            let mut writer = ByteCounter::wrap(writer);

            let mut dictionary = lipsum::MarkovChain::default();
            dictionary.learn(lipsum::LOREM_IPSUM);

            // Not taking new line character into account
            // because we take space character for every word (including the last one)
            // into account
            let mut file_remaining = size;
            while file_remaining > 0 {
                let line_size = rand::random_range(LINE_SIZE_RANGE);
                let line = line_generator(&dictionary, line_size.min(file_remaining));
                writeln!(writer, "{line}").context("write line")?;
                if writer.counter == 0 {
                    // There are so little space have left that a word can not fit in it
                    break;
                }

                file_remaining = file_remaining.saturating_sub(writer.counter);
                writer.reset();
            }

            writer.flush().context("flushing file")?
        }

        Command::Read { path } => {
            let file = File::options()
                .create(false)
                .read(true)
                .open(path)
                .context("open file")?;

            let mut reader = BufReader::new(file);
            let mut rng = rand::rng();

            // let Some(line) = read_random_line1(&mut reader, &mut rng).context("read lines")? else {
            //     println!("File is empty");
            //     return Ok(());
            // };
            //
            // println!("Random line: {line}");

            // let first_line_range = read_random_line3(&mut reader, &mut rng).unwrap().unwrap();
            // copy_range(&mut reader, &mut io::stdout(), first_line_range).unwrap();

            let line_range = (&mut reader)
                .line_ranges()
                .choose(&mut rng)
                .transpose()
                .context("Choosing line")?;

            let Some(line_range) = line_range else {
                println!("File is empty");
                return Ok(());
            };

            copy_range(&mut reader, &mut io::stdout(), line_range)
                .context("Copy line to stdout")?;
        }
    }

    Ok(())
}
