mod logger;
slint::include_modules!();
use native_dialog;
use native_dialog::{DialogBuilder};

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    let weak_ui = ui.as_weak();

    ui.set_is_file_included(false);
    ui.set_is_password_filled(false);
    ui.set_is_processing(false);

    ui.on_select_file(move || {
        if let Some(ui) = weak_ui.upgrade() {
            if let Some(path) = DialogBuilder::file()
                .set_location("~/Desktop")
                .add_filter("PDF Files", &["pdf"])
                .add_filter("ZIP Files", &["zip"])
                .add_filter("TAR Files", &["tar"])
                .open_single_file()
                .show()
                .unwrap(){
                let path_str = path.to_string_lossy().to_string();
                log_inf!("File: {} was selected", path_str);
                ui.set_input_file(path_str.into());
                ui.set_is_file_included(true);
            }
        }
    });
    ui.run()
}
