use asr::settings::gui::Title;
use asr::settings::Gui;

#[derive(Gui)]
pub struct Settings {
    /// Welcome to the ANTONBLAST autosplitter settings
    _message: Title,

    /// Start Options
    _timer_mode_title: Title,

    #[default = true]
    /// Enable
    pub start_enable: bool,

    /// Split Options
    _splits_title: Title,

    #[default = true]
    /// Enable
    pub splits_enable: bool,

    /// Reset Options
    _reset_title: Title,

    #[default = true]
    /// Enable
    pub reset_enable: bool,
}
