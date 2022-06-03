use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn proof_of_rc() {
    // Rc is a smart pointer which manages reference count.
    // You can share ownership for reference by multiple variables.

    // create Rc instance
    let x = Rc::new(42);
    assert_eq!(Rc::strong_count(&x), 1);

    // increase reference count by clone method
    let y = x.clone();
    assert_eq!(Rc::strong_count(&y), 2);

    // Check whether two variables share ownership for one reference or not
    assert!(Rc::ptr_eq(&x, &y));
    eprintln!("x = {0:p} (points to {0:})", x);
    eprintln!("y = {0:p} (points to {0:})", y);

    // However, it is not thread safe.
    // The sample code below, it cannot be compiled.
    //
    // let thread = thread::spawn(|| {
    //     eprintln!("value = {}", x);
    // });
}

fn proof_of_arc() {
    // Arc is a smart pointer like Rc
    // But it can be shared by multiple thread (aka. thread safe)

    let x = Arc::new(42);
    let y = x.clone();

    let thread_1 = thread::spawn(move || {
	eprintln!("thread_1: value = {}", x);
	eprintln!("thread_1: address = {:p}", x);
    });

    let thread_2 = thread::spawn(move || {
	eprintln!("thread_2: value = {}", y);
	eprintln!("thread_2: address = {:p}", y);
    });

    thread_1.join().unwrap();
    thread_2.join().unwrap();
}

fn main() -> Result<(), std::io::Error> {
    proof_of_rc();
    proof_of_arc();

    Ok(())
}
