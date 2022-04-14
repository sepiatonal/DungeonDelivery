use lore_render::*;
use crate::*;
use gui::*;
use std::borrow::Cow;

pub const INV_WIDTH: u8 = 5;
pub const INV_HEIGHT: u8 = 10;

const ITEM_FRAME_X: f32 = 475.0;
const ITEM_FRAME_Y: f32 = 74.0;
const ITEM_FRAME_SLOT_WIDTH: f32 = 23.0;
const ITEM_FRAME_SLOT_HEIGHT: f32 = 22.0;

pub struct Inventory {
    background: Button,
    items: Vec<((u8, u8), Item)>,
}

impl Inventory {
    pub fn new() -> Self {
        let background = Button {
            x: 453.0,
            y: 51.0,
            z: 0.5,
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

    pub fn item_fits(&self, item: &Item, pos: (u8, u8)) -> bool {
        let (x, y) = pos;
        let (w, h) = item.blueprint.inv_dimensions;
        for (other_pos, other_item) in &self.items {
            let (ox, oy) = other_pos.clone();
            let (ow, oh) = other_item.blueprint.inv_dimensions;

            let mut collides = false;
            if x + w < ox {
                collides = true;
            }
            if ox + ow < x {
                collides = true;
            }
            if y + h < oy {
                collides = true;
            }
            if oy + oh < y {
                collides = true;
            }

            if collides {
                return false
            }
        }

        true
    }

    pub fn add_item(&mut self, state: &mut State, rendering_instance: &mut RenderingInstance, mut item: Item, pos: (u8, u8)) {
        let (x, y) = pos;
        item.button.x = ITEM_FRAME_X + x as f32 * (ITEM_FRAME_SLOT_WIDTH + 1.0);
        item.button.y = ITEM_FRAME_Y + y as f32 * (ITEM_FRAME_SLOT_HEIGHT + 1.0);
        item.button.create(rendering_instance, state.gui.gui_pl);
        self.items.push((pos, item));
    }

    pub fn add_item_anywhere(&mut self, state: &mut State, rendering_instance: &mut RenderingInstance, item: Item) -> Result<(u8, u8), ()> {
        for x in 0..INV_WIDTH {
            for y in 0..INV_HEIGHT {
                if self.item_fits(&item, (x, y)) {
                    self.add_item(state, rendering_instance, item, (x, y));
                    return Ok((x, y));
                }
            }
        }
        Err(())
    }
}

pub struct Item {
    pub button: Button,
    pub blueprint: &'static ItemBlueprint,
}

impl Item {
    pub fn from_blueprint(blueprint: &'static ItemBlueprint) -> Self {
        let button = Button {
            z: 0.0,
            width: blueprint.image_dimensions.0,
            height: blueprint.image_dimensions.1,
            name: blueprint.name.to_string(),
            img_path: blueprint.image_path.to_string(),
            ..Default::default()
        };

        Self {
            button,
            blueprint,
        }
    }
}

pub struct ItemBlueprint {
    name: Cow<'static, str>,
    image_path: Cow<'static, str>,
    image_dimensions: (f32, f32),
    inv_dimensions: (u8, u8),
}

pub const ITEM_SWORD: ItemBlueprint = ItemBlueprint {
    name: Cow::Borrowed("Sword"),
    image_path: Cow::Borrowed("assets/ui/items/sword.png"),
    image_dimensions: (23.0, 68.0),
    inv_dimensions: (1, 3),
};

//enums
enum ItemType {
    Weapon,
    Offhand,
    Armor,
    MiscEquip,
    Usable,
    Other
}