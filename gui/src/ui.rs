use crate::config::Config;
use crate::ui::app::App;

pub struct Ui {
    min_width: f32,
    min_height: f32,
    title: String,
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            min_width: 950.0,
            min_height: 550.0,
            title: String::from("xPlagiarismChecker"),
        }
    }
}

impl Ui {
    pub fn start(self, config: Config) -> eframe::Result {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_title(&self.title)
                .with_inner_size([self.min_width, self.min_height])
                .with_min_inner_size([self.min_width, self.min_height])
                .with_icon(
                    eframe::icon_data::from_png_bytes(
                        &include_bytes!("../assets/icon.png")[..],
                    )
                    .unwrap_or_else(|err| {
                        log::error!("Failed to load app icon. {err}");
                        std::process::exit(1);
                    }),
                ),
            centered: true,
            ..Default::default()
        };

        eframe::run_native(
            &self.title,
            native_options,
            Box::new(|cc| Ok(Box::new(App::new(cc, config)))),
        )
    }
}

pub mod app;
pub mod modals;
pub mod themes;
