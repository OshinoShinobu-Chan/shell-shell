use crate::Shsh;
use crate::BUF_SIZE;
use std::io::prelude::*;

#[derive(Debug)]
/// struct for post run processing
struct PostRun {
    shsh: Shsh,
}

#[derive(Debug)]
/// errors that can occur during post run processing
enum PostRunError {
    IOError(std::io::Error),
    /// The input exceeds the buffer size. This error is not necessarily an error,
    /// you can call `readline` again to read the rest of the input
    BufferOverflow,
}

pub fn post_run(shsh: Shsh) {
    let mut post_run = PostRun::new(shsh);
    loop {
        let read_n = post_run.shsh.master.read(&mut post_run.shsh.buffer);
        post_run.shsh.master.flush().unwrap();
        match read_n {
            Ok(0) => break,
            Ok(_) => {
                std::io::stdout().write_all(&post_run.shsh.buffer).unwrap();
                std::io::stdout().flush().unwrap();
                post_run.shsh.buffer = [0; BUF_SIZE];
            }
            Err(e) => {
                eprintln!("Error writing to stdout: {:?}", e);
                break;
            }
        }
    }
}

impl PostRun {
    pub fn new(shsh: Shsh) -> Self {
        PostRun { shsh }
    }

    /// read a from the master pty unless the input exceeds the buffer size
    /// # Returns
    /// * `Result<usize, PostRunError>` - the number of bytes read
    pub fn read(&mut self) -> Result<usize, PostRunError> {
        let mut i = 0;
        loop {
            let byte = self.shsh.master.read(&mut self.shsh.buffer[i..i + 1]);
            match byte {
                Ok(0) => break,
                Ok(n) => {
                    i += n;
                    if i == BUF_SIZE {
                        return Err(PostRunError::BufferOverflow);
                    }
                    if self.shsh.buffer[i - 1] == b'\n' {
                        break;
                    }
                }
                Err(e) => return Err(PostRunError::IOError(e)),
            }
        }
        Ok(i)
    }
}
