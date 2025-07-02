
mod ui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_drag_and_drop(false)
            .with_decorations(false),
        ..Default::default()
    };

    eframe::run_native(
        "Mod Menu",
        options,
        Box::new(|_cc| Ok(Box::new(ui::KaliTerminalApp::default()))),
    )
}