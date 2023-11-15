use crate::ui::{RenderSurface, RenderTreeNode, Paintable, UIState};
use std::sync::{Arc,Mutex};

struct Text {
    dirty: bool
}

impl Paintable for Text {
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


pub fn text(str: String) -> RenderTreeNode {
    RenderTreeNode {
        paintable: Arc::new(Mutex::new(Text {
            dirty: true,
        })),
        children: Vec::new(),
    }
}

