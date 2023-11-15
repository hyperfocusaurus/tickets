use crate::ui::{RenderSurface, RenderTreeNode, UIState, Paintable};
use std::sync::{Arc,Mutex};

struct Tabs {
    dirty: bool
}

impl Paintable for Tabs {
    fn update(&self, state: &UIState) {
    }
    fn mark_dirty(&self) {
    }
    fn is_dirty(&self) -> bool {
        self.dirty
    }
    fn paint(&self, rend: RenderSurface)
    {
        ()
    }
}


pub fn tabs(_tabstate: TabState, children: Vec<RenderTreeNode>) -> RenderTreeNode {
    RenderTreeNode {
        paintable: Arc::new(Mutex::new(Tabs {
            dirty: true,
        })),
        children: children.into_iter().map(Box::new).collect(),
    }
}

struct TabContents {
    dirty: bool
}

impl Paintable for TabContents {
    fn update(&self, state: &UIState) {
        
    }
    fn mark_dirty(&self) {
        
    }
    fn is_dirty(&self) -> bool {
        self.dirty
    }
    fn paint(&self, rend: RenderSurface) {
        
    }
}

pub fn tab_contents(tabstate: TabState, tab_title: &'static str, children: Vec<RenderTreeNode>) -> RenderTreeNode {
    RenderTreeNode {
        paintable: Arc::new(Mutex::new(TabContents {
            dirty: true,
        })),
        children: children.into_iter().map(Box::new).collect(),
    }
}

pub struct TabState{
    selected_tab: usize, // refers to the index of the currently-selected tab
}

impl TabState {
    pub fn new() -> TabState {
        TabState {
            selected_tab: 0,
        }
    }
}
