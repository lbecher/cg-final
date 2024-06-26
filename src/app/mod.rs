mod central_panel;
mod menu_bar;
mod side_panel;

use eframe::{App as EframeApp, Frame};
use eframe::egui::{CentralPanel, ColorImage, Context, Pos2, SidePanel, TopBottomPanel};

use crate::app::menu_bar::menu_bar_content;
use crate::app::side_panel::side_panel_content;
use crate::app::central_panel::central_panel_content;
use crate::camera::Camera;
use crate::lighting::Lighting;
use crate::object::Object;
use crate::render::Render;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    Front,
    Side,
    Top,
    Perspective,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderType {
    Wireframe,
    Constant,
    Gouraud,
    Phong,
}

#[derive(Default)]
pub struct TextEditStrings {
    pub camera_vrp_x: String,
    pub camera_vrp_y: String,
    pub camera_vrp_z: String,
    pub camera_p_x: String,
    pub camera_p_y: String,
    pub camera_p_z: String,
    pub camera_dp: String,

    pub lighting_ila_x: String,
    pub lighting_ila_y: String,
    pub lighting_ila_z: String,
    pub lighting_il_x: String,
    pub lighting_il_y: String,
    pub lighting_il_z: String,
    pub lighting_l_x: String,
    pub lighting_l_y: String,
    pub lighting_l_z: String,

    pub object_scale: String,
    pub object_rotate_x: String,
    pub object_rotate_y: String,
    pub object_rotate_z: String,
    pub object_translate_x: String,
    pub object_translate_y: String,
    pub object_translate_z: String,
    pub object_material_ka_r: String,
    pub object_material_ka_g: String,
    pub object_material_ka_b: String,
    pub object_material_kd_r: String,
    pub object_material_kd_g: String,
    pub object_material_kd_b: String,
    pub object_material_ks_r: String,
    pub object_material_ks_g: String,
    pub object_material_ks_b: String,
    pub object_material_n: String,
}

pub struct App {
    objects: Vec<Object>,

    camera: Camera,
    lighting: Lighting,
    
    view: View,
    image: ColorImage,
    redraw: bool,
    render: Option<Render>,

    render_type: RenderType,
    selected_object: Option<usize>,

    control_points: Vec<Pos2>,
    text_edit_strings: TextEditStrings,
}

impl Default for App {
    fn default() -> Self {
        let mut objects = Vec::new();
        objects.push(Object::default());

        let camera = Camera::default();
        let lighting = Lighting::default();

        let image = ColorImage::example();
        let redraw = true;
        let render = None;

        let view = View::Perspective;
        let render_type = RenderType::Constant;
        let selected_object = Some(0);
        
        let control_points = Vec::new();

        let mut text_edit_strings = TextEditStrings::default();

        text_edit_strings.camera_vrp_x = camera.vrp.x.to_string();
        text_edit_strings.camera_vrp_y = camera.vrp.y.to_string();
        text_edit_strings.camera_vrp_z = camera.vrp.z.to_string();
        text_edit_strings.camera_p_x = camera.p.x.to_string();
        text_edit_strings.camera_p_y = camera.p.y.to_string();
        text_edit_strings.camera_p_z = camera.p.z.to_string();
        text_edit_strings.camera_dp = camera.dp.to_string();

        text_edit_strings.lighting_ila_x = lighting.ila.x.to_string();
        text_edit_strings.lighting_ila_y = lighting.ila.y.to_string();
        text_edit_strings.lighting_ila_z = lighting.ila.z.to_string();
        text_edit_strings.lighting_il_x = lighting.il.x.to_string();
        text_edit_strings.lighting_il_y = lighting.il.y.to_string();
        text_edit_strings.lighting_il_z = lighting.il.z.to_string();
        text_edit_strings.lighting_l_x = lighting.l.x.to_string();
        text_edit_strings.lighting_l_y = lighting.l.y.to_string();
        text_edit_strings.lighting_l_z = lighting.l.z.to_string();

        Self {
            objects,

            camera,
            lighting,

            view,
            image,
            redraw,
            render,

            render_type,
            selected_object,

            control_points,
            text_edit_strings,
        }
    }
}

impl EframeApp for App {
    fn update(
        &mut self,
        ctx: &Context,
        _frame: &mut Frame,
    ) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            menu_bar_content(self, ui);
        });
        
        SidePanel::right("side_panel").exact_width(280.0).show(ctx,  |ui| {
            side_panel_content(self, ui);
        });

        CentralPanel::default().show(ctx, |ui| {
            central_panel_content(self, ui);
        });
    }
}