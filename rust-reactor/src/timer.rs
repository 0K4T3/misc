use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread::JoinHandle;
use std::time::Duration;

pub struct Timeout {
    th: Option<JoinHandle<()>>,
    state: Arc<Mutex<Poll<()>>>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl Timeout {
    pub fn set(duration: Duration) -> Self {
	let waker = Arc::new(Mutex::new(None::<Waker>));
	let state = Arc::new(Mutex::new(Poll::Pending));
	let w = waker.clone();
	let s = state.clone();
	let th = std::thread::spawn(move || {
	    std::thread::sleep(duration);

	    let mut state = s.lock().unwrap();
	    *state = Poll::Ready(());

	    if let Some(waker) = &*w.lock().unwrap() {
		waker.wake_by_ref()
	    }
	});

	Self {
	    th: Some(th),
	    state,
	    waker,
	}
    }
}

impl Future for Timeout {
    type Output = ();

    fn poll(mut self: Pin<&mut self>, ctx: &mut Context) -> Poll<Self::Output> {
	*self.waker.lock().unwrap() = Some(ctx.waker().clone());
	match *self.state.lock().unwrap() {
	    Poll::Pending => Poll::Pending,
	    Poll::Ready(()) => Poll::Ready(()),
	}
    }
}

impl Drop for Timeout {
    fn drop(&mut self) {
	self.th.take().unwrap().join().unwrap();
    }
}
