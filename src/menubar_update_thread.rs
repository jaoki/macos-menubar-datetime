use std::{sync::{mpsc, Arc}, thread::{self, JoinHandle}, time::Duration};

use chrono::Utc;

use crate::datetime_menubar_app::MutexButton;

// TODO this should not be needed after a time is calculated in MenubarUpdateThread
pub enum MenubarTextMessage {
    UpdateIcon(String),
    Quit,
}


pub struct MenubarUpdateThread {
    rx: mpsc::Receiver<MenubarTextMessage>,
// TODO can this Arc be removed?
    button: Arc<MutexButton>
}

impl MenubarUpdateThread {

// TODO can this Arc be removed?
    pub fn new(
        rx: mpsc::Receiver<MenubarTextMessage>, 
        button: Arc<MutexButton>
    ) -> Self {
        Self { rx, button }
    }

    pub fn spawn(self) -> JoinHandle<()> {
        thread::spawn(move || {
            self.run();
        })
    }

    pub fn run(self) {
        while let Ok(msg) = self.rx.recv() {
            match msg {
                MenubarTextMessage::UpdateIcon(text) => self.fun_name(&text),
                MenubarTextMessage::Quit => break,
            }
        };
    }

    fn fun_name(&self, text: &String) {
        let utc_now = Utc::now();
        let time_str = utc_now.format("%H:%M:%S").to_string();


        self.button.set_text(&time_str)
    }

}


pub struct TimerThread {
    tx: mpsc::Sender<MenubarTextMessage>
}

// TODO TimerThread and mpsc::channel are suboptimal. A single thread (MenubarUpdateThread) with an infinite loop should do the job
impl TimerThread {
    pub fn new(tx: mpsc::Sender<MenubarTextMessage>) -> Self {
        Self { tx }
    }

    pub fn spawn(self) -> JoinHandle<()> {
        thread::spawn(move || {
            self.run();
        })
    }

    fn run(self) {
        loop {
            thread::sleep(Duration::from_secs(1));

            // TODO don't need to send any String to tx. Clean this up
            let var_name = String::from("");
            if self.tx.send(MenubarTextMessage::UpdateIcon(var_name)).is_err() {
                break;
            }
        }
    }
}

