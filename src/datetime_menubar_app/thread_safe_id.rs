use cocoa::base::id;

pub struct ThreadSafeId {
    raw_id: id
}

impl ThreadSafeId {
    pub fn new(button: id) -> Self {
        Self {
            raw_id: button
        }
    }

    pub fn get_raw_id(&self) -> id {
        self.raw_id
    }
}


unsafe impl Send for ThreadSafeId {}
unsafe impl Sync for ThreadSafeId {}
