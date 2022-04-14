use lore_render::{
    ObjectInstance,
    InputEvent,
};
use lore_render::{
    cgmath::*,
    RenderingInstance,
    VirtualKeyCode, ElementState,
};

use bimap::{
    BiMap,
};

mod party;
mod utils;
mod battle_scene;
mod battler;

// all gamestate is contained in this struct
struct State {
    // (usize, usize) is the type of an object ID. don't worry about why
    cube: (usize, usize),
}

pub fn main() {
    // initialize the engine
    lore_render::run(
        setup,
        update,
        input,
    );
}

// setup runs once before the first update, and must return a new State
fn setup(rendering_instance: &mut RenderingInstance) -> State {
    // load shaders (rendering stuff, don't worry)
    let pl = rendering_instance.create_default_render_pipeline();

    // load a cube mesh
    let cube_mesh = {
        let mesh_data = lore_render::Mesh::from_gltf("assets/cube.gltf");
        rendering_instance.bind_mesh(&mesh_data, pl, None)
    };

    // create an instance of the mesh we just loaded (many instances of the same mesh can be created)
    let cube = rendering_instance.create_object_instance(
        cube_mesh,
        ObjectInstance::from_position(-0.5, 0.0, 0.0),
    );

    // return the initialized state
    State {
        cube  
    }
}

// the update function is called every frame
fn update(rendering_instance: &mut RenderingInstance, state: &mut State) {

}

// the input function is called whenever the mouse or keyboard does anything
fn input(rendering_instance: &mut RenderingInstance, state: &mut State, input: InputEvent) {
    match input {
        InputEvent::Keyboard(key_input) => {
            // keyboard handling here
            if let Some(keycode) = key_input.virtual_keycode {
                // on space press (note that this event continues to trigger again as long as space is held)
                if keycode == VirtualKeyCode::Space && key_input.state == ElementState::Pressed {
                    // changing the position/rotation of an object is done this way
                    rendering_instance.modify_instance(state.cube, |inst| {
                        inst.rotation = inst.rotation * Quaternion::<f32>::from_axis_angle(Vector3::unit_y(), Deg(5.0));
                    });
                }
            }
        },
        InputEvent::Mouse(button_state, button) => {
            // mouse handling here
        },
    }
}