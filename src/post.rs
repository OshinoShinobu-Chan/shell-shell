use crate::error::*;
use crate::Shsh;
use crate::BUF_SIZE;

use std::io::prelude::*;

#[derive(Debug)]
/// struct for post run processing
struct PostRun {
    shsh: Shsh,
}

pub fn post_run(shsh: Shsh) {
    let mut post_run = PostRun::new(shsh);
    loop {
        let read_n = post_run.shsh.master.read(&mut post_run.shsh.buffer);
        post_run.shsh.master.flush().unwrap();
        match read_n {
            Ok(0) => continue,
            Ok(_) => {
                std::io::stdout().write_all(&post_run.shsh.buffer).unwrap();
                std::io::stdout().flush().unwrap();
                post_run.shsh.buffer = [0; BUF_SIZE];
            }
            Err(e) => {
                Error::new(
                    format!("Error reading from master pty: {:?}", e),
                    ErrorType::IOError,
                )
                .print();
                break;
            }
        }
    }
    panic!();
}

impl PostRun {
    pub fn new(shsh: Shsh) -> Self {
        PostRun { shsh }
    }
}
