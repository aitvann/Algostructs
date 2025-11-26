use std::{collections::VecDeque, iter, marker::PhantomData, ops::ControlFlow};

pub trait Node<T> {
    #[expect(dead_code)]
    fn data(&self) -> &T;
    #[expect(dead_code)]
    fn data_mut(&mut self) -> &mut T;

    fn children(&self) -> impl Iterator<Item = &Self>;
    #[expect(dead_code)]
    fn children_mut(&mut self) -> impl Iterator<Item = &mut Self>;
}

#[derive(Clone, Debug)]
pub struct Walker<'a, T, N> {
    heap: VecDeque<&'a N>,
    _data: PhantomData<T>,
}

#[derive(Eq, PartialEq, Default, Copy, Clone, Debug)]
pub enum ContinueFlow {
    #[default]
    Forward,
    Skip,
}

impl<'a, T, N: Node<T>> Walker<'a, T, N> {
    pub fn new(tree: &'a N) -> Self {
        Self {
            heap: iter::once(tree).collect(),
            _data: PhantomData,
        }
    }

    #[expect(dead_code)]
    pub fn set(&mut self, tree: &'a N) {
        self.heap.clear();
        self.heap.push_front(tree);
    }
}

impl<'a, T, N: Node<T>> Walker<'a, T, N> {
    pub fn bfs_step<R>(
        &mut self,
        mut f: impl FnMut(&'a N) -> ControlFlow<R, ContinueFlow>,
    ) -> ControlFlow<R, ContinueFlow> {
        self.bfs_step_by_ref(&mut f)
    }

    pub fn bfs_step_by_ref<R>(
        &mut self,
        f: &mut impl FnMut(&'a N) -> ControlFlow<R, ContinueFlow>,
    ) -> ControlFlow<R, ContinueFlow> {
        let Some(current_node) = self.heap.pop_front() else {
            return ControlFlow::Continue(ContinueFlow::Forward);
        };

        let control_flow = f(current_node);
        if !matches!(control_flow, ControlFlow::Continue(ContinueFlow::Skip)) {
            self.heap.extend(current_node.children());
        }
        control_flow
    }

    #[expect(dead_code)]
    pub fn bfs<R>(
        &mut self,
        mut f: impl FnMut(&'a N) -> ControlFlow<R, ContinueFlow>,
    ) -> Option<R> {
        while !self.heap.is_empty() {
            if let ControlFlow::Break(value) = self.bfs_step_by_ref(&mut f) {
                return value.into();
            }
        }

        None
    }

    #[expect(dead_code)]
    pub fn dfs_step<R>(
        &mut self,
        mut f: impl FnMut(&'a N) -> ControlFlow<R, ContinueFlow>,
    ) -> ControlFlow<R, ContinueFlow> {
        self.dfs_step_by_ref(&mut f)
    }

    pub fn dfs_step_by_ref<R>(
        &mut self,
        f: &mut impl FnMut(&'a N) -> ControlFlow<R, ContinueFlow>,
    ) -> ControlFlow<R, ContinueFlow> {
        let Some(current_node) = self.heap.pop_back() else {
            return ControlFlow::Continue(ContinueFlow::Forward);
        };

        let control_flow = f(current_node);
        if !matches!(control_flow, ControlFlow::Continue(ContinueFlow::Skip)) {
            self.heap.extend(current_node.children());
        }
        control_flow
    }

    pub fn dfs<R>(
        &mut self,
        mut f: impl FnMut(&'a N) -> ControlFlow<R, ContinueFlow>,
    ) -> Option<R> {
        while !self.heap.is_empty() {
            if let ControlFlow::Break(value) = self.dfs_step_by_ref(&mut f) {
                return value.into();
            }
        }

        None
    }
}
