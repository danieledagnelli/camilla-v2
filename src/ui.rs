/// ui.rs
use crate::engine::GameOfLife;
use eframe::egui;

pub struct UiApp {
    engine: GameOfLife,
    simulation_running: bool,
    simulation_speed: f32,
}

impl UiApp {
    pub fn new() -> Self {
        Self {
            engine: GameOfLife::new(128, 128),
            simulation_running: false,
            simulation_speed: 10.0,
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

            if self.simulation_running {
                self.engine.update();
            }
            // Rendering logic will go here in the next phases.
        });
    }
}
