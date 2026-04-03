use std::cell::Cell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Orientation, PolicyType};
use webkit2gtk::{LoadEvent, NavigationAction, URIRequestExt, WebContext, WebView, WebViewExt};

use crate::error_page::{render_error_page, render_start_page};
use crate::navigation::{normalize_user_input, NavigationError};

const APP_NAME: &str = "Skryvola";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LoadState {
    Idle,
    Loading,
}

pub struct BrowserWindow {
    window: gtk::ApplicationWindow,
}

impl BrowserWindow {
    pub fn new(app: &gtk::Application, initial_url: Option<String>) -> Self {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title(APP_NAME);
        window.set_default_size(1200, 800);

        let root = gtk::Box::new(Orientation::Vertical, 0);
        let toolbar = gtk::Box::new(Orientation::Horizontal, 6);
        toolbar.set_margin_top(8);
        toolbar.set_margin_bottom(8);
        toolbar.set_margin_start(8);
        toolbar.set_margin_end(8);

        let back_button = gtk::Button::with_label("Back");
        back_button.set_sensitive(false);

        let forward_button = gtk::Button::with_label("Forward");
        forward_button.set_sensitive(false);

        let reload_button = gtk::Button::with_label("Reload");

        let address_entry = gtk::Entry::new();
        address_entry.set_hexpand(true);
        address_entry.set_placeholder_text(Some("Enter a URL"));

        toolbar.pack_start(&back_button, false, false, 0);
        toolbar.pack_start(&forward_button, false, false, 0);
        toolbar.pack_start(&reload_button, false, false, 0);
        toolbar.pack_start(&address_entry, true, true, 0);

        let progress_bar = gtk::ProgressBar::new();
        progress_bar.set_show_text(false);
        progress_bar.set_fraction(0.0);

        let scrolled_window =
            gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        scrolled_window.set_policy(PolicyType::Automatic, PolicyType::Automatic);
        scrolled_window.set_hexpand(true);
        scrolled_window.set_vexpand(true);

        let web_context = WebContext::new_ephemeral();
        let web_view = WebView::with_context(&web_context);
        scrolled_window.add(&web_view);

        root.pack_start(&toolbar, false, false, 0);
        root.pack_start(&progress_bar, false, false, 0);
        root.pack_start(&scrolled_window, true, true, 0);

        window.add(&root);

        let load_state = Rc::new(Cell::new(LoadState::Idle));

        connect_entry(&address_entry, &web_view);
        connect_navigation_buttons(
            &back_button,
            &forward_button,
            &reload_button,
            &web_view,
            Rc::clone(&load_state),
        );
        connect_web_view_events(
            &window,
            &address_entry,
            &back_button,
            &forward_button,
            &reload_button,
            &progress_bar,
            &web_view,
            Rc::clone(&load_state),
        );

        window.show_all();
        progress_bar.hide();

        match initial_url {
            Some(initial_url) => load_text_request(&web_view, &initial_url),
            None => web_view.load_html(&render_start_page(), Some("about:blank")),
        }

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}

fn connect_entry(address_entry: &gtk::Entry, web_view: &WebView) {
    let web_view = web_view.clone();
    address_entry.connect_activate(move |entry| {
        let text = entry.text().to_string();
        load_text_request(&web_view, &text);
    });
}

fn connect_navigation_buttons(
    back_button: &gtk::Button,
    forward_button: &gtk::Button,
    reload_button: &gtk::Button,
    web_view: &WebView,
    load_state: Rc<Cell<LoadState>>,
) {
    let back_view = web_view.clone();
    back_button.connect_clicked(move |_| {
        back_view.go_back();
    });

    let forward_view = web_view.clone();
    forward_button.connect_clicked(move |_| {
        forward_view.go_forward();
    });

    let reload_view = web_view.clone();
    reload_button.connect_clicked(move |_| {
        if load_state.get() == LoadState::Loading {
            reload_view.stop_loading();
        } else {
            reload_view.reload();
        }
    });
}

fn connect_web_view_events(
    window: &gtk::ApplicationWindow,
    address_entry: &gtk::Entry,
    back_button: &gtk::Button,
    forward_button: &gtk::Button,
    reload_button: &gtk::Button,
    progress_bar: &gtk::ProgressBar,
    web_view: &WebView,
    load_state: Rc<Cell<LoadState>>,
) {
    let reload_button_for_load = reload_button.clone();
    let progress_bar_for_load = progress_bar.clone();
    let load_state_for_load = Rc::clone(&load_state);
    web_view.connect_load_changed(move |_, event| match event {
        LoadEvent::Started => {
            set_load_state(
                &load_state_for_load,
                &reload_button_for_load,
                &progress_bar_for_load,
                LoadState::Loading,
            );
        }
        LoadEvent::Finished => {
            progress_bar_for_load.set_fraction(0.0);
            set_load_state(
                &load_state_for_load,
                &reload_button_for_load,
                &progress_bar_for_load,
                LoadState::Idle,
            );
        }
        _ => {}
    });

    let failing_view = web_view.clone();
    let load_state_for_fail = Rc::clone(&load_state);
    let reload_button_for_fail = reload_button.clone();
    let progress_bar_for_fail = progress_bar.clone();
    web_view.connect_load_failed(move |_, _, failing_uri: &str, error: &glib::Error| {
        set_load_state(
            &load_state_for_fail,
            &reload_button_for_fail,
            &progress_bar_for_fail,
            LoadState::Idle,
        );
        failing_view.load_html(
            &render_error_page("Page Load Failed", &error.to_string(), Some(failing_uri)),
            Some("about:blank"),
        );
        true
    });

    let tls_view = web_view.clone();
    let load_state_for_tls = Rc::clone(&load_state);
    let reload_button_for_tls = reload_button.clone();
    let progress_bar_for_tls = progress_bar.clone();
    web_view.connect_load_failed_with_tls_errors(move |_, failing_uri: &str, _, errors| {
        set_load_state(
            &load_state_for_tls,
            &reload_button_for_tls,
            &progress_bar_for_tls,
            LoadState::Idle,
        );
        tls_view.load_html(
            &render_error_page(
                "TLS Error",
                &format!("The connection failed TLS validation: {errors:?}"),
                Some(failing_uri),
            ),
            Some("about:blank"),
        );
        true
    });

    let progress_bar_for_progress = progress_bar.clone();
    let load_state_for_progress = Rc::clone(&load_state);
    web_view.connect_notify_local(Some("estimated-load-progress"), move |web_view, _| {
        let fraction = web_view.estimated_load_progress();
        progress_bar_for_progress.set_fraction(fraction);
        if load_state_for_progress.get() == LoadState::Loading {
            progress_bar_for_progress.show();
        }
    });

    let window_for_title = window.clone();
    web_view.connect_notify_local(Some("title"), move |web_view, _| {
        update_window_title(&window_for_title, web_view.title().as_deref());
    });

    let address_entry_for_uri = address_entry.clone();
    web_view.connect_notify_local(Some("uri"), move |web_view, _| {
        if let Some(uri) = web_view.uri() {
            address_entry_for_uri.set_text(uri.as_str());
        }
    });

    let back_button_for_notify = back_button.clone();
    web_view.connect_notify_local(Some("can-go-back"), move |web_view, _| {
        back_button_for_notify.set_sensitive(web_view.can_go_back());
    });

    let forward_button_for_notify = forward_button.clone();
    web_view.connect_notify_local(Some("can-go-forward"), move |web_view, _| {
        forward_button_for_notify.set_sensitive(web_view.can_go_forward());
    });

    let existing_web_view = web_view.clone();
    web_view.connect_create(move |_, navigation_action: &NavigationAction| {
        if let Some(request) = navigation_action.request() {
            if let Some(uri) = request.uri() {
                existing_web_view.load_uri(uri.as_str());
            }
        }
        None
    });
}

fn load_text_request(web_view: &WebView, text: &str) {
    match normalize_user_input(text) {
        Ok(request) => web_view.load_uri(&request.normalized_uri),
        Err(error) => load_navigation_error(web_view, text, error),
    }
}

fn load_navigation_error(web_view: &WebView, attempted: &str, error: NavigationError) {
    let (title, detail) = match error {
        NavigationError::EmptyInput => (
            "Address Bar Is Empty",
            "Enter a URL before pressing Enter.",
        ),
        NavigationError::InvalidUri(_) => (
            "Invalid Address",
            "The text in the address bar is not a valid absolute URL or host.",
        ),
    };

    web_view.load_html(
        &render_error_page(title, detail, Some(attempted.trim())),
        Some("about:blank"),
    );
}

fn set_load_state(
    load_state: &Cell<LoadState>,
    reload_button: &gtk::Button,
    progress_bar: &gtk::ProgressBar,
    next_state: LoadState,
) {
    load_state.set(next_state);
    match next_state {
        LoadState::Idle => {
            reload_button.set_label("Reload");
            progress_bar.hide();
        }
        LoadState::Loading => {
            reload_button.set_label("Stop");
            progress_bar.show();
        }
    }
}

fn update_window_title(window: &gtk::ApplicationWindow, title: Option<&str>) {
    let window_title = match title {
        Some(title) if !title.trim().is_empty() => format!("{title} - {APP_NAME}"),
        _ => APP_NAME.to_string(),
    };
    window.set_title(&window_title);
}
