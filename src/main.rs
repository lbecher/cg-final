mod app;

use eframe::egui;

use app::App;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Modelador por Revolução",
        options,
        Box::new(|_cc| {
            Box::<App>::default()
        }),
    )
}