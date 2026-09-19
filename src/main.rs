// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

mod dat;
mod paths;

fn main() -> Result<(), slint::PlatformError> {
    /*paths::create_downgraded_copy(&String::from(
        r"/home/watduhhekbro/downloads/galarov/GALAROV_TEST_done/",
    ));*/

    // if result = error, then delete new directory probably

    //let main_window = MainWindow::new()?;
    //main_window.run()
    Ok(())
}
