use crate::Shsh;
use crate::BUF_SIZE;
use pty::prelude::*;
use std::io::prelude::*;

#[derive(Debug)]
/// struct for post run processing
struct PreRun {
    shsh: Shsh,
}

#[derive(Debug)]
/// errors that can occur during post run processing
enum PreRunError {
    IOError(std::io::Error),
    /// The input exceeds the buffer size. This error is not necessarily an error,
    /// you can call `readline` again to read the rest of the input
    BufferOverflow,
}

fn trivial_write(master: &mut Master, buf: &[u8]) -> usize {
    master.write(buf).unwrap()
}

pub fn pre_run(shsh: Shsh) {
    let mut pre_run = PreRun::new(shsh);
    loop {
        let read_n = std::io::stdin().read(&mut pre_run.shsh.buffer);
        match read_n {
            Ok(0) => break,
            Ok(n) => {
                let mut len = n;
                let written = trivial_write(&mut pre_run.shsh.master, &pre_run.shsh.buffer[..len]);
                if written != len {
                    eprintln!("Error writing to master pty");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {:?}", e);
                break;
            }
        }
    }
}

impl PreRun {
    pub fn new(shsh: Shsh) -> Self {
        PreRun { shsh }
    }
}
