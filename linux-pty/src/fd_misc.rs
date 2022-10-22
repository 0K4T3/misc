use std::error::Error;
use std::fs::File;
use std::io::{stdout, Write};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin_file_obj = unsafe { File::from_raw_fd(0) };
    let mut stdout_file_obj = unsafe { File::from_raw_fd(1) };
    let mut stderr_file_obj = unsafe { File::from_raw_fd(2) };

    println!("stdin: {:?}", stdin_file_obj);
    println!("stdout: {:?}", stdout_file_obj);
    println!("stderr: {:?}", stderr_file_obj);

    stdin_file_obj.write(b"hello to stdin\n")?;
    stdout_file_obj.write(b"hello to stdout\n")?;
    stderr_file_obj.write(b"hello to stderr\n")?;

    let mut proc = Command::new("echo");
    proc.arg("hello")
        .stdout(unsafe { File::from_raw_fd(stdout().as_raw_fd()) });
    proc.spawn()?;

    Ok(())
}
