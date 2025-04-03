use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub struct Loop
{
    shutdown: AtomicBool,
}

impl Loop
{
    pub fn new() -> Self
    {
        Self {
            shutdown: AtomicBool::new(false),
        }
    }

    pub fn is_running(&self) -> bool
    {
        !self.shutdown.load(Ordering::SeqCst)
    }

    pub fn shutdown(&self)
    {
        self.shutdown.store(true, Ordering::SeqCst);
    }
}
