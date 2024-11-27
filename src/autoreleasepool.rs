use cocoa::base::{id, nil};
use cocoa::foundation::NSAutoreleasePool;


pub struct AutoreleasePool {
    pool: id
}

impl AutoreleasePool {
    pub fn new() -> Self {
        unsafe {
            Self {
                pool: NSAutoreleasePool::new(nil),

            }
        }

    }

}

impl Drop for AutoreleasePool {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.pool, drain];
        }
    }
}
 