use futures::{Poll, Async, AsyncSink, StartSend};
use futures::stream::Stream;
use crc8::{self, Crc8};

const SOF: u8 = 0b01111110;
const ESC: u8 = 0b01111101;
const ESC_SOF: u8 = 0b01011110;
const ESC_ESC: u8 = 0b01011101;
static CRC: &Crc8 = &crc8::predefined::MAXIM;

macro_rules! try_poll {
    ($e:expr) => (match $e {
        Ok(::futures::Async::Ready(Some(t))) => t,
        Ok(::futures::Async::Ready(None)) => return Ok(::futures::Async::Ready(None)),
        Ok(::futures::Async::NotReady) => return Ok(::futures::Async::NotReady),
        Err(e) => return Err(From::from(e)),
    })
}

enum State {
    Sof, /// Start-Of-Frame
    AfterSof,
    Payload(usize),
}

pub struct FrameStream<'a, Inner, F> {
    inner: Inner,
    buf: &'a mut [u8],
    decoder: F,
    state: State,
}

impl<'a, Inner: Stream, T, F: FnMut(&[u8]) -> T> FrameStream<'a, Inner, F> {
    pub fn new(inner: Inner, buf: &'a mut [u8], decoder: F) -> Self {
        FrameStream {
            inner: inner,
            buf: buf,
            decoder: decoder,
            state: State::Sof,
        }
    }
}

impl<'a, Inner: Stream<Item=u8>, T, F: FnMut(&[u8]) -> T> Stream for FrameStream<'a, Inner, F> {
    type Item = T;
    type Error = Inner::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            match self.state {
                State::Sof => {
                    loop {
                        if try_poll!(self.inner.poll()) == SOF {
                            self.state = State::AfterSof;
                            break;
                        }
                    }
                }
                State::AfterSof => {
                    loop {
                        let v = try_poll!(self.inner.poll());
                        if v != SOF {
                            self.buf[0] = v;
                            self.state = State::Payload(1);
                            break;
                        }
                    }
                }
                State::Payload(len) => {
                    let v = try_poll!(self.inner.poll());
                    self.buf[len] = v;
                    let len = len + 1;
                    if len == self.buf.len() {
                        self.state = State::Sof;
                        let res = (self.decoder)(self.buf);
                        return Ok(Async::Ready(Some(res)));
                    } else {
                        self.state = State::Payload(len);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream;
    use futures::Async;

    #[test]
    fn empty() {
        run_test(
            &[],
            &[]
        );
    }

    #[test]
    fn one_frame() {
        run_test(
            &[SOF,1,2,3,66],
            &[&[1,2,3]]
        );
    }

    #[test]
    fn two_frames() {
        run_test(
            &[SOF,1,2,3,66,SOF,4,5,6,77],
            &[&[1,2,3], &[4,5,6]]
        );
    }

    #[test]
    fn junk_between_frames() {
        run_test(
            &[9,8,SOF,1,2,3,66,7,6,SOF,4,5,6,77,5,4],
            &[&[1,2,3], &[4,5,6]]
        );
    }

    #[test]
    fn multiple_sofs() {
        run_test(
            &[SOF,SOF,1,2,3,66,SOF,SOF,SOF,4,5,6,77],
            &[&[1,2,3], &[4,5,6]]
        );
    }

    #[test]
    fn sof_in_payload() {
        run_test(
            &[SOF,1,ESC,ESC_SOF,2,66],
            &[&[1,SOF,2]]
        );
    }

    #[test]
    fn esc_in_payload() {
        run_test(
            &[SOF,1,ESC,ESC_ESC,2,66],
            &[&[1,ESC,2]]
        );
    }

    fn run_test(data: &[u8], expected: &[&[u8]]) {
        let inner = stream::iter::<_, _, ()>(data.iter().map(|it| Ok(*it)));
        let mut buf = [0u8; 3];
        let mut expected_iter = expected.iter();
        let mut unit = FrameStream::new(inner, &mut buf, |buf| {
            let expected = expected_iter.next().unwrap();
            assert_eq!(&buf, expected);
            3
        });
        for &expected_frame in expected.iter() {
            let res = unit.poll().unwrap();
            if let Async::Ready(Some(actual_frame)) = res {
                assert_eq!(actual_frame, 3);
            } else {
                panic!("expected {:?}, got {:?}", expected_frame, res);
            }
        }
        assert_eq!(unit.poll().unwrap(), Async::Ready(None));
    }
}
