use lore_render::*;
use crate::*;

pub fn create_button_handlers() -> HashMap<String, fn(&mut RenderingInstance, &mut State) -> ()> {
    let mut button_handlers: HashMap<String, fn(&mut RenderingInstance, &mut State) -> ()> = HashMap::new();
    button_handlers.insert("up_arrow".into(), on_up_arrow);
    button_handlers
}

fn on_up_arrow(rendering_instance: &mut RenderingInstance, state: &mut State) {
    println!("Up!");
}