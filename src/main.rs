use std::sync::mpsc;

mod datetime_menubar_app;
use datetime_menubar_app::DatetimeMenubarApp;
use menubar_update_thread::{MenubarUpdateThread, TimerThread};

#[macro_use]
extern crate objc;

mod autoreleasepool;
mod application;
mod menubar_update_thread;



fn main() {
    let _pool = autoreleasepool::AutoreleasePool::new();

    let nsapp = application::NSApplicationWrapper::new();

    let menubar_app = DatetimeMenubarApp::new();

    menubar_app.set_text("🦀rust");
    menubar_app.setup_status_bar();

    let (tx, rx) = mpsc::channel();

    let timer_thread = TimerThread::new(tx);
    timer_thread.spawn();
    let button = menubar_app.get_button();

    let menubar_update_thread = MenubarUpdateThread::new(rx, button);
    menubar_update_thread.spawn();

    nsapp.run()

}


