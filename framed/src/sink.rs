use core::marker::PhantomData;
use futures::{Poll, Async, AsyncSink, StartSend};
use futures::sink::Sink;
use constants::{SOF, ESC, ESC_SOF, ESC_ESC, CRC};

enum State {
    /// Nothing to write
    Empty,
    /// `pos` bytes remaining to be written
    Data { pos: usize },
    /// Just sent ESC, so next byte will be escaped (either ESC_SOF or ESC_ESC)
    AfterEsc { escaped: u8, pos: usize },
}

pub struct FrameSink<'a, Inner, Item, F> {
    inner: Inner,
    _item: PhantomData<Item>,
    encoder: F,
    buf: &'a mut [u8],
    state: State,
}

impl<'a, Inner, Item, F> FrameSink<'a, Inner, Item, F>
where
    Inner: Sink<SinkItem = u8>,
    F: FnMut(Item, &mut [u8]),
{
    pub fn new(inner: Inner, encoder: F, buf: &'a mut [u8]) -> Self {
        Self {
            inner: inner,
            _item: PhantomData,
            encoder: encoder,
            buf: buf,
            state: State::Empty,
        }
    }
}

impl<'a, Inner, Item, F> Sink for FrameSink<'a, Inner, Item, F>
where
    Inner: Sink<SinkItem = u8>,
    F: FnMut(Item, &mut [u8]),
{
    type SinkItem = Item;
    type SinkError = Inner::SinkError;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        match self.state {
            State::Empty => {
                self.buf[0] = SOF;
                let buf_len = self.buf.len();
                let crc = {
                    let payload = &mut self.buf[1..buf_len - 1];
                    (self.encoder)(item, payload);
                    CRC.calc_buf(payload)
                };
                self.buf[buf_len - 1] = crc;
                self.state = State::Data { pos: 0 };
                //TODO: needed?
                //self.poll_complete()?;
                Ok(AsyncSink::Ready)
            }
            _ => Ok(AsyncSink::NotReady(item)),
        }
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        loop {
            match self.state {
                State::AfterEsc { pos, escaped } => {
                    try_send!(self.inner.start_send(escaped));
                    self.state = State::Data { pos: pos + 1 };
                }
                State::Data { pos } => {
                    if pos == self.buf.len() {
                        self.state = State::Empty;
                        return Ok(Async::Ready(()));
                    }
                    self.inner.poll_complete()?;
                    let v = self.buf[pos];
                    if pos > 0 && (v == SOF || v == ESC) {
                        try_send!(self.inner.start_send(ESC));
                        self.state = State::AfterEsc {
                            pos: pos,
                            escaped: match v {
                                SOF => ESC_SOF,
                                ESC => ESC_ESC,
                                _ => unreachable!(),
                            },
                        };
                    } else {
                        try_send!(self.inner.start_send(v));
                        self.state = State::Data { pos: pos + 1 };
                    }
                }
                State::Empty => return self.inner.poll_complete(),
            }
        }
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::Async;
    use sync_sink::SyncSink;

    include!("common_tests.rs");

    fn run_test(frames: &[&[u8]], expected_bytes: &[u8]) {
        let mut expected_bytes = expected_bytes.iter();
        {
            let sink = SyncSink::new(|actual_byte| {
                assert_eq!(actual_byte, *expected_bytes.next().unwrap());
            });
            let mut buf = [0u8; 5];
            let mut unit =
                FrameSink::new(sink, |item, buf| { buf.copy_from_slice(item); }, &mut buf);
            for frame in frames.into_iter() {
                assert_eq!(unit.start_send(frame).unwrap(), AsyncSink::Ready);
                assert_eq!(unit.poll_complete().unwrap(), Async::Ready(()));
            }
        }
        assert_eq!(expected_bytes.next(), None);
    }
}
