use lore_render::*;
use crate::*;
use gui::*;

struct Inventory {
    background: gui::Button,
    items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        let background = Button {
            x: 420.0,
            y: 355.0,
            width: 28.0,
            height: 28.0,
            instance: (0, 0),
            img_path: "assets/ui/frame/inventory.png".into(),
            name: "inventory".to_string(),
            ..Default::default()
        };

        Self {
            background,
            items: Vec::new(),
        }
    }
    
    pub fn show(&mut self, state: &mut State, rendering_instance: &mut RenderingInstance) {
        self.background.create(rendering_instance, state.gui.gui_pl);
    }
    
    pub fn hide(&mut self, state: &mut State, rendering_instance: &mut RenderingInstance) {
        self.background.destroy(rendering_instance);
    }
}

struct Item {
    button: Button,
}