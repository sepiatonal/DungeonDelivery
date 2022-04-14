mod map;
mod gui;
mod buttons;
mod inventory;

use lore_render::{
    ObjectInstance,
    InputEvent,
    cgmath::*,
    RenderingInstance,
    VirtualKeyCode, ElementState, KeyboardInput, VirtualKeyCode::*, MouseButton,
    asset_loading::images::load_png,
    TextInstance,
};
use map::*;
use gui::*;
use inventory::*;
use buttons::*;
use crate::battler::*;
use crate::party::*;
use std::collections::HashMap;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 480.0;

use bimap::{
    BiMap,
};

mod party;
mod utils;
mod battle_scene;
mod battler;

// all gamestate is contained in this struct
pub struct State {
    map: Map,
    gui: GUI,
    button_handlers: HashMap<String, fn(&mut RenderingInstance, &mut State) -> ()>,
    player_location: Vector2<u8>,
    player_facing: Direction,
    mouse_x: f64,
    mouse_y: f64,
    font_brush: usize,
}

impl State {
    pub fn current_inventory(&self) -> &Inventory {
        todo!();
    }
}

pub fn main() {
    lore_render::run(
        setup,
        update,
        input,
    );
}

// setup runs once before the first update, and must return a new State
fn setup(rendering_instance: &mut RenderingInstance) -> State {
    // load shaders
    let pl = rendering_instance.create_default_render_pipeline();
    let gui_pl = rendering_instance.create_default_gui_render_pipeline();
    let gui = GUI::create(rendering_instance, gui_pl);
    let font_brush = rendering_instance.create_glyph_brush("assets/ui/Dico.ttf");

    let mut inv_example = Inventory::new();
    let text_example = rendering_instance.create_text_box(TextInstance {
        position: (229.0, 259.0),
        dimensions: (430.0, 90.0),
        brush: font_brush,
        color: [0.0, 0.0, 0.0, 1.0],
        scale: 20.0,
        text: "This is some example text".into(),
    });

    // initialize the player party
    let paris = &mut PlayerUnit {
        name: "Paris".to_string(),
        base_params: BattleBaseParameters::new(),
        battle: Battler::new(),
    };

    let dejiko = &mut PlayerUnit {
        name: "Dejiko".to_string(),
        base_params: BattleBaseParameters::new(),
        battle: Battler::new(),
    };

    let john_wick = &mut PlayerUnit {
        name: "John Wick desu nya!".to_string(),
        base_params: BattleBaseParameters::new(),
        battle: Battler::new(),
    };

    paris.base_params = BattleBaseParameters {max_hp: 8, attack: 3, defense: 2, strength: 6, speed: 9, armor: 0.00};
    //paris.refresh_params();

    dejiko.base_params = BattleBaseParameters {max_hp: 4, attack: 2, defense: 0, strength: 3, speed: 15, armor: 0.00};
    //dejiko.refresh_params();

    john_wick.base_params = BattleBaseParameters {max_hp: 6, attack: 4, defense: 1, strength: 6, speed: 11, armor: 0.00};
    //john_wick.refresh_params();

    // return the initialized state
    let mut st = State {
        map: Map::new(),
        gui,
        button_handlers: create_button_handlers(),
        player_location: (0, 0).into(),
        player_facing: Direction::NORTH,
        mouse_x: 0.0,
        mouse_y: 0.0,
        font_brush,
    };

    inv_example.show(&mut st, rendering_instance);
    inv_example.add_item_anywhere(&mut st, rendering_instance, Item::from_blueprint(&inventory::ITEM_SWORD)).ok();

    st
}

// the update function is called every frame
fn update(rendering_instance: &mut RenderingInstance, state: &mut State) {
    let cam_x = map::TILE_SIZE * (state.player_location.x as f32) + map::TILE_SIZE / 2.0;
    let cam_y = 0.0;
    let cam_z = map::TILE_SIZE * (state.player_location.y as f32) + map::TILE_SIZE / 2.0;
    let cam_pos = (cam_x, cam_y, cam_z).into();
    let cam_target = cam_pos + match state.player_facing {
        Direction::NORTH => Vector3::<f32>::unit_z(),
        Direction::SOUTH => -Vector3::<f32>::unit_z(),
        Direction::EAST => Vector3::<f32>::unit_x(),
        Direction::WEST => -Vector3::<f32>::unit_x(),
    };
    rendering_instance.set_camera_transform(Some(cam_pos), Some(cam_target), None);
}

// the input function is called whenever the mouse or keyboard does anything
fn input(rendering_instance: &mut RenderingInstance, state: &mut State, input: InputEvent) {
    match input {
        InputEvent::Keyboard(key_input) => {
            if let Some(keycode) = key_input.virtual_keycode {
                // key pressed
                if key_input.state == ElementState::Pressed {
                    match keycode {
                        W | A | S | D => {
                                process_movement_input(state, keycode);
                        },
                        _ => {},
                    }
                }
            }
        },
        InputEvent::Mouse(button_state, button) => {
            process_mouse_input(rendering_instance, state, button_state, button);
        },
        InputEvent::MouseLocation(x, y) => {
            state.mouse_x = x;
            state.mouse_y = y;
        }
    }
}

fn process_mouse_input(rendering_instance: &mut RenderingInstance, state: &mut State, button_state: &ElementState, button: &MouseButton) -> Option<()> {
    match button_state {
        ElementState::Pressed => {
            let btn_name: String = {
                let btn_mayb = state.gui.check_collision(state.mouse_x as f32, state.mouse_y as f32);
                if let Some(btn) = btn_mayb {
                    btn.name.clone()
                } else {
                    return None
                }
            };
            state.button_handlers.get(&btn_name)?(rendering_instance, state);
            None
        },
        ElementState::Released => {
            None
        }
    }
}

fn process_movement_input(state: &mut State, key: VirtualKeyCode) {
    match key {
        W => move_player(state, state.player_facing),
        A => state.player_facing = state.player_facing.counterclockwise(),
        D => state.player_facing = state.player_facing.clockwise(),
        S => move_player(state, state.player_facing.opposite()),
        _ => { panic!("unreachable") }
    };
}

fn move_player(state: &mut State, direction: Direction) {
    state.player_location = map_location_shifted(state.player_location, direction);
}

fn map_location_shifted(pos: Vector2<u8>, dir: Direction) -> Vector2<u8> {
    match dir {
        Direction::WEST => {
            if pos.x > 0 {
                pos - Vector2::<u8>::new(1, 0)
            } else { pos }
        },
        Direction::EAST => {
            if pos.x < u8::MAX {
                pos + Vector2::<u8>::new(1, 0)
            } else { pos }
        },
        Direction::NORTH => {
            if pos.y < u8::MAX {
                pos + Vector2::<u8>::new(0, 1)
            } else { pos }
        },
        Direction::SOUTH => {
            if pos.y > 0 {
                pos - Vector2::<u8>::new(0, 1)
            } else { pos }
        },
    }
}