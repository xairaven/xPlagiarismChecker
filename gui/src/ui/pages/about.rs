use crate::PROJECT_TITLE;
use crate::context::Context;
use crate::localization::LocalizedLabel;
use crate::ui::pages::Page;
use crate::ui::styles;
use egui::RichText;
use rust_i18n_derive::Localized;

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

        let theme = ctx.settings.theme.get_converted();

        ui.add_space(ui.ctx().content_rect().height() / 5.0);
        ui.vertical_centered_justified(|ui| {
            ui.add(egui::Label::new(
                RichText::new(format!("{} v{}", PROJECT_TITLE, self.version))
                    .size(styles::heading::XLARGE)
                    .color(theme.fg_success_text_color_visuals()),
            ));
            ui.label(LocalizedLabel::PageAboutDescription);

            ui.add_space(20.0);

            ui.label(LocalizedLabel::PageAboutDeveloper);

            ui.add_space(20.0);

            let gh_repo_hyperlink = egui::Hyperlink::from_label_and_url(
                RichText::new(LocalizedLabel::PageAboutCheckGithub.localize())
                    .color(styles::colors::GREEN)
                    .underline(),
                GITHUB_REPO_LINK,
            );
            ui.add(gh_repo_hyperlink);

            let gh_releases_hyperlink = egui::Hyperlink::from_label_and_url(
                RichText::new(format!(
                    "*{}*",
                    LocalizedLabel::PageAboutLatestRelease.localize()
                ))
                .color(styles::colors::GREEN)
                .underline(),
                GITHUB_RELEASES_LINK,
            );
            ui.add(gh_releases_hyperlink);
        });
    }

    fn page_header(&self, ui: &mut egui::Ui) {
        ui.add_space(styles::space::PAGE_HEADER);
    }
}
