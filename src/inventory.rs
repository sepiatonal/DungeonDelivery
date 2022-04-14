use lore_render::*;
use crate::*;
use gui::*;

pub struct Inventory {
    background: gui::Button,
    items: Vec<((u8, u8), Item)>,
}

impl Inventory {
    pub fn new() -> Self {
        let background = Button {
            x: 453.0,
            y: 51.0,
            z: 0.1,
            width: 186.0,
            height: 300.0,
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

    pub fn add_item(&mut self, item: Item) {
        
    }
}

pub struct Item {
    button: Button,
    blueprint: &'static ItemBlueprint,
}

impl Item {
    fn from_blueprint(blueprint: &'static ItemBlueprint) -> Self {
        let button = Button {
            z: 0.2,
            width: blueprint.image_dimensions.0,
            height: blueprint.image_dimensions.1,
            name: blueprint.name.clone(),
            img_path: blueprint.image_path.clone(),
            ..Default::default()
        };

        Self {
            button,
            blueprint,
        }
    }
}

pub struct ItemBlueprint {
    name: String,
    image_path: String,
    image_dimensions: (f32, f32),
    inv_dimensions: (u8, u8),
}

//enums

enum ItemType {
    Weapon,
    Offhand,
    Armor,
    MiscEquip,
    Usable,
    Other
}