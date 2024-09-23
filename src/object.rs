use eframe::emath::TSTransform;
use egui::Rect;

pub struct SpaceObject {
    pos: egui::Pos2,
    size: f64,
    kind: ObjectType,
}

pub enum ObjectType {
    Dark,
    Green,
    Light,
    Orange,
}

impl From<u8> for ObjectType {
    fn from(value: u8) -> Self {
        match value {
            0 => ObjectType::Dark,
            1 => ObjectType::Green,
            2 => ObjectType::Light,
            _ => ObjectType::Orange,
        }
    }
}

impl ObjectType {
    pub fn image(&self) -> egui::ImageSource {
        match self {
            ObjectType::Dark => egui::include_image!("./assets/dark.png"),
            ObjectType::Green => egui::include_image!("./assets/green.png"),
            ObjectType::Light => egui::include_image!("./assets/light.png"),
            ObjectType::Orange => egui::include_image!("./assets/orange.png"),
        }
    }
}

impl SpaceObject {
    pub fn new(pos: egui::Pos2, kind: ObjectType, size: f64) -> Self {
        Self { pos, size, kind }
    }

    pub fn ui(&self, ui: &mut egui::Ui, id: egui::Id, transform: TSTransform, rect: Rect) {
        let window_layer = ui.layer_id();
        let id = egui::Area::new(id)
            .fixed_pos(self.pos)
            .order(egui::Order::Middle)
            .constrain(false)
            .show(ui.ctx(), |ui| {
                ui.set_clip_rect(transform.inverse() * rect);
                egui::Frame::default()
                    .rounding(egui::Rounding::same(10.0))
                    .inner_margin(egui::Margin::same(0.5))
                    .stroke(egui::Stroke::new(0.5, egui::Color32::LIGHT_YELLOW))
                    .fill(ui.style().visuals.panel_fill)
                    .show(ui, |ui| {
                        ui.add(
                            egui::Image::new(self.kind.image())
                                .fit_to_exact_size(egui::Vec2::new(
                                    self.size as f32,
                                    self.size as f32,
                                ))
                                .rounding(egui::Rounding::same(10.0)),
                        );
                    });
            })
            .response
            .layer_id;
        ui.ctx().set_transform_layer(id, transform);
        ui.ctx().set_sublayer(window_layer, id);
    }
}
