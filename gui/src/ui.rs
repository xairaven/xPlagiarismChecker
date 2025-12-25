use crate::PROJECT_TITLE;
use crate::config::Config;
use crate::errors::ProjectError;
use crate::ui::app::App;

pub struct Ui {
    min_width: f32,
    min_height: f32,
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            min_width: 950.0,
            min_height: 550.0,
        }
    }
}

impl Ui {
    pub fn start(self, config: Config) -> Result<(), ProjectError> {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_app_id(PROJECT_TITLE) // Wayland requirement
                .with_title(PROJECT_TITLE)
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
            PROJECT_TITLE,
            native_options,
            Box::new(|cc| Ok(Box::new(App::new(cc, config)))),
        )
        .map_err(ProjectError::EFrame)
    }

    pub fn native_panic_message(error: ProjectError) {
        rfd::MessageDialog::new()
            .set_title("Critical Error")
            .set_description(error.to_string())
            .set_level(rfd::MessageLevel::Error)
            .show();
    }
}

pub mod app;
pub mod channel;
pub mod commands;
pub mod components;
pub mod context;
pub mod modals;
pub mod pages;
pub mod state;
pub mod styles;
pub mod themes;
pub mod widgets;
