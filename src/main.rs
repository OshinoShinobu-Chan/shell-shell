use libc::c_int;

use pty::prelude::*;

use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;

use std::os::fd::AsRawFd;
use std::os::unix::process::CommandExt;
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

fn clear_termios(old_termios: termios::Termios) {
    let stdin_fd = std::io::stdin().as_raw_fd();
    let mut termios = termios::Termios::from_fd(stdin_fd).unwrap();
    termios.c_lflag = old_termios.c_lflag;
    termios::tcsetattr(stdin_fd, termios::os::linux::TCSANOW, &termios).unwrap();
}

fn main() {
    let fork = Fork::from_ptmx().unwrap();
    let termios = termios::Termios::from_fd(std::io::stdin().as_raw_fd()).unwrap();
    let old_termios = termios.clone();

    if let Some(master) = fork.is_parent().ok() {
        let child_pid = {
            if let Fork::Parent(pid, _) = fork {
                pid
            } else {
                unreachable!()
            }
        };

        let mut signals =
            Signals::new(&[SIGINT, SIGTSTP, SIGCHLD]).expect("Failed to register signals");
        let sig_handle = signals.handle();
        let sig_thread = std::thread::spawn(move || {
            for signal in &mut signals {
                match signal {
                    SIGINT | SIGTSTP => unsafe {
                        println!("Sending signal {} to child process {}", signal, child_pid);
                        libc::kill(child_pid as c_int, signal);
                    },
                    SIGCHLD => {
                        println!("Inner shell exited");
                        clear_termios(old_termios);
                        std::process::exit(0);
                    }
                    _ => unreachable!(),
                }
            }
        });

        let pre_run_shsh = Shsh {
            master,
            buffer: [0; BUF_SIZE],
        };

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
        sig_handle.close();
        sig_thread.join().unwrap();
        clear_termios(old_termios);
    } else {
        let err = Command::new("/bin/fish").exec();
        eprintln!("Error executing /bin/fish: {:?}", err);
    }
}
