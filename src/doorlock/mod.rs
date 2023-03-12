#[cfg(feature = "motor")]
#[path = "hardware_lock.rs"]
mod doorlock_impl;
#[cfg(not(feature = "motor"))]
#[path = "software_lock.rs"]
mod doorlock_impl;

pub use doorlock_impl::DoorLock;

pub trait DoorLockTrait {
    fn new() -> Self;
    fn open(&mut self);
    fn close(&mut self);
    fn is_open(&self) -> bool;
}
