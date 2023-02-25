use tui_tree_widget::{TreeItem, TreeState};

pub struct StatefulTree<'a> {
    pub state: TreeState,
    pub items: Vec<TreeItem<'a>>,
}

impl<'a> StatefulTree<'a> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            state: TreeState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<TreeItem<'a>>) -> Self {
        Self {
            state: TreeState::default(),
            items,
        }
    }

    pub fn first(&mut self) {
        self.state.select_first();
    }

    pub fn last(&mut self) {
        self.state.select_last(&self.items);
    }

    pub fn down(&mut self) {
        self.state.key_down(&self.items);
    }

    pub fn up(&mut self) {
        self.state.key_up(&self.items);
    }

    pub fn left(&mut self) {
        self.state.key_left();
    }

    pub fn right<'b>(&'b mut self) {
        self.state.key_right();
    }

    pub fn toggle(&mut self) {
        self.state.toggle_selected();
    }

    fn items_mut<'b>(&'b mut self) -> &'b mut Vec<TreeItem<'a>> {
        &mut self.items
    }

    pub fn with_selected_leaf<'b>(&'b mut self, f: impl FnOnce(Option<&'b mut TreeItem<'a>>)) where 'a: 'b
     {
        fn traverse<'short, 'long>(path: Vec<usize>, nodes: &'short mut [TreeItem<'long>]) -> Option<&'short mut TreeItem<'long>> where 'long: 'short {
            let first = path.first()?;
            let node = nodes.get_mut(*first)?;
            if path.len() == 1 {
                Some(node)
            } else {
                traverse(path[1..].to_owned(), node.children_mut())
            }
        }

        let res = traverse(self.state.selected(), self.items_mut());

        f(res)
    }
}
