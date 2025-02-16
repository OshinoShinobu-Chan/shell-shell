use libc::c_int;

use pty::prelude::*;

use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;

use log::{debug, info};

use std::os::fd::AsRawFd;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::time::SystemTime;

use error::{Error, ErrorType};
use post::post_run;
use pre::pre_run;

mod error;
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

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn clear_termios(old_termios: termios::Termios) {
    debug!("Clearing termios...");
    let stdin_fd = std::io::stdin().as_raw_fd();
    let mut termios = termios::Termios::from_fd(stdin_fd).unwrap();
    termios.c_lflag = old_termios.c_lflag;
    termios::tcsetattr(stdin_fd, termios::os::linux::TCSANOW, &termios).unwrap();
}

fn main() {
    setup_logger().expect("Failed to setup logger");

    info!("Starting shell-shell...");

    let fork = Fork::from_ptmx().unwrap();
    let termios = termios::Termios::from_fd(std::io::stdin().as_raw_fd()).unwrap();
    let old_termios = termios.clone();

    // create psuedo terminal
    debug!("Creating psuedo terminal...");
    if let Some(master) = fork.is_parent().ok() {
        let child_pid = {
            if let Fork::Parent(pid, _) = fork {
                pid
            } else {
                Error::new(
                    "Failed to get child pid".to_string(),
                    ErrorType::UnreachableCode,
                )
                .print();
                std::process::exit(1);
            }
        };
        debug!("Child pid: {}.", child_pid);

        // signal handler
        debug!("Creating signal handler...");
        let signals = Signals::new(&[SIGINT, SIGTSTP, SIGCHLD]);
        if signals.is_err() {
            Error::new(
                format!("Failed to create signal handler: {:?}", signals.err()),
                ErrorType::OtherError,
            )
            .print();
            std::process::exit(1);
        }
        let mut signals = signals.unwrap();
        let sig_handle = signals.handle();
        let sig_thread = std::thread::spawn(move || {
            for signal in &mut signals {
                match signal {
                    SIGINT | SIGTSTP => unsafe {
                        info!("Sending signal {} to child process {}.", signal, child_pid);
                        libc::kill(child_pid as c_int, signal);
                    },
                    SIGCHLD => {
                        info!("Child process {} exited. Exiting gracefully...", child_pid);
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

        // disable echo and canonical mode
        debug!("Disabling echo and canonical mode...");
        let stdin_fd = std::io::stdin().as_raw_fd();
        let mut termios = termios::Termios::from_fd(stdin_fd).unwrap();
        termios.c_lflag &= !(termios::ECHO | termios::ICANON);
        termios::tcsetattr(stdin_fd, termios::os::linux::TCSANOW, &termios).unwrap();

        let post_run_shsh = pre_run_shsh.clone();

        let handler_pre = std::thread::spawn(move || {
            info!("Starting pre run thread...");
            pre_run(pre_run_shsh);
        });

        let handler_post = std::thread::spawn(move || {
            info!("Starting post run thread...");
            post_run(post_run_shsh);
        });

        // when the post or pre thread crashes, join will return an error, and we will exit gracefully
        if let Err(e) = handler_post.join() {
            Error::new(
                format!(
                    "Failed to join post run thread or post run thread crashed: {:?}",
                    e
                ),
                ErrorType::PostRunError,
            )
            .print();
            info!("Exiting gracefully...");
            clear_termios(old_termios);
            std::process::exit(1);
        }
        if let Err(e) = handler_pre.join() {
            Error::new(
                format!(
                    "Failed to join pre run thread or pre run thread crashed: {:?}",
                    e
                ),
                ErrorType::PreRunError,
            )
            .print();
            info!("Exiting gracefully...");
            clear_termios(old_termios);
            std::process::exit(1);
        }
        sig_handle.close();
        sig_thread.join().unwrap();
        clear_termios(old_termios);
    } else {
        info!("Child process started.");
        let err = Command::new("/bin/fish").exec();
        Error::new(
            format!("Failed to execute command: {:?}", err),
            ErrorType::OtherError,
        )
        .print();
    }
}
