use std::error::Error;
use std::fs::File;
use std::os::unix::io::{FromRawFd};

use nix::pty;

fn main() -> Result<(), Box<dyn Error>> {
    let result = pty::openpty(None, None)?;

    let master = unsafe { File::from_raw_fd(result.master) };
    println!("{:?}", master);

    Ok(())
}
