use lore_render::*;
use lore_render::asset_loading::{
    images::load_png,
};

pub struct GUI {
    pub gui_pl: usize,
    pub buttons: Vec<Button>,
}

impl GUI {
    pub fn create(rendering_instance: &mut RenderingInstance, gui_pl: usize) -> Self {
        create_hud_background(rendering_instance, gui_pl);

        let mut up_arrow_btn = Button {
            x: 420.0,
            y: 355.0,
            width: 28.0,
            height: 28.0,
            img_path: "assets/ui/frame/arrow_up.png".into(),
            textures_in_file: 2,
            index: 0,
            name: "up_arrow".to_string(),
            ..Default::default()
        };
        up_arrow_btn.create(rendering_instance, gui_pl);

        let mut right_arrow_btn = Button {
            x: 459.0,
            y: 388.0,
            width: 26.0,
            height: 30.0,
            img_path: "assets/ui/frame/arrow_right.png".into(),
            textures_in_file: 2,
            index: 0,
            name: "right_arrow".to_string(),
            ..Default::default()
        };
        right_arrow_btn.create(rendering_instance, gui_pl);

        GUI {
            buttons: vec!(
                up_arrow_btn,
                right_arrow_btn,
            ),
            gui_pl,
        }
    }

    pub fn check_collision(&self, x: f32, y: f32) -> Option<&Button> {
        for btn in &self.buttons {
            if btn.active && x < btn.x + btn.width && x > btn.x && y < btn.y + btn.height && y > btn.y {
                return Some(btn);
            }
        }
        None
    }
}

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32, // pixels
    pub height: f32, // pixels
    pub instance: (usize, usize), // this is garbage data sometimes, don't forget to call Button::create()
    pub img_path: String,
    pub textures_in_file: u8,
    pub index: u8,
    pub name: String,
    pub active: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 20.0,
            height: 20.0,
            instance: (0, 0),
            img_path: "".into(),
            textures_in_file: 1,
            index: 0,
            name: "unnamed_button".into(),
            active: false,
        }
    }
}

impl Button {
    pub fn create(&mut self,rendering_instance: &mut RenderingInstance, gui_pl: usize) {
        let texture = rendering_instance.create_texture(load_png(&self.img_path));
        let fl_width = self.width / crate::WIDTH * 2.0;
        let fl_height = self.height / crate::HEIGHT * 2.0;
        let fl_x = (self.x / crate::WIDTH) * 2.0 - 1.0;
        let fl_y = (self.y / crate::HEIGHT) * -2.0 + 1.0;
        
        // numbers for changing texture coords for multi-texture image files
        let tx_coord_l = (1.0f32 / self.textures_in_file as f32) * self.index as f32;
        let tx_coord_shift = 1.0f32 / self.textures_in_file as f32;
        let tx_coord_r = tx_coord_l + tx_coord_shift;

        let mesh = rendering_instance.bind_mesh(&lore_render::Mesh {
            vertices: vec!(
                lore_render::Vertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, -1.0],
                    tex_coords: [tx_coord_l, 1.0],
                },
                lore_render::Vertex {
                    position: [fl_width, 0.0, 0.0],
                    normal: [0.0, 0.0, -1.0],
                    tex_coords: [tx_coord_r, 1.0],
                },
                lore_render::Vertex {
                    position: [0.0, fl_height, 0.0],
                    normal: [0.0, 0.0, -1.0],
                    tex_coords: [tx_coord_l, 0.0],
                },
                lore_render::Vertex {
                    position: [fl_width, fl_height, 0.0],
                    normal: [0.0, 0.0, -1.0],
                    tex_coords: [tx_coord_r, 0.0],
                },
            ),
            indices: vec!(2, 1, 3, 0, 1, 2),
        }, gui_pl, Some(texture));
        let instance = rendering_instance.create_object_instance(mesh, ObjectInstance::from_position(fl_x, fl_y - fl_height, 0.0));
        self.instance = instance;
        self.active = true;
    }

    pub fn destroy(&mut self, rendering_instance: &mut RenderingInstance) {
        rendering_instance.remove_object_instance(self.instance);
        self.active = false;
    }
}

pub fn create_hud_background(rendering_instance: &mut RenderingInstance, gui_pl: usize) -> (usize, usize) {
    let hud = rendering_instance.create_texture(load_png("assets/ui/frame/frame.png"));
    let hud_mesh = rendering_instance.bind_mesh(&lore_render::Mesh {
        vertices: vec!(
            lore_render::Vertex {
                position: [-1.0, -1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [0.0, 1.0],
            },
            lore_render::Vertex {
                position: [1.0, -1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [1.0, 1.0],
            },
            lore_render::Vertex {
                position: [-1.0, 1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [0.0, 0.0],
            },
            lore_render::Vertex {
                position: [1.0, 1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [1.0, 0.0],
            },
        ),
        indices: vec!(2, 1, 3, 0, 1, 2),
    }, gui_pl, Some(hud));
    let hud_instance = rendering_instance.create_object_instance(
        hud_mesh,
        ObjectInstance::from_position(0.0, 0.0, 1.0),
    );

    hud_instance
}