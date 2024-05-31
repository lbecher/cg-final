use eframe::egui::{TextEdit, Ui};

use crate::app::{App, View, RenderType};
use crate::constants::TEXT_EDIT_WIDTH;
use crate::types::Vec3;

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

        ui.horizontal(|ui| {
            ui.label("Vista:");
            if eframe::egui::ComboBox::from_label("view")
                .selected_text(match app.view {
                    View::Front => "Frente",
                    View::Side => "Lado",
                    View::Top => "Topo",
                    View::Perspective => "Perspectiva",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut app.view,
                        View::Front,
                        "Frente",
                    ).clicked() || ui.selectable_value(
                        &mut app.view,
                        View::Side,
                        "Lado",
                    ).clicked() || ui.selectable_value(
                        &mut app.view,
                        View::Top,
                        "Topo",
                    ).clicked() || ui.selectable_value(
                        &mut app.view,
                        View::Perspective,
                        "Perspectiva",
                    ).clicked()
                })
                .inner
                .unwrap_or(false)
            {
                app.redraw = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Renderização:");
            if eframe::egui::ComboBox::from_label("render")
                .selected_text(match app.render_type {
                    RenderType::Wireframe => "Wireframe",
                    RenderType::Constant => "Constante",
                    RenderType::Gouraud => "Gouraud",
                    RenderType::Phong => "Phong",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut app.render_type,
                        RenderType::Wireframe,
                        "Wireframe",
                    ).clicked() || ui.selectable_value(
                        &mut app.render_type,
                        RenderType::Constant,
                        "Constante",
                    ).clicked() || ui.selectable_value(
                        &mut app.render_type,
                        RenderType::Gouraud,
                        "Gouraud",
                    ).clicked() || ui.selectable_value(
                        &mut app.render_type,
                        RenderType::Phong,
                        "Phong",
                    ).clicked()
                })
                .inner
                .unwrap_or(false)
            {
                app.redraw = true;
            }
        });

        if let Some(index) = app.selected_object {
            ui.collapsing("Propriedades do Objeto", |ui| {
                ui.collapsing("Transladar", |ui| {
                    ui.horizontal(|ui|{
                        ui.label("X:");
                        ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_translate_x).desired_width(TEXT_EDIT_WIDTH));
                    });
        
                    ui.horizontal(|ui|{
                        ui.label("Y:");
                        ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_translate_y).desired_width(TEXT_EDIT_WIDTH));
                    });
        
                    ui.horizontal(|ui|{
                        ui.label("Z:");
                        ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_translate_z).desired_width(TEXT_EDIT_WIDTH));
                    });
        
                    ui.horizontal(|ui| {
                        if ui.button("Aplicar").clicked() {
                            app.objects[index].translate(Vec3::new(
                                convert_string_to_f32(&mut app.text_edit_strings.object_translate_x),
                                convert_string_to_f32(&mut app.text_edit_strings.object_translate_y),
                                convert_string_to_f32(&mut app.text_edit_strings.object_translate_z),
                            ));
                            app.redraw = true;
                        }
                    });
                });

                ui.collapsing("Rotacionar", |ui| {
                    ui.horizontal(|ui|{
                        ui.label("X:");
                        ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_rotate_x).desired_width(TEXT_EDIT_WIDTH));
                    });

                    ui.horizontal(|ui|{
                        ui.label("Y:");
                        ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_rotate_y).desired_width(TEXT_EDIT_WIDTH));
                    });

                    ui.horizontal(|ui|{
                        ui.label("Z:");
                        ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_rotate_z).desired_width(TEXT_EDIT_WIDTH));
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Aplicar").clicked() {
                            app.objects[index].rotate_x(convert_string_to_f32(&mut app.text_edit_strings.object_rotate_x));
                            app.objects[index].rotate_y(convert_string_to_f32(&mut app.text_edit_strings.object_rotate_y));
                            app.objects[index].rotate_z(convert_string_to_f32(&mut app.text_edit_strings.object_rotate_z));
                            app.redraw = true;
                        }
                    });
                });

                ui.collapsing("Escalar", |ui| {
                    ui.horizontal(|ui|{
                        ui.label("Fator:");
                        ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_scale).desired_width(TEXT_EDIT_WIDTH));
                    });
        
                    ui.horizontal(|ui| {
                        if ui.button("Aplicar").clicked() {
                            app.objects[index].scale(convert_string_to_f32(&mut app.text_edit_strings.object_scale));
                            app.redraw = true;
                        }
                    });
                });

                ui.collapsing("Material", |ui| {
                    ui.collapsing("Ka", |ui| {
                        ui.horizontal(|ui|{
                            ui.label("R:");
                            ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_material_ka_r).desired_width(TEXT_EDIT_WIDTH));
                        });
        
                        ui.horizontal(|ui|{
                            ui.label("G:");
                            ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_material_ka_g).desired_width(TEXT_EDIT_WIDTH));
                        });
        
                        ui.horizontal(|ui|{
                            ui.label("B:");
                            ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_material_ka_b).desired_width(TEXT_EDIT_WIDTH));
                        });
        
                        ui.horizontal(|ui| {
                            if ui.button("Aplicar").clicked() {
                                app.objects[index].material.ka.x = convert_string_to_f32(&mut app.text_edit_strings.object_material_ka_r);
                                app.objects[index].material.ka.y = convert_string_to_f32(&mut app.text_edit_strings.object_material_ka_g);
                                app.objects[index].material.ka.z = convert_string_to_f32(&mut app.text_edit_strings.object_material_ka_b);
                                app.redraw = true;
                            }
                        });
                    });

                    ui.collapsing("Kd", |ui| {
                        ui.horizontal(|ui|{
                            ui.label("R:");
                            ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_material_kd_r).desired_width(TEXT_EDIT_WIDTH));
                        });
        
                        ui.horizontal(|ui|{
                            ui.label("G:");
                            ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_material_kd_g).desired_width(TEXT_EDIT_WIDTH));
                        });
        
                        ui.horizontal(|ui|{
                            ui.label("B:");
                            ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_material_kd_b).desired_width(TEXT_EDIT_WIDTH));
                        });
        
                        ui.horizontal(|ui| {
                            if ui.button("Aplicar").clicked() {
                                app.objects[index].material.kd.x = convert_string_to_f32(&mut app.text_edit_strings.object_material_kd_r);
                                app.objects[index].material.kd.y = convert_string_to_f32(&mut app.text_edit_strings.object_material_kd_g);
                                app.objects[index].material.kd.z = convert_string_to_f32(&mut app.text_edit_strings.object_material_kd_b);
                                app.redraw = true;
                            }
                        });
                    });

                    ui.collapsing("Ks", |ui| {
                        ui.horizontal(|ui|{
                            ui.label("R:");
                            ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_material_ks_r).desired_width(TEXT_EDIT_WIDTH));
                        });
        
                        ui.horizontal(|ui|{
                            ui.label("G:");
                            ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_material_ks_g).desired_width(TEXT_EDIT_WIDTH));
                        });
        
                        ui.horizontal(|ui|{
                            ui.label("B:");
                            ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_material_ks_b).desired_width(TEXT_EDIT_WIDTH));
                        });
        
                        ui.horizontal(|ui| {
                            if ui.button("Aplicar").clicked() {
                                app.objects[index].material.ks.x = convert_string_to_f32(&mut app.text_edit_strings.object_material_ks_r);
                                app.objects[index].material.ks.y = convert_string_to_f32(&mut app.text_edit_strings.object_material_ks_g);
                                app.objects[index].material.ks.z = convert_string_to_f32(&mut app.text_edit_strings.object_material_ks_b);
                                app.redraw = true;
                            }
                        });
                    });

                    ui.collapsing("n", |ui| {
                        ui.horizontal(|ui|{
                            ui.label("n:");
                            ui.add(TextEdit::singleline(&mut app.text_edit_strings.object_material_n).desired_width(TEXT_EDIT_WIDTH));
                        });
            
                        ui.horizontal(|ui| {
                            if ui.button("Aplicar").clicked() {
                                app.objects[index].material.n = convert_string_to_f32(&mut app.text_edit_strings.object_material_n);
                                app.redraw = true;
                            }
                        });
                    });
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