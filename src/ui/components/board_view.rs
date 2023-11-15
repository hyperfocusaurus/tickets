use crate::ui::{Paintable, UIState, RenderSurface, RenderTreeNode};
use std::sync::{Arc, Mutex};

struct BoardView {
    dirty: bool
}

impl Paintable for BoardView {
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

pub fn board_view(children: Vec<RenderTreeNode>) -> RenderTreeNode {
    RenderTreeNode { 
        children: children.into_iter().map(Box::new).collect(), 
        paintable: Arc::new(Mutex::new(BoardView {
            dirty: true,
        }))
    }
}

