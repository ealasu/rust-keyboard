use core::marker::PhantomData;
use futures::{Poll, Async, AsyncSink, StartSend};
use futures::sink::Sink;
use super::{SOF, ESC, ESC_SOF, ESC_ESC, CRC};

pub struct FrameSink<'a, Inner, Item, F> {
    inner: Inner,
    _item: PhantomData<Item>,
    encoder: F,
    buf: &'a mut [u8],
    pos: Option<usize>,
}

impl<'a, Inner, Item, F> FrameSink<'a, Inner, Item, F>
where Inner: Sink<SinkItem=u8>, F: FnMut(Item, &mut [u8]) {
    pub fn new(inner: Inner, encoder: F, buf: &'a mut [u8]) -> Self {
        Self {
            inner: inner,
            _item: PhantomData,
            encoder: encoder,
            buf: buf,
            pos: None,
        }
    }
}

impl<'a, Inner, Item, F> Sink for FrameSink<'a, Inner, Item, F>
where Inner: Sink<SinkItem=u8>, F: FnMut(Item, &mut [u8]) {
    type SinkItem = Item;
    type SinkError = Inner::SinkError;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        if self.pos.is_none() {
            self.buf[0] = SOF;
            let buf_len = self.buf.len();
            let crc = {
                let payload = &mut self.buf[1..buf_len - 1];
                (self.encoder)(item, payload);
                CRC.calc_buf(payload)
            };
            self.buf[buf_len - 1] = crc;
            self.pos = Some(0);
            //TODO: needed?
            //self.poll_complete()?;
            Ok(AsyncSink::Ready)
        } else {
            Ok(AsyncSink::NotReady(item))
        }
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        loop {
            if let Some(pos) = self.pos {
                self.inner.poll_complete()?;
                if self.inner.start_send(self.buf[pos])? == AsyncSink::Ready {
                    let pos = pos + 1;
                    if pos == self.buf.len() {
                        self.pos = None;
                        return Ok(Async::Ready(()));
                    } else {
                        self.pos = Some(pos);
                    }
                } else {
                    return Ok(Async::NotReady)
                }
            } else {
                return self.inner.poll_complete();
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
    use core::fmt::Debug;
    use futures::sink;
    use futures::Async;
    use super::super::sync_sink::SyncSink;

    #[test]
    fn one_frame() {
        run_test(
            &[&[1,2,3]],
            &[SOF,1,2,3,216]
        );
    }

    fn run_test(frames: &[&[u8]], expected_bytes: &[u8]) {
        let mut expected_bytes = expected_bytes.iter();
        {
            let sink = SyncSink::new(|actual_byte| {
                assert_eq!(actual_byte, *expected_bytes.next().unwrap());
            });
            let mut buf = [0u8; 5];
            let mut unit = FrameSink::new(sink, |item, buf| {
                buf.copy_from_slice(item);
            }, &mut buf);
            for frame in frames.into_iter() {
                assert_eq!(unit.start_send(frame).unwrap(), AsyncSink::Ready);
                assert_eq!(unit.poll_complete().unwrap(), Async::Ready(()));
            }
        }
        assert_eq!(expected_bytes.next(), None);
    }
}
