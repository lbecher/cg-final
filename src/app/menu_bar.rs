use eframe::egui::Ui;

use crate::app::App;

pub fn menu_bar_content(
    app: &mut App,
    ui: &mut Ui,
) {
    let style = (*ui.ctx().style()).clone();
    let new_visuals = style.visuals.light_dark_small_toggle_button(ui);
    if let Some(visuals) = new_visuals {
        if let Some(render) = &mut app.render {
            let clear_color = [
                visuals.window_fill().r(),
                visuals.window_fill().b(),
                visuals.window_fill().b(),
            ];
            render.set_clear_color(clear_color);
            app.redraw = true;
        }

        ui.ctx().set_visuals(visuals);
    }
}