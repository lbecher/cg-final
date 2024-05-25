mod central_panel;
mod menu_bar;
mod side_panel;

use eframe::{App as EframeApp, Frame};
use eframe::egui::{CentralPanel, Color32, ColorImage, Context, Pos2, SidePanel, Stroke, TopBottomPanel};

use crate::app::menu_bar::menu_bar_content;
use crate::app::side_panel::side_panel_content;
use crate::app::central_panel::central_panel_content;
use crate::render::Render;

#[derive(Debug, Clone)]
pub enum ParallelViewType {
    Front,
    Side,
    Top,
}

pub struct App {
    image: ColorImage,
    redraw: bool,
    render: Option<Render>,

    perspective_view: bool,
    parallel_view_type: ParallelViewType,
    selected_object: Option<usize>,

    control_points: Vec<Pos2>,
}

impl Default for App {
    fn default() -> Self {
        let image = ColorImage::example();
        let redraw = true;
        let render = None;

        let perspective_view = false;
        let parallel_view_type = ParallelViewType::Front;
        
        let selected_object = None;
        
        let control_points = Vec::new();
        

        Self {
            image,
            redraw,
            render,

            perspective_view,
            parallel_view_type,
            selected_object,

            control_points,
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
        
        SidePanel::right("side_panel").show(ctx,  |ui| {
            side_panel_content(self, ui);
        });

        CentralPanel::default().show(ctx, |ui| {
            central_panel_content(self, ui);
        });
    }
}