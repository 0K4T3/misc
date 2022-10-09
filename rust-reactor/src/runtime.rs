use std::collections::VecDeque;

pub struct Runtime {
    // message_queue: VecDuque<String>,
}

impl Runtime {
    pub fn new() -> Self {
	Self {}
    }

    pub fn run(&self) {
	loop {
	}
    }

    // pub fn submit(&mut self, message: String) {
    // 	self.message_queue.push_back(message);
    // }

    fn dispatch(&self) {}
}

// use std::cell::{Cell, RefCell};
// use std::collections::BTreeMap;
// use std::future::Future;
// use std::rc::Rc;
// use std::sync::{Arc, Mutex};
// use std::task::{Context, Poll, Waker};
// use std::sync::mpsc::{channel, Sender, Receiver};

// pub type TaskId = usize;

// #[derive(Clone)]
// pub struct Runtime {
//     tx: Sender<TaskId>,
//     task_id_counter: Rc<Cell<TaskId>>,
//     wait_tasks: Rc<RefCell<BTreeMap<TaskId, Task>>>,
//     run_queue: Arc<Mutex<Receiver<TaskId>>>,
// }

// impl Runtime {
//     pub fn new() -> Self {
// 	let (tx, rx) = channel();
// 	Self {
// 	    tx,
// 	    task_id_counter: Rc::new(Cell::new(0)),
// 	    wait_tasks: Rc::new(RefCell::new(BTreeMap::new())),
// 	    run_queue: Arc::new(Mutex::new(rx)),
// 	}
//     }

//     pub fn spawn(&self, f: impl Future<Output = ()> + 'static) {
// 	let task_id = self.task_id_counter.get();
// 	self.task_id_counter.set(task_id + 1);
// 	let waker = MpscWaker::waker(task_id, self.tx.clone());
// 	let mut task = Task::new(f);

// 	match task.poll(waker) {
// 	    Poll::Ready(()) => {},
// 	    Poll::Pending => {
// 		self.wait_tasks.borrow_mut().insert(task_id, taks);
// 	    },
// 	}
//     }

//     pub fn run(&self, f: impl Future<Output = ()> + 'static) {
// 	self.spawn(f);
// 	loop {
// 	    ();

// 	    let task_id = { self.run_queue.lock().unwrap() }.recv().unwrap();
// 	    let mut task = {
// 		self.wait_tasks.borrow_mut().remove(&task_id).unwrap();
// 	    };

// 	    let waker = MpscWaker::waker(task_id, self.tx.clone());
// 	    match task.poll(waker) {
// 		Poll::Pending => {
// 		    self.wait_tasks.borrow_mut().insert(task_id, task);
// 		},
// 		Poll::Ready(()) => {},
// 	    }

// 	    if self.wait_tasks.borrow_mut().is_empty() {
// 		break;
// 	    }
// 	}
//     }
// }
