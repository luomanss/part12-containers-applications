use futures::channel::oneshot;
use leptos::prelude::*;
use std::time::Duration;

struct Waker<T: 'static> {
    counter: u32,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Clone for Waker<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Waker<T> {}

impl<T> Default for Waker<T> {
    fn default() -> Self {
        Self {
            counter: 0,
            _phantom: std::marker::PhantomData,
        }
    }
}

unsafe impl<T> Send for Waker<T> {}
unsafe impl<T> Sync for Waker<T> {}

impl<T> Waker<T> {
    pub fn next(&self) -> Self {
        Self {
            counter: self.counter.wrapping_add(1),
            _phantom: std::marker::PhantomData,
        }
    }
}
pub struct WakerSender<T: 'static>(RwSignal<Waker<T>>);

impl<T> WakerSender<T> {
    pub fn wake(&self) {
        self.0.set(self.0.get().next());
    }
}

impl<T> Clone for WakerSender<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Copy for WakerSender<T> {}

pub struct WakerReceiver<T: 'static>(RwSignal<Waker<T>>);

impl<T> Clone for WakerReceiver<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Copy for WakerReceiver<T> {}

impl<T> WakerReceiver<T> {
    pub fn subscribe(&self) {
        let _ = self.0.get();
    }
}

pub fn use_waker<T>() -> WakerReceiver<T> {
    let waker_signal = RwSignal::new(Waker::<T>::default());
    let waker_sender = WakerSender(waker_signal.clone());
    let waker_receiver = WakerReceiver(waker_signal);

    provide_context(waker_sender);

    waker_receiver
}

pub async fn sleep(duration: Duration) {
    let (send, recv) = oneshot::channel();

    set_timeout(
        move || {
            let _ = send.send(());
        },
        duration,
    );

    let _ = recv.await;
}
