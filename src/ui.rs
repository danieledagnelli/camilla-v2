
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

            let grid = self.engine.get_display_grid();
            let width = self.engine.width;
            let height = self.engine.height;
            let pixel_size = 5.0; // Each cell will be drawn as a 5x5 pixel square

            ui.horizontal(|ui| {
                let (rect, _response) = ui.allocate_exact_size(
                    egui::Vec2::new(width as f32 * pixel_size, height as f32 * pixel_size),
                    egui::Sense::hover(),
                );

                let painter = ui.painter();
                for row in 0..height {
                    for col in 0..width {
                        let color = if grid[row * width + col] {
                            egui::Color32::WHITE
                        } else {
                            egui::Color32::BLACK
                        };
                        let x = rect.min.x + col as f32 * pixel_size;
                        let y = rect.min.y + row as f32 * pixel_size;
                        painter.rect_filled(
                            egui::Rect::from_min_size(
                                egui::Pos2 { x, y },
                                egui::Vec2::new(pixel_size, pixel_size),
                            ),
                            0.0,
                            color,
                        );
                    }
                }
            });
        });
    }
}