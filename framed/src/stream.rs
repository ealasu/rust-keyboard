use futures::{Poll, Async};
use futures::stream::Stream;
use constants::{SOF, ESC, ESC_SOF, ESC_ESC, CRC};

#[derive(Debug)]
enum State {
    /// Scanning for Start-Of-Frame byte.
    Sof,
    /// Recieved a Start-Of-Frame byte.
    AfterSof,
    /// Read `len` bytes of the payload. If `esc`, next byte will be un-escaped.
    Payload { len: usize, esc: bool },
}

pub struct FrameStream<'a, Inner, F> {
    inner: Inner,
    buf: &'a mut [u8],
    decoder: F,
    state: State,
}

impl<'a, Inner, Item, F> FrameStream<'a, Inner, F>
where
    Inner: Stream<Item = u8>,
    F: FnMut(&[u8]) -> Item,
{
    pub fn new(inner: Inner, buf: &'a mut [u8], decoder: F) -> Self {
        Self {
            inner: inner,
            buf: buf,
            decoder: decoder,
            state: State::Sof,
        }
    }
}

impl<'a, Inner, Item, F> Stream for FrameStream<'a, Inner, F>
where
    Inner: Stream<Item = u8>,
    F: FnMut(&[u8]) -> Item,
{
    type Item = Item;
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
                            if v == ESC {
                                self.state = State::Payload { len: 0, esc: true };
                            } else {
                                self.buf[0] = v;
                                self.state = State::Payload { len: 1, esc: false };
                            }
                            break;
                        }
                    }
                }
                State::Payload { len, esc } => {
                    let v = try_poll!(self.inner.poll());
                    if v == ESC {
                        self.state = State::Payload {
                            len: len,
                            esc: true,
                        };
                    } else {
                        let v = if esc {
                            match v {
                                ESC_SOF => SOF,
                                ESC_ESC => ESC,
                                _ => v,
                            }
                        } else {
                            v
                        };
                        self.buf[len] = v;
                        let len = len + 1;
                        if len == self.buf.len() {
                            self.state = State::Sof;
                            let payload = &self.buf[..self.buf.len() - 1];
                            let expected_crc = self.buf[self.buf.len() - 1];
                            let actual_crc = CRC.calc_buf(payload);
                            if expected_crc == actual_crc {
                                let res = (self.decoder)(payload);
                                return Ok(Async::Ready(Some(res)));
                            } else {
                                // ignore frame
                                // TODO: debug!("crc: {}, expected: {}", actual_crc, expected_crc);
                            }
                        } else {
                            self.state = State::Payload {
                                len: len,
                                esc: false,
                            };
                        }
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use futures::{stream, Async};

    include!("common_tests.rs");

    #[test]
    fn junk_between_frames() {
        run_test(
            &[&[1, 2, 3], &[4, 5, 6]],
            &[9, 8, SOF, 1, 2, 3, 216, 7, 6, SOF, 4, 5, 6, 188, 5, 4],
        );
    }

    #[test]
    fn corrupt_frame() {
        run_test(
            &[&[4, 5, 6]],
            &[9, 8, SOF, 1, 2, 3, 215, 7, 6, SOF, 4, 5, 6, 188, 5, 4],
        );
    }

    #[test]
    fn multiple_sofs() {
        run_test(
            &[&[1, 2, 3], &[4, 5, 6]],
            &[SOF, SOF, 1, 2, 3, 216, SOF, SOF, SOF, 4, 5, 6, 188],
        );
    }

    #[test]
    fn esc_in_stream() {
        // not supposed to happen, but should be handled
        run_test(&[&[1, 2, 3]], &[SOF, 1, ESC, 2, 3, 216]);
    }

    #[test]
    fn esc_esc_in_stream() {
        // not supposed to happen, but should be handled
        run_test(&[&[1, 2, 3]], &[SOF, 1, ESC, ESC, 2, 3, 216]);
    }

    fn run_test(expected: &[&[u8]], data: &[u8]) {
        let inner = stream::iter_ok::<_, ()>(data.iter().map(|&v| v));
        let mut buf = [0u8; 4];
        let mut expected_iter = expected.iter();
        let mut unit = FrameStream::new(inner, &mut buf, |buf| {
            let expected = expected_iter.next().unwrap();
            assert_eq!(&buf, expected);
            buf.len()
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
