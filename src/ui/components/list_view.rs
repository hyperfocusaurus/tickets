use crate::ui::{RenderSurface, UIState, RenderTreeNode, Paintable};
use std::sync::{Arc, Mutex};

struct ListView {
    dirty: bool
}

impl Paintable for ListView {
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

pub fn list_view(children: Vec<RenderTreeNode>) -> RenderTreeNode {
    RenderTreeNode {
        paintable: Arc::new(Mutex::new(ListView{
            dirty: true,
        })),
        children: children.into_iter().map(Box::new).collect(),
    }
}

