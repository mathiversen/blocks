use core::marker::PhantomData;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Init,
}

#[derive(Debug)]
pub struct EventDispatcher<A, F> {
    listener: Arc<Mutex<F>>,
    argument: PhantomData<A>,
}

impl<A, F> Clone for EventDispatcher<A, F> {
    #[inline]
    fn clone(&self) -> Self {
        EventDispatcher {
            listener: self.listener.clone(),
            argument: self.argument,
        }
    }
}

impl<A, F> EventDispatcher<A, F>
where
    F: FnMut(A),
{
    #[inline]
    pub fn new(listener: F) -> Self {
        Self {
            listener: Arc::new(Mutex::new(listener)),
            argument: PhantomData,
        }
    }

    #[inline]
    pub fn send(&self, event: A) {
        let listener = &mut *self.listener.lock().unwrap();
        listener(event);
    }
}
