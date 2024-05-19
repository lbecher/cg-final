use eframe::egui;

use crate::app::App;

pub fn side_panel_content(
    app: &mut App,
    ui: &mut egui::Ui,
) {
    ui.collapsing("Iluminação", |ui| {
        
    });

    let some_object_selected = app.selected_object.is_some();
    if some_object_selected {
        ui.collapsing("Propriedades do objeto", |ui| {
            
        });
    }
}