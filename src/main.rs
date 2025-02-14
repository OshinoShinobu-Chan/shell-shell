use pty::prelude::*;
use std::io::prelude::*;
use std::os::fd::AsRawFd;
use std::process::Command;

use post::post_run;
use pre::pre_run;

mod post;
mod pre;

pub const BUF_SIZE: usize = 8192;

#[derive(Debug, Clone)]
/// core struct for the shell-shell
pub struct Shsh {
    /// the master side of the pty
    master: Master,
    /// buffer, read or write
    buffer: [u8; BUF_SIZE],
}

fn trivial_read(master: &mut Master, buf: &mut [u8]) -> usize {
    master.read(buf).unwrap()
}

fn trivial_write(master: &mut Master, buf: &[u8]) -> usize {
    master.write(buf).unwrap()
}

fn main() {
    let fork = Fork::from_ptmx().unwrap();

    if let Some(master) = fork.is_parent().ok() {
        let pre_run_shsh = Shsh {
            master,
            buffer: [0; BUF_SIZE],
        };
        // let master_fd = pre_run_shsh.master.as_raw_fd();
        // let mut termios = termios::Termios::from_fd(master_fd).unwrap();
        // termios.c_lflag &= !(termios::IGNBRK | termios::ECHO | termios::ICANON);
        // termios::tcsetattr(master_fd, termios::os::linux::TCSANOW, &termios).unwrap();

        let stdin_fd = std::io::stdin().as_raw_fd();
        let mut termios = termios::Termios::from_fd(stdin_fd).unwrap();
        termios.c_lflag &= !(termios::ECHO | termios::ICANON);
        termios::tcsetattr(stdin_fd, termios::os::linux::TCSANOW, &termios).unwrap();

        let post_run_shsh = pre_run_shsh.clone();

        let handler_pre = std::thread::spawn(move || {
            pre_run(pre_run_shsh);
        });

        let handler_post = std::thread::spawn(move || {
            post_run(post_run_shsh);
        });

        handler_post.join().unwrap();
        handler_pre.join().unwrap();
    } else {
        let _ = Command::new("/bin/fish").status();
    }
}
