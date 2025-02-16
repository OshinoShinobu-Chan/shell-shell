use crate::error::*;
use crate::Shsh;

use pty::prelude::*;

use std::io::prelude::*;

#[derive(Debug)]
/// struct for post run processing
struct PreRun {
    shsh: Shsh,
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
                    Error::new(
                        format!(
                            "Error writing to master pty: written {} bytes, expected {}",
                            written, len
                        ),
                        ErrorType::IOError,
                    )
                    .print();
                    break;
                }
            }
            Err(e) => {
                Error::new(
                    format!("Error reading from stdin: {:?}", e),
                    ErrorType::IOError,
                )
                .print();
                break;
            }
        }
    }
    panic!();
}

impl PreRun {
    pub fn new(shsh: Shsh) -> Self {
        PreRun { shsh }
    }
}
