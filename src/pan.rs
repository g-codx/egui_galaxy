use crate::generation::Galaxy;
use crate::object::{ObjectType, SpaceObject};
use crate::Command;
use egui::emath::TSTransform;

#[derive(Clone, PartialEq, Default)]
pub struct PanZoom {
    transform: TSTransform,
    galaxy: Galaxy,
}

impl Eq for PanZoom {}

impl PanZoom {
    pub fn ui(&mut self, ui: &mut egui::Ui, command: &mut Option<Command>) {
        let (id, rect) = ui.allocate_space(ui.available_size());
        let response = ui.interact(rect, id, egui::Sense::click_and_drag());

        if response.dragged() {
            self.transform.translation += response.drag_delta();
        }

        if response.double_clicked() {
            self.transform = TSTransform::default();
        }

        let transform =
            TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * self.transform;

        if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
            if response.hovered() {
                let pointer_in_layer = transform.inverse() * pointer;
                let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
                let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta);

                self.transform = self.transform
                    * TSTransform::from_translation(pointer_in_layer.to_vec2())
                    * TSTransform::from_scaling(zoom_delta)
                    * TSTransform::from_translation(-pointer_in_layer.to_vec2());

                self.transform = TSTransform::from_translation(pan_delta) * self.transform;
            }
        }

        if let Some(cmd) = command.take() {
            match cmd {
                Command::NewGeneration(stars, arms, factor) => {
                    self.galaxy = Galaxy::new(stars, arms, factor);
                }
            }
        }

        for (i, star) in self.galaxy.stars.iter().enumerate() {
            SpaceObject::new(star.pos, ObjectType::from(star.star_type), star.size).ui(
                ui,
                id.with(("object", i)),
                transform,
                rect,
            );
        }
    }
}
