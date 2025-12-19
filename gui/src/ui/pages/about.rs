use crate::PROJECT_TITLE;
use crate::context::Context;
use crate::localization::{Localized, LocalizedLabel};
use crate::ui::pages::{Page, PageId};
use crate::ui::styles;
use egui::RichText;

const GITHUB_REPO_LINK: &str = "https://github.com/xairaven/xPlagiarismChecker";
const GITHUB_RELEASES_LINK: &str =
    "https://github.com/xairaven/xPlagiarismChecker/releases";

#[derive(Debug)]
pub struct AboutPage {
    version: semver::Version,
}

impl Default for AboutPage {
    fn default() -> Self {
        Self {
            version: semver::Version::parse(env!("CARGO_PKG_VERSION")).unwrap_or(
                semver::Version {
                    major: 0,
                    minor: 0,
                    patch: 1,
                    pre: Default::default(),
                    build: Default::default(),
                },
            ),
        }
    }
}

impl Page for AboutPage {
    fn show_content(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        self.page_header(ui);

        let style = &ctx.gui.style;

        ui.add_space(ui.ctx().content_rect().height() / 5.0);
        ui.vertical_centered_justified(|ui| {
            ui.add(egui::Label::new(
                RichText::new(format!("{} v{}", PROJECT_TITLE, self.version))
                    .size(styles::heading::XLARGE)
                    .color(style.theme.fg_success_text_color_visuals()),
            ));
            ui.label(LocalizedLabel::AboutDescription.localize());

            ui.add_space(20.0);

            ui.label(LocalizedLabel::AboutDeveloper.localize());

            ui.add_space(20.0);

            ui.hyperlink_to(
                LocalizedLabel::AboutCheckGithub.localize(),
                GITHUB_REPO_LINK,
            );
            ui.hyperlink_to(
                format!("*{}*", LocalizedLabel::AboutLatestRelease.localize()),
                GITHUB_RELEASES_LINK,
            );
        });
    }

    fn page_header(&self, ui: &mut egui::Ui) {
        ui.add_space(styles::space::PAGE_HEADER);
    }

    fn id(&self) -> PageId {
        PageId::About
    }
}
