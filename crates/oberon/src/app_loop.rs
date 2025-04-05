use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Default)]
pub struct Loop
{
    shutdown: AtomicBool,
}

impl Loop
{
    pub fn is_running(&self) -> bool
    {
        !self.shutdown.load(Ordering::SeqCst)
    }

    pub fn shutdown(&self)
    {
        self.shutdown.store(true, Ordering::SeqCst);
    }
}
