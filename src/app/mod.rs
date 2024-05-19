mod central_panel;
mod menu_bar;
mod side_panel;

use eframe::egui;
use eframe::egui::epaint;

use crate::app::{
    menu_bar::menu_bar_content,
    side_panel::side_panel_content,
    central_panel::central_panel_content,
};

pub struct App {
    selected_object: Option<usize>,

    control_points: Vec<epaint::Pos2>,

    /// Bézier curve degree, it can be 3, 4.
    degree: usize,

    /// Stroke for Bézier curve.
    stroke: epaint::Stroke,

    /// Fill for Bézier curve.
    fill: epaint::Color32,

    /// Stroke for auxiliary lines.
    aux_stroke: epaint::Stroke,

    bounding_box_stroke: epaint::Stroke,
}

impl Default for App {
    fn default() -> Self {
        let selected_object = None;

        let control_points = [
            epaint::pos2(50.0, 50.0),
            epaint::pos2(60.0, 250.0),
            epaint::pos2(200.0, 200.0),
            epaint::pos2(250.0, 50.0),
        ].to_vec();

        Self {
            selected_object,

            control_points,
            degree: 4,
            stroke: epaint::Stroke::new(1.0, epaint::Color32::from_rgb(25, 200, 100)),
            fill: epaint::Color32::from_rgb(50, 100, 150).linear_multiply(0.25),
            aux_stroke: epaint::Stroke::new(1.0, epaint::Color32::RED.linear_multiply(0.25)),
            bounding_box_stroke: epaint::Stroke::new(0.0, epaint::Color32::LIGHT_GREEN.linear_multiply(0.25)),
        }
    }
}

impl eframe::App for App {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            menu_bar_content(self, ui);
        });
        
        egui::SidePanel::right("side_panel").show(ctx,  |ui| {
            side_panel_content(self, ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            central_panel_content(self, ui);
        });
    }
}