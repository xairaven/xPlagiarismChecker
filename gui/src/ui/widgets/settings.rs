use crate::context::Context;
use crate::localization::{Localized, LocalizedLabel};
use crate::ui::commands::UiCommand;
use crate::ui::styles;
use egui::{Button, DragValue, RichText};

pub trait SettingWidget<V>
where
    V: PartialEq + Clone,
{
    fn with_label(mut self, label: &str) -> Self
    where
        Self: Sized,
    {
        self.common_mut().label = label.to_string();
        self
    }

    fn takes_effect_after_restart(mut self) -> Self
    where
        Self: Sized,
    {
        self.common_mut().takes_effect_after_restart = true;
        self
    }

    fn send_command_on_save(mut self, closure: impl Fn(&V) -> UiCommand + 'static) -> Self
    where
        Self: Sized,
    {
        let closure = OnSaveClosure::new(Box::new(closure));
        self.common_mut().commands_on_save.push(closure);
        self
    }

    fn render_label(&self, ui: &mut egui::Ui) {
        self.common().render_label(ui);
    }

    fn common(&self) -> &SettingsCommon<V>;
    fn common_mut(&mut self) -> &mut SettingsCommon<V>;
}

pub struct OnSaveClosure<V>(Box<dyn Fn(&V) -> UiCommand>);

impl<V> OnSaveClosure<V> {
    pub fn new<F>(closure: F) -> Self
    where
        F: 'static + Fn(&V) -> UiCommand,
    {
        Self(Box::new(closure))
    }
}

impl<V> std::fmt::Debug for OnSaveClosure<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OnSaveClosure").finish()
    }
}

#[derive(Debug)]
pub struct SettingsCommon<V> {
    label: String,
    takes_effect_after_restart: bool,
    commands_on_save: Vec<OnSaveClosure<V>>,
    state: SettingState,
}

impl<V> Default for SettingsCommon<V> {
    fn default() -> Self {
        Self {
            label: "".to_string(),
            takes_effect_after_restart: false,
            commands_on_save: vec![],

            state: Default::default(),
        }
    }
}

impl<V> SettingsCommon<V>
where
    V: PartialEq + Clone,
{
    fn update_state(&mut self, current_value: &V, context_value: &V) {
        self.state.check_is_applied(current_value, context_value);
    }

    fn render_label(&self, ui: &mut egui::Ui) {
        let mut text = RichText::new(&self.label).strong();

        if !self.state.is_applied {
            text = text.color(styles::colors::RED);
        }

        let label = ui.label(text);
        if self.takes_effect_after_restart {
            label.on_hover_text(LocalizedLabel::SettingsNoteRestartNeeded.localize());
        }
    }

    fn apply_button(&self, new_value: &V, ui: &mut egui::Ui, ctx: &Context) {
        if ui
            .add_enabled(
                !self.state.is_applied,
                Button::new(LocalizedLabel::ButtonApply.localize()),
            )
            .clicked()
        {
            for command_closure in &self.commands_on_save {
                let command = command_closure.0(new_value);
                ctx.gui.try_send_ui_command(command);
            }
        }
    }

    fn save_to_config_button(&self, new_value: &V, ui: &mut egui::Ui, ctx: &Context) {
        if ui
            .add_enabled(
                !self.state.is_applied,
                Button::new(LocalizedLabel::ButtonSave.localize()),
            )
            .clicked()
        {
            for command_closure in &self.commands_on_save {
                let command = command_closure.0(new_value);
                ctx.gui.try_send_ui_command(command);
            }
            ctx.gui.try_send_ui_command(UiCommand::SaveConfig);
        }
    }

    fn save_button(&self, current: &V, ui: &mut egui::Ui, ctx: &Context) {
        if self.takes_effect_after_restart {
            self.save_to_config_button(current, ui, ctx);
        } else {
            self.apply_button(current, ui, ctx);
        }
    }

    fn reset_value_button(
        &self, ui: &mut egui::Ui, current_value: &mut V, context_value: &V,
    ) {
        if ui
            .add_enabled(!self.state.is_applied, Button::new("🔙"))
            .clicked()
        {
            *current_value = context_value.clone();
        }
    }
}

#[derive(Debug)]
pub struct SettingState {
    is_applied: bool,
}

impl Default for SettingState {
    fn default() -> Self {
        Self { is_applied: true }
    }
}

impl SettingState {
    fn check_is_applied<V>(&mut self, current_value: &V, context_value: &V)
    where
        V: PartialEq,
    {
        self.is_applied = current_value.eq(context_value);
    }
}

#[derive(Debug)]
pub struct ComboBoxSetting<V>
where
    V: Clone + PartialEq + std::fmt::Display,
{
    common: SettingsCommon<V>,
    current: V,
    possible_values: Vec<V>,
}

impl<V> SettingWidget<V> for ComboBoxSetting<V>
where
    V: Clone + PartialEq + std::fmt::Display,
{
    fn common(&self) -> &SettingsCommon<V> {
        &self.common
    }

    fn common_mut(&mut self) -> &mut SettingsCommon<V> {
        &mut self.common
    }
}

impl<V> ComboBoxSetting<V>
where
    V: Clone + PartialEq + std::fmt::Display,
{
    pub fn new(value: &V, possible_values: Vec<V>) -> Self {
        Self {
            common: Default::default(),
            current: value.clone(),
            possible_values,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, context_value: &V, ctx: &Context) {
        self.common.update_state(&self.current, context_value);

        self.render_label(ui);

        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
            egui::ComboBox::from_label("")
                .selected_text(self.current.to_string())
                .show_ui(ui, |ui| {
                    for value in &self.possible_values {
                        let is_selected = value.eq(&self.current);
                        let button = Button::selectable(is_selected, value.to_string());
                        if ui.add(button).clicked() {
                            self.current = value.clone();
                        }
                    }
                });
        });

        self.common.save_button(&self.current, ui, ctx);
        self.common
            .reset_value_button(ui, &mut self.current, context_value);

        ui.end_row();
    }
}

#[derive(Debug)]
pub struct DragValueSetting<V>
where
    V: Clone + PartialEq + egui::emath::Numeric,
{
    common: SettingsCommon<V>,
    current: V,
    range: std::ops::RangeInclusive<V>,
    step: Option<V>,
    suffix: Option<String>,
}

impl<V> SettingWidget<V> for DragValueSetting<V>
where
    V: Clone + PartialEq + egui::emath::Numeric,
{
    fn common(&self) -> &SettingsCommon<V> {
        &self.common
    }

    fn common_mut(&mut self) -> &mut SettingsCommon<V> {
        &mut self.common
    }
}

impl<V> DragValueSetting<V>
where
    V: Clone + PartialEq + egui::emath::Numeric + Into<f64>,
{
    pub fn new(
        value: &V, range: std::ops::RangeInclusive<V>, step: Option<V>,
        suffix: Option<String>,
    ) -> Self {
        Self {
            common: Default::default(),

            current: *value,
            range,
            step,
            suffix,
        }
    }

    fn show(&mut self, ui: &mut egui::Ui, context_value: &V, ctx: &Context) {
        self.common.update_state(&self.current, context_value);

        self.render_label(ui);

        let mut drag = DragValue::new(&mut self.current).range(self.range.clone());
        if let Some(step) = &self.step {
            drag = drag.speed(*step);
        }
        if let Some(suffix) = &self.suffix {
            drag = drag.suffix(suffix.clone());
        }
        ui.add(drag);

        self.common.save_button(&self.current, ui, ctx);
        self.common
            .reset_value_button(ui, &mut self.current, context_value);

        ui.end_row();
    }
}
