mod central_panel;
mod menu_bar;
mod side_panel;

use eframe::egui;

use crate::app::{
    menu_bar::menu_bar_content,
    side_panel::side_panel_content,
    central_panel::central_panel_content,
};

pub struct App {

}

impl Default for App {
    fn default() -> Self {
        Self {
            
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
            side_panel_content(self, ui);
        });
    }
}