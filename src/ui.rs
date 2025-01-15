
/// ui.rs
use crate::engine::GameOfLife;
use eframe::egui;

pub struct UiApp {
    engine: GameOfLife,
    simulation_running: bool,
    simulation_speed: f32,
    random_factor: f64,
}

impl UiApp {
    pub fn new() -> Self {
        let mut engine = GameOfLife::new(128, 128);
        engine.randomize(0.5);
        Self {
            engine,
            simulation_running: false,
            simulation_speed: 10.0,
            random_factor: 0.5,
        }
    }
}

impl eframe::App for UiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Play / Pause").clicked() {
                self.simulation_running = !self.simulation_running;
            }
            ui.add(egui::Slider::new(&mut self.simulation_speed, 0.0..=60.0).text("UPS"));
            ui.add(egui::Slider::new(&mut self.random_factor, 0.0..=1.0).text("Random Factor"));
            if ui.button("Randomize Grid").clicked() {
                self.engine.randomize(self.random_factor);
            }

            if self.simulation_running {
                self.engine.update();
            }
            // Rendering logic will go here in the next phases.
        });
    }
}
