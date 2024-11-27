use cocoa::appkit::{
    NSStatusBar, 
    NSStatusItem, 
    NSVariableStatusItemLength, 
};

use cocoa::base::{
    id,
    nil
};

use std::sync::{Arc, Mutex};

use cocoa::foundation::NSString;

mod thread_safe_id;
use thread_safe_id::ThreadSafeId;


pub struct MutexButton {
    // Use interior mutability pattern
    button: Mutex<ThreadSafeId>,
}

// unsafe because button is shared by sender 
unsafe impl Send for MutexButton {}

impl MutexButton {
    fn new(button: id) -> Self {
        Self {
            button: Mutex::new(ThreadSafeId::new(button)),
        }
    }

    pub fn set_text(&self, text: &str) {
        if let Ok(button) = self.button.lock() {
            unsafe {
                let title = NSString::alloc(nil).init_str(text);
                let _: () = msg_send![button.get_raw_id(), setTitle:title];
            }
        }
        // TODO do error handling?
    }

}




pub struct DatetimeMenubarApp {
    status_bar_item: id,
    button: Arc<MutexButton>
}

impl DatetimeMenubarApp {
    pub fn new() -> Self {
        let status_bar_item: id;
        let button: Arc<MutexButton>;
        unsafe {

            let status_bar: id = NSStatusBar::systemStatusBar(nil);
            status_bar_item = status_bar.statusItemWithLength_(NSVariableStatusItemLength);
            let raw_button = status_bar_item.button();
            button = Arc::new(MutexButton::new(raw_button));

        }

        Self{status_bar_item, button}
    }

    // TODO remove this as this is just a wrapper
    pub fn set_text(&self, text: &str) {
        self.button.set_text(text);
    }

    pub fn setup_status_bar(&self) {
        unsafe {
            // Create the menu
            let menu: id = msg_send![class!(NSMenu), new];

            // add a quit item
            let quit_title = NSString::alloc(nil).init_str("Quit");
            let quit_action = sel!(terminate:);
            let quit_item: id = msg_send![class!(NSMenuItem), alloc];
            let _: () = msg_send![quit_item,
                initWithTitle: quit_title
                action: quit_action
                keyEquivalent: NSString::alloc(nil).init_str("q")
            ];

            
            let _: () = msg_send![menu, addItem: quit_item];
            let _: () = msg_send![self.status_bar_item, setMenu: menu];

        }

    }

    pub fn get_button(&self) -> Arc<MutexButton> {
        Arc::clone(&self.button)
    }

}

