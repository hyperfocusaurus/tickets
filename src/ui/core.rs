// UI functions needed:
// layout functionality... grid based?
// draw tabs with content view
// draw "card" (title, body, etc.)
// draw rectangle
// render text
// build up more complex components based on rectangles + text, using composition
// "paint" method for each component, paints any sub-components
// "update" method for each component, updates the component state
// work out if need to repaint after update call (check if component has become dirty)
// co-operative - component marks itself as dirty during update

use std::sync::{Arc, Mutex};
use std::thread;

pub type Scalar = u64;
pub type Color = u32;

pub struct Point2D {
    pub x: Scalar,
    pub y: Scalar
}

pub struct BoxConfig {
    border_color: Color,
    border_width: Scalar,
    background_color: Color,
    text_color: Color,
}

pub struct RenderSurface {
    width: Scalar,
    height: Scalar,
    stride: Scalar,
    framebuffer: Box<Vec<u32>>,
}

pub trait RenderBackend {
    fn render(&self, surface: RenderSurface);
}

impl RenderSurface {
    pub fn new(
        width: Scalar,
        height: Scalar,
        stride: Scalar
        ) -> Self {
        Self {
            width,
            height,
            stride,
            framebuffer: Box::new(vec![0;width as usize * height as usize]),
        }
    }
    fn get_subsurface(&self, _origin: Point2D, _width: Scalar, _height: Scalar, _box_config: BoxConfig) {
        todo!("get subsurface");
    }
    fn render_rect(&self, _origin: Point2D, _width: Scalar, _height: Scalar, _box_config: BoxConfig) {
        todo!("Render rectangle!");
    }
    fn render_text(&self, _origin: Point2D, _text: String, _box_config: BoxConfig) {
        todo!("Render text!");
    }
}

pub struct UIComponent {
    children: Vec<UIComponent>
}

pub enum KeyboardModifiers {
    Alt     = 0b0001,
    Ctrl    = 0b0010,
    Super   = 0b0100,
    Shift   = 0b1000,
}

pub struct UIState {
    pub mouse: Point2D,
    pub mouse_leftclick: bool,
    pub mouse_rightclick: bool,
    pub keypress: u16, // upper 8 bits are modifier mask, lower 8 bits are key code
}

// each UI Component should implement this trait
pub trait Paintable {
    fn paint(&self, rend: RenderSurface);
    fn is_dirty(&self) -> bool;
    fn mark_dirty(&self);
    fn update(&self, state: &UIState);
}

pub struct RenderTreeNode {
    pub children: Vec<Box<RenderTreeNode>>,
    pub paintable: Arc<Mutex<dyn Paintable + Send>>,
}

impl RenderTreeNode {
    fn get_children(&self) -> &Vec<Box<RenderTreeNode>> {
        return &self.children;
    }
    fn add_child(&mut self, child: Box<RenderTreeNode>) { 
        self.children.push(child);
    }
}

impl Paintable for RenderTreeNode {
    fn update(&self, state: &UIState) {
        let paintable = self.paintable.lock().unwrap();
        paintable.update(state);
        for child in &self.children {
            child.update(&state);
        }
    }
    fn mark_dirty(&self) {
        let paintable = self.paintable.lock().unwrap();
        paintable.mark_dirty(); 
    }
    fn is_dirty(&self) -> bool {
        let paintable = self.paintable.lock().unwrap();
        paintable.is_dirty()
    }
    fn paint(&self, rend: RenderSurface) {
        let paintable = self.paintable.lock().unwrap();
        paintable.paint(rend);
    }
}

// the entrypoint into the UI library - call this once at application startup
pub fn ui_init<RB: RenderBackend + Send, IM: InputModule + Send>(render_backend: RB, rend: RenderSurface, root: RenderTreeNode, input: IM) {
    let root_arc = Arc::new(Mutex::new(root));
    let rend_arc = Arc::new(Mutex::new(rend));
    // start state update loop
    let upd_root = root_arc.clone();
    let upd_rend = rend_arc.clone();
    thread::spawn(move || {
        loop {
            ui_update(*upd_rend, *upd_root, input);
        }
    });
    // start render loop
    let rnd_root = root_arc.clone();
    thread::spawn(move || {
        loop {
            ui_render(rend, *rnd_root);
            // essentially the "blit"
            render_backend.render(rend);
        }
    });
}

pub trait InputModule {
    fn get_state(&self) -> UIState;
}

// todo: does this method really need a `rend`?  I would have thought not...
pub fn ui_update<IM: InputModule>(rend: RenderSurface, root: RenderTreeNode, input: IM) {
    root.update(&input.get_state());
    for child in root.get_children() {
        ui_update(rend, **child, input);
        if child.is_dirty() {
            root.mark_dirty();
        }
    }
}

pub fn ui_render(rend: RenderSurface, root: RenderTreeNode) {
    // paint the current root
    root.paint(rend);
    for child in root.get_children() {
        // descend down the subtree
        ui_render(rend, **child);
    }
}

