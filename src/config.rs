use crate::Command;
use egui::Id;

pub struct Config {
    stars: usize,
    arms: usize,
    factor: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            stars: 1000,
            arms: 8,
            factor: 0.15,
        }
    }
}

impl Config {
    pub fn ui(&mut self, ui: &mut egui::Ui, command: &mut Option<Command>) {
        egui::Window::new("Config").show(ui.ctx(), |ui| {
            ui.label("Перемещать камеру можно, удерживая клик на мыши. \
            Что бы использовать zoom необходимо зажать CTRL и использовать колесо (без CTRL работает как scroll).");
            egui::Grid::new(Id::new("conf_grid")).show(ui, |ui| {
                ui.label("Stars count: ");
                ui.add(egui::Slider::new(&mut self.stars, 1000..=10000));
                ui.end_row();

                ui.label("Arms count: ");
                ui.add(egui::Slider::new(&mut self.arms, 1..=30));
                ui.end_row();

                ui.label("Spiral factor: ");
                ui.add(egui::Slider::new(&mut self.factor, 0.1..=0.9));
                ui.end_row();
            });
            if ui.button("Create").clicked() {
                *command = Some(Command::NewGeneration(self.stars, self.arms, self.factor));
            }
        });
    }
}
