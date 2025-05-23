use crate::{log_dbg, log_inf, log_wrn, log_err};
use crate::common::MainWindow; // Import MainWindow from common.rs
use slint::{ComponentHandle, Weak};
use native_dialog::{DialogBuilder};

pub struct UiBindings;

impl UiBindings {
    pub fn new(ui: &MainWindow) -> Self {
        let instance = Self;
        instance.setup_callbacks(ui);
        instance
    }

    fn setup_callbacks(&self, ui: &MainWindow) {
        self.setup_log_dbg(&ui);
        self.setup_select_file(&ui);
    }

    fn setup_log_dbg(&self, ui: &MainWindow) {
        let weak_ui = ui.as_weak();
        ui.on_log_dbg(move |message| {
            if let Some(ui) = weak_ui.upgrade() {
                log_dbg!("{}", message);
            }
        });
    }

    fn setup_select_file(&self, ui: &MainWindow) {
        let weak_ui = ui.as_weak();
        ui.on_select_file(move || {
            if let Some(ui) = weak_ui.upgrade() {

                let dialog_result = DialogBuilder::file()
                    .set_location("~/Desktop")
                    .add_filter("PDF Files", &["pdf"])
                    .add_filter("ZIP Files", &["zip"])
                    .add_filter("TAR Files", &["tar"])
                    .open_single_file()
                    .show();

                match dialog_result {
                    Ok(Some(path)) => {
                        let path_str = path.to_string_lossy().to_string();
                        log_inf!("Selected file: \"{}\"", path_str);
                        ui.set_input_file(path_str.into());
                        ui.set_is_file_included(true);
                    }
                    Ok(None) => {
                        log_inf!("File selection cancelled.");
                    }
                    Err(e) => {
                        log_err!("Error opening file dialog: {}", e);
                    }
                }
            }
        });
    }
}