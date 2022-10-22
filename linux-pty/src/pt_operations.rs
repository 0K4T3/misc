use nix::{fcntl, pty};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
use std::io::Write;

fn create_pt() -> Result<(std::os::unix::io::RawFd, std::path::PathBuf), Box<dyn std::error::Error>> {
    let pt = pty::posix_openpt(
        fcntl::OFlag::O_RDWR | fcntl::OFlag::O_NOCTTY
    )?;
    pty::grantpt(&pt)?;
    pty::unlockpt(&pt)?;

    let ptsname = pty::ptsname_r(&pt)?.into();
    let pt_fd = pt.into_raw_fd();

    Ok((pt_fd, ptsname))
}

// nix::ioctl_write_ptr_bad!(
//     set_term_size_unsafe,
//     libc::TIOCSWINSZ,
//     nix::pty::Winsize
// );

struct Size {
    row: u16,
    col: u16,
    xpixel: u16,
    ypixel: u16,
}

impl Size {
    pub fn new(row: u16, col: u16) -> Self {
        Self {
            row,
            col,
            xpixel: 0,
            ypixel: 0,
        }
    }
}

impl From<&Size> for nix::pty::Winsize {
    fn from(size: &Size) -> Self {
        Self {
            ws_row: size.row,
            ws_col: size.col,
            ws_xpixel: size.xpixel,
            ws_ypixel: size.ypixel,
        }
    }
}

#[derive(Debug)]
struct Pty {
    pt: std::fs::File,
    ptsname: std::path::PathBuf,
}

impl Pty {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (pt_fd, ptsname) = create_pt()?;
        let pt = unsafe { std::fs::File::from_raw_fd(pt_fd) };

        Ok(Self { pt, ptsname })
    }

    fn pt(&self) -> &std::fs::File {
        &self.pt
    }

    fn pts(&self) -> Result<std::fs::File, Box<dyn std::error::Error>> {
        let fh = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.ptsname)?;

        Ok(fh)
    }

    fn resize(&self, size: &Size) -> Result<(), Box<dyn std::error::Error>> {
        // let fd = self.pt().as_raw_fd();
        // let size = size.into();

        // unsafe {
        //     set_term_size_unsafe(fd, ::std::ptr::NonNull::from(&size).as_ptr())?;
        // }

        Ok(())
    }
}

fn setup_pty() -> Result<
        (Pty, std::fs::File, std::os::unix::io::RawFd, std::os::unix::io::RawFd, std::os::unix::io::RawFd),Box<dyn std::error::Error>>
{
    let pty = Pty::new()?;
    let pts = pty.pts()?;

    let pts_fd = pts.as_raw_fd();
    let stdin = nix::unistd::dup(pts_fd)?;
    let stdout = nix::unistd::dup(pts_fd)?;
    let stderr = nix::unistd::dup(pts_fd)?;

    Ok((pty, pts, stdin, stdout, stderr))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (pty, pts, stdin, stdout, stderr) = setup_pty()?;
    println!("PTY: {:?}", pty);
    println!("PTS: {:?}", pts);
    println!("PTS stdin: {:?}", stdin);
    println!("PTS stdout: {:?}", stdout);
    println!("PTS stderr: {:?}", stderr);

    let mut shell_process = std::process::Command::new("/bin/bash");
    shell_process
        .stdin(unsafe { std::process::Stdio::from_raw_fd(stdin) })
        .stdout(unsafe { std::process::Stdio::from_raw_fd(stdout) })
        .stderr(unsafe { std::process::Stdio::from_raw_fd(stderr) });
    // shell_process.spawn()?;

    println!("{:?}", shell_process);

    Ok(())
}
