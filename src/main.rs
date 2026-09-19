// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

mod dat;
mod paths;
mod util;

use native_dialog::DialogBuilder;

fn main() -> Result<(), slint::PlatformError> {
    let title = format!(
        "Minecraft 26.1 World Downgrader (v{})",
        env!("CARGO_PKG_VERSION")
    );

    let ui = Main::new()?;
    ui.set_args_title(title.into());
    let ui_handle = ui.as_weak();

    ui.on_execute_downgrade(move || {
        let ui = ui_handle.unwrap();

        // Locate directory
        let path = DialogBuilder::file().open_single_dir().show().unwrap();

        let Some(path) = path else {
            ui.set_status("User cancelled the file dialogue.".into());
            return;
        };

        let result = paths::create_downgraded_copy(&path);

        match result {
            Ok(_) => {
                ui.set_status(
                    "Successfully created a downgraded copy (in the same folder).".into(),
                );
            }
            Err(error) => {
                ui.set_status(format!("{error}").into());
            }
        }
    });

    ui.run()

    //Ok(())
}
