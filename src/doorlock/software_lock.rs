use tracing::info;

use super::DoorLockTrait;

#[derive(Debug)]
pub struct DoorLock {
    is_open: bool,
}

impl DoorLockTrait for DoorLock {
    fn new() -> Self {
        info!("software door initialized");
        Self { is_open: false }
    }

    fn open(&mut self) {
        info!("door open");
        self.is_open = true;
    }

    fn close(&mut self) {
        info!("door close");
        self.is_open = false;
    }

    fn is_open(&self) -> bool {
        false
    }
}
