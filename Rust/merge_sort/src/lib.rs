fn merge<T: Ord + Clone>(src_left: &[T], src_right: &[T], dst: &mut [T]) {
    assert_eq!(src_left.len() + src_right.len(), dst.len());

    let mut left_iter = src_left.iter().peekable();
    let mut right_iter = src_right.iter().peekable();
    let mut dst_iter = dst.iter_mut();

    loop {
        match (left_iter.peek(), right_iter.peek()) {
            (Some(l), Some(r)) if l < r => {
                *dst_iter.next().unwrap() = left_iter.next().unwrap().clone()
            }
            (Some(_), Some(_)) => *dst_iter.next().unwrap() = right_iter.next().unwrap().clone(),
            (Some(_), None) => *dst_iter.next().unwrap() = left_iter.next().unwrap().clone(),
            (None, Some(_)) => *dst_iter.next().unwrap() = right_iter.next().unwrap().clone(),
            (None, None) => break,
        }
    }
}

fn merge_by_step<T: Ord + Clone>(src: &[T], dst: &mut [T], step: usize) {
    assert_eq!(src.len(), dst.len());

    let src_chunks = src.chunks(step);
    let dst_chunks = dst.chunks_mut(step);
    for (subsrc, subdst) in src_chunks.zip(dst_chunks) {
        let mid = subsrc.len().min(step / 2);
        merge(&subsrc[..mid], &subsrc[mid..], subdst);
    }
}

pub fn sort_buf<T: Ord + Clone>(data: &mut [T], buf: &mut [T]) {
    assert_eq!(data.len(), buf.len());

    if data.len() > 1 {
        let mut swap = false;
        let mut subarr_size = 2;

        while subarr_size < data.len() * 2 {
            if swap {
                merge_by_step(buf, data, subarr_size);
            } else {
                merge_by_step(data, buf, subarr_size);
            }

            subarr_size *= 2;
            swap = !swap;
        }

        if swap {
            data.clone_from_slice(buf);
        }
    }
}

pub fn sort<T: Ord + Clone>(data: &mut [T]) {
    let mut buf = data.to_vec();
    sort_buf(data, buf.as_mut_slice())
}
