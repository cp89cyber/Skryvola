#[cfg(not(target_os = "linux"))]
compile_error!("Skryvola only supports Linux.");

mod browser_window;
mod error_page;
mod navigation;

use browser_window::BrowserWindow;
use gio::ApplicationFlags;
use gtk::prelude::*;

fn main() {
    let program_name = std::env::args()
        .next()
        .unwrap_or_else(|| "skryvola".to_string());
    let initial_url = match parse_args() {
        Ok(initial_url) => initial_url,
        Err(exit_code) => std::process::exit(exit_code),
    };

    let app = gtk::Application::new(Some("com.skryvola.Browser"), ApplicationFlags::FLAGS_NONE);
    app.connect_activate(move |app| {
        let browser = BrowserWindow::new(app, initial_url.clone());
        browser.present();
    });
    let exit_code: i32 = app.run_with_args(&[program_name]).into();
    std::process::exit(exit_code);
}

fn parse_args() -> Result<Option<String>, i32> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [] => Ok(None),
        [initial_url] => Ok(Some(initial_url.clone())),
        _ => {
            eprintln!("Usage: skryvola [initial_url]");
            Err(2)
        }
    }
}
