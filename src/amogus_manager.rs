use egui;

pub const MIN_AMOGUS_SIZE: f32 = 10.0;
pub const MAX_AMOGUS_SIZE: f32 = 200.0;
pub const MIN_AMOGUS_TWERK: f32 = 1.0;
pub const MAX_AMOGUS_TWERK: f32 = 10.0;

pub struct AmogusManager {
    pub random_color: bool,
    pub color: [f32; 3],
    pub random_size: bool,
    pub size: f32,
    pub random_twerk_speed: bool,
    pub twerk_speed: f32,
    pub should_kill_all: bool,
}

impl AmogusManager {
    pub fn new() -> Self {
        Self {
            random_color: true,
            color: [1.0, 0.0, 0.0],
            random_size: true,
            size: 100.0,
            random_twerk_speed: true,
            twerk_speed: 5.0,
            should_kill_all: false,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("parameters for new amogus");

        ui.label("amogus color");
        ui.checkbox(&mut self.random_color, "random color");
        if !self.random_color {
            egui::widgets::color_picker::color_edit_button_rgb(ui, &mut self.color);
        }

        ui.label("amogus size");
        ui.checkbox(&mut self.random_size, "random size");
        if !self.random_size {
            ui.add(egui::Slider::new(
                &mut self.size,
                MIN_AMOGUS_SIZE..=MAX_AMOGUS_SIZE,
            ));
        }

        ui.label("amogus twerk speed");
        ui.checkbox(&mut self.random_twerk_speed, "random twerk speed");
        if !self.random_twerk_speed {
            ui.add(egui::Slider::new(
                &mut self.twerk_speed,
                MIN_AMOGUS_TWERK..=MAX_AMOGUS_TWERK,
            ));
        }

        ui.colored_label(
            egui::Color32::from_rgb(255, 0, 0),
            "drag to create new amogus",
        );

        ui.heading("amogus removal");
        if ui.button("KILL ALL").clicked() {
            self.should_kill_all = true;
        }
    }
}
