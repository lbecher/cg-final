use eframe::egui::{TextEdit, Ui};

use crate::app::App;
use crate::constants::TEXT_EDIT_WIDTH;
use crate::object;

fn convert_string_to_f32(string: &mut String) -> f32 {
    match string.parse::<f32>() {
        Ok(value) => value,
        Err(_) => {
            *string = "Insira um ponto flutuante!".to_string();
            0.0
        },
    }
}

pub fn side_panel_content(
    app: &mut App,
    ui: &mut Ui,
) {
    let width = ui.available_width();
    eframe::egui::ScrollArea::vertical().show(ui, |ui| {
        ui.set_width(width);

        if let Some(index) = app.selected_object {
            ui.collapsing("Propriedades do Objeto", |ui| {
                ui.collapsing("Transladar", |ui| {

                });

                ui.collapsing("Rotacionar", |ui| {
                    
                });

                ui.collapsing("Escalar", |ui| {
                    
                });

                ui.collapsing("Material", |ui| {
                    
                });
            });
        }

        ui.collapsing("Iluminação", |ui| {
            ui.collapsing("L", |ui| {
                ui.horizontal(|ui|{
                    ui.label("X:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.lighting_l_x).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui|{
                    ui.label("Y:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.lighting_l_y).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui|{
                    ui.label("Z:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.lighting_l_z).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui| {
                    if ui.button("Aplicar").clicked() {
                        app.lighting.l.x = convert_string_to_f32(&mut app.text_edit_strings.lighting_l_x);
                        app.lighting.l.y = convert_string_to_f32(&mut app.text_edit_strings.lighting_l_y);
                        app.lighting.l.z = convert_string_to_f32(&mut app.text_edit_strings.lighting_l_z);
                        app.redraw = true;
                    }
                });
            });
    
            ui.collapsing("IL", |ui| {
                ui.horizontal(|ui|{
                    ui.label("X:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.lighting_il_x).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui|{
                    ui.label("Y:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.lighting_il_y).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui|{
                    ui.label("Z:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.lighting_il_z).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui| {
                    if ui.button("Aplicar").clicked() {
                        app.lighting.il.x = convert_string_to_f32(&mut app.text_edit_strings.lighting_il_x);
                        app.lighting.il.y = convert_string_to_f32(&mut app.text_edit_strings.lighting_il_y);
                        app.lighting.il.z = convert_string_to_f32(&mut app.text_edit_strings.lighting_il_z);
                        app.redraw = true;
                    }
                });
            });
    
            ui.collapsing("ILA", |ui| {
                ui.horizontal(|ui|{
                    ui.label("X:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.lighting_ila_x).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui|{
                    ui.label("Y:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.lighting_ila_y).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui|{
                    ui.label("Z:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.lighting_ila_z).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui| {
                    if ui.button("Aplicar").clicked() {
                        app.lighting.ila.x = convert_string_to_f32(&mut app.text_edit_strings.lighting_ila_x);
                        app.lighting.ila.y = convert_string_to_f32(&mut app.text_edit_strings.lighting_ila_y);
                        app.lighting.ila.z = convert_string_to_f32(&mut app.text_edit_strings.lighting_ila_z);
                        app.redraw = true;
                    }
                });
            });
        });
    
        ui.collapsing("Câmera", |ui| {
            ui.collapsing("VRP", |ui| {
                ui.horizontal(|ui|{
                    ui.label("X:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.camera_vrp_x).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui|{
                    ui.label("Y:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.camera_vrp_y).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui|{
                    ui.label("Z:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.camera_vrp_z).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui| {
                    if ui.button("Aplicar").clicked() {
                        app.camera.vrp.x = convert_string_to_f32(&mut app.text_edit_strings.camera_vrp_x);
                        app.camera.vrp.y = convert_string_to_f32(&mut app.text_edit_strings.camera_vrp_y);
                        app.camera.vrp.z = convert_string_to_f32(&mut app.text_edit_strings.camera_vrp_z);
                        app.redraw = true;
                    }
                });
            });
    
            ui.collapsing("P", |ui| {
                ui.horizontal(|ui|{
                    ui.label("X:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.camera_p_x).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui|{
                    ui.label("Y:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.camera_p_y).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui|{
                    ui.label("Z:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.camera_p_z).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui| {
                    if ui.button("Aplicar").clicked() {
                        app.camera.p.x = convert_string_to_f32(&mut app.text_edit_strings.camera_p_x);
                        app.camera.p.y = convert_string_to_f32(&mut app.text_edit_strings.camera_p_y);
                        app.camera.p.z = convert_string_to_f32(&mut app.text_edit_strings.camera_p_z);
                        app.redraw = true;
                    }
                });
            });
    
            ui.collapsing("DP", |ui| {
                ui.horizontal(|ui|{
                    ui.label("DP:");
                    ui.add(TextEdit::singleline(&mut app.text_edit_strings.camera_dp).desired_width(TEXT_EDIT_WIDTH));
                });
    
                ui.horizontal(|ui| {
                    if ui.button("Aplicar").clicked() {
                        app.camera.dp = convert_string_to_f32(&mut app.text_edit_strings.camera_dp);
                        app.redraw = true;
                    }
                });
            });
        });
    });
}