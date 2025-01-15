/// main.rs
mod engine;
mod ui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Game of Life",
        native_options,
        Box::new(|_cc| Ok(Box::new(ui::UiApp::new())))
    );
}
