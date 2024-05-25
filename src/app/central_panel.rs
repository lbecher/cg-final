use eframe::egui::{
    pos2,
    Color32, Pos2, Rect, Sense, Shape, Stroke, TextureId, TextureOptions, Ui, Vec2
};
use eframe::egui::emath::RectTransform;
use eframe::egui::epaint::PathShape;

use crate::app::App;
use crate::render::Render;

pub fn central_panel_content(
    app: &mut App,
    ui: &mut Ui,
) {

    let size = Vec2::new(ui.available_width(), ui.available_height());
    let (response, painter) = ui.allocate_painter(size, Sense::hover());

    if app.redraw {
        if app.render.is_none() {
            let width = response.rect.width() as u32;
            let height = response.rect.height() as u32;
            let clear_color = [
                ui.visuals().window_fill().r(),
                ui.visuals().window_fill().b(),
                ui.visuals().window_fill().b(),
            ];
            app.render = Some(Render::new(width, height, clear_color));
        }
        
        if let Some(render) = &mut app.render {
            app.image = render.redraw();
            app.redraw = false;
        }
    }

    let texture = ui.ctx().load_texture("render", app.image.clone(), TextureOptions::default());
    let texture_id = TextureId::from(&texture);
    let uv = Rect {
        min: pos2(0.0, 0.0),
        max: pos2(1.0, 1.0),
    };

    painter.image(texture_id, response.rect, uv, Color32::WHITE);

    if !app.perspective_view {
        let to_screen = RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );

        // adiciona ponto caso haja um clique do mouse
        let click_response = ui.interact(response.rect, response.id, Sense::click());
        if click_response.clicked() {
            if let Some(position_on_window) = click_response.hover_pos() {
                let painter_origin = painter.clip_rect().min;
                let position_on_painter = position_on_window - painter_origin;
                app.control_points.push(Pos2::new(position_on_painter.x, position_on_painter.y));
            }
        }
        
        // cria os pontos de controle para alterar o traço
        let control_point_radius = 8.0;
        let control_point_shapes: Vec<Shape> = app.control_points
            .iter_mut()
            .enumerate()
            .map(|(i, point)| {
                let size = Vec2::splat(2.0 * control_point_radius);

                let point_in_screen = to_screen.transform_pos(*point);
                let point_rect = Rect::from_center_size(point_in_screen, size);
                let point_id = response.id.with(i);
                let point_response = ui.interact(point_rect, point_id, Sense::drag());

                *point += point_response.drag_delta();
                *point = to_screen.from().clamp(*point);

                let point_in_screen = to_screen.transform_pos(*point);
                let control_points_stroke = Stroke::new(2.0, ui.ctx().style().visuals.text_color());

                Shape::circle_stroke(point_in_screen, control_point_radius, control_points_stroke)
            })
            .collect();

        let mut points_in_screen: Vec<Pos2> = app.control_points
            .iter()
            .map(|point| to_screen * *point)
            .collect();

        if points_in_screen.len() > 2 {
            let first = points_in_screen[0];
            points_in_screen.push(first);
        }

        let stroke = Stroke::new(2.0, Color32::GOLD);
        painter.add(PathShape::line(points_in_screen, stroke));
        painter.extend(control_point_shapes);
    }
}