use crate::ui::{UIState, RenderBackend, InputModule, Point2D};

struct Winit {

}

impl RenderBackend for Winit {
    fn render(&self, surface: super::RenderSurface) {

    }
}

impl InputModule for Winit {
    fn get_state(&self) -> UIState {
        UIState {
            keypress: 0,
            mouse: Point2D {
                x: 0,
                y: 0,
            },
            mouse_leftclick: false,
            mouse_rightclick: false,
        }
    }
}

pub struct RenderBackends {}

// the initialization function for each backend
impl RenderBackends {
    pub fn winit() -> Winit {
        Winit {
        }
    }
}

