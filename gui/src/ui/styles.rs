pub mod colors {
    use egui::Color32;

    pub const RED: Color32 = Color32::from_rgb(255, 0, 0);
    pub const GREEN: Color32 = Color32::from_rgb(0, 255, 0);
}

pub mod heading {
    pub const HUGE: f32 = 20.0;
    pub const XLARGE: f32 = 32.0;

    pub fn huge(title: &str) -> egui::RichText {
        egui::RichText::new(title).size(HUGE).strong()
    }
}

pub mod space {
    pub const SMALL: f32 = 10.0;
    pub const PAGE_HEADER: f32 = 13.0;
    pub const NAVIGATOR_HEADER: f32 = 32.0;
}
