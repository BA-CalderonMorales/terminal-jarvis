use crate::gates::logic::stream::tee;
use std::io;

/// A scripted reader that replays exact outcomes, so the retry-vs-break
/// decision in `tee` (EINTR must retry; other errors must stop) is
/// witnessed deterministically instead of racing a real pipe.
struct Scripted {
    steps: Vec<io::Result<Vec<u8>>>,
    cursor: usize,
}

impl io::Read for Scripted {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if self.cursor >= self.steps.len() {
            return Ok(0);
        }
        let step = self.steps[self.cursor].as_ref().map(|text| {
            let take = text.len().min(buffer.len());
            buffer[..take].copy_from_slice(&text[..take]);
            take
        });
        match step {
            Ok(take) => {
                self.cursor += 1;
                Ok(take)
            }
            Err(error) => {
                self.cursor += 1;
                Err(io::Error::new(error.kind(), "scripted"))
            }
        }
    }
}

fn interrupted() -> io::Result<Vec<u8>> {
    Err(io::Error::new(io::ErrorKind::Interrupted, "eintr"))
}

fn other() -> io::Result<Vec<u8>> {
    Err(io::Error::other("boom"))
}

#[test]
fn tee_retries_an_interrupted_read_and_keeps_the_payload() {
    let mut reader = Scripted {
        steps: vec![interrupted(), Ok(b"kept".to_vec())],
        cursor: 0,
    };
    assert_eq!(tee(&mut reader, false), "kept");
}

#[test]
fn tee_stops_on_a_real_error_and_forgets_nothing_read() {
    let mut reader = Scripted {
        steps: vec![Ok(b"seen".to_vec()), other(), Ok(b"ghost".to_vec())],
        cursor: 0,
    };
    assert_eq!(tee(&mut reader, false), "seen");
}
