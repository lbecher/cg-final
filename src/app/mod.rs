mod central_panel;
mod menu_bar;
mod side_panel;

use eframe::{App as EframeApp, Frame};
use eframe::egui::{CentralPanel, Color32, Context, Pos2, SidePanel, Stroke, TopBottomPanel};

use crate::app::{
    menu_bar::menu_bar_content,
    side_panel::side_panel_content,
    central_panel::central_panel_content,
};

pub struct App {
    selected_object: Option<usize>,

    control_points: Vec<Pos2>,
    stroke: Stroke,
}

impl Default for App {
    fn default() -> Self {
        let selected_object = None;

        let control_points = Vec::new();
        let stroke = Stroke::new(2.0, Color32::GOLD.linear_multiply(0.25));

        Self {
            selected_object,

            control_points,
            stroke,
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