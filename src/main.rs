mod logger;
mod ui_bindings;
mod common;

use slint::ComponentHandle;
use ui_bindings::UiBindings;
use common::MainWindow; // Import MainWindow from common.rs

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?; // Use the centralized MainWindow
    let ui_bindings = UiBindings::new(&ui);
    ui.run()
}