use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

pub struct Task {
    futuer: Pin<Box<dyn Future<Output = ()> + 'static>>,
}

impl Task {
    fn new(f: impl Future<Output = ()> + 'static) -> Self {
	Self {
	    future: Box::pin(f),
	}
    }

    fn poll(&mut self, waker: Waker) -> Poll<()> {
	let mut ctx = Context::from_waker(&waker);
	match Future::poll(self.future.as_mut(), &mut ctx) {
	    Poll::Pending => Poll::Pending,
	    Poll::Ready(()) => Poll::Ready(()),
	}
    }
}
