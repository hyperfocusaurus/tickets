use crate::ui::{Paintable, UIState, RenderSurface, RenderTreeNode};
use std::sync::{Arc, Mutex};

struct Frame {
}

impl Paintable for Frame { 
    fn update(&self, state: &UIState) {

    }
    fn mark_dirty(&self) {
        
    }
    fn is_dirty(&self) -> bool {
        false
    }
    fn paint(&self, rend: RenderSurface) {
        
    }
}

pub fn frame(children: Vec<RenderTreeNode>) -> RenderTreeNode {
    RenderTreeNode {
        paintable: Arc::new(Mutex::new(Frame {
        })),
        children: children.into_iter().map(Box::new).collect(),
    }
}
