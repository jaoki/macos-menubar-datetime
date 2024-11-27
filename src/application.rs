use cocoa::base::{
    id,
    nil,
    YES
};
use cocoa::appkit::{
    NSApplication,
    NSApplicationActivationPolicy
};

pub struct NSApplicationWrapper {
    app: id
}

impl NSApplicationWrapper {
    pub fn new() -> Self {
        unsafe {
            // Get and initialize the shared application
            let app = NSApplication::sharedApplication(nil);
            app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
            Self {app}
        }

    }

    pub fn run(&self) {
        unsafe {
            // Indicate that this is a background application
            self.app.activateIgnoringOtherApps_(YES);

            // Set up as agent (status bar) application
            self.app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);

            // Finish launching and run the application
            self.app.finishLaunching();
            self.app.run();
        }

    }

}