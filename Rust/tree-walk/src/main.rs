use std::{fmt, ops::ControlFlow};

use crate::walk::{ContinueFlow, Node, Walker};

mod walk;

#[derive(Clone, Debug)]
struct OwningNode<T> {
    data: T,
    children: Vec<OwningNode<T>>,
}

impl<T> Node<T> for OwningNode<T> {
    fn data(&self) -> &T {
        &self.data
    }

    fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    fn children(&self) -> impl Iterator<Item = &Self> {
        self.children.iter()
    }

    fn children_mut(&mut self) -> impl Iterator<Item = &mut Self> {
        self.children.iter_mut()
    }
}

#[derive(Copy, Clone, Debug)]
struct Data {
    depth: i32,
    sibling: i32,
}

impl From<(i32, i32)> for Data {
    fn from((depth, sibling): (i32, i32)) -> Self {
        Self { depth, sibling }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.depth, self.sibling)
    }
}

fn main() {
    let tree = OwningNode::<Data> {
        data: (1, 1).into(),
        children: vec![
            OwningNode {
                data: (2, 1).into(),
                children: vec![],
            },
            OwningNode {
                data: (2, 2).into(),
                children: vec![
                    OwningNode {
                        data: (3, 1).into(),
                        children: vec![],
                    },
                    OwningNode {
                        data: (3, 2).into(),
                        children: vec![],
                    },
                    OwningNode {
                        data: (3, 3).into(),
                        children: vec![],
                    },
                ],
            },
        ],
    };

    println!("bfs:");
    let mut walker = Walker::new(&tree);
    for i in 0..6 {
        print!("step {i} ");
        _ = walker.bfs_step(|n| {
            println!("node data: {}", n.data);
            ControlFlow::Continue::<(), _>(ContinueFlow::Forward)
        });
    }

    println!();

    println!("dfs:");
    let mut walker = Walker::new(&tree);
    walker.dfs(|n| {
        println!("node data: {}", n.data);
        if n.data.depth >= 2 {
            ControlFlow::Continue::<(), _>(ContinueFlow::Skip)
        } else {
            ControlFlow::Continue::<(), _>(ContinueFlow::Forward)
        }
    });

    println!();

    println!("dfs search:");
    let mut walker = Walker::new(&tree);
    let res = walker.dfs(|n| {
        println!("node data: {}", n.data);
        if n.data.depth + n.data.sibling == 6 {
            ControlFlow::Break(n)
        } else {
            ControlFlow::Continue(ContinueFlow::Forward)
        }
    });
    println!("search res: {res:?}");
}
