use core::marker::PhantomData;
use futures::{Poll, Async, AsyncSink, StartSend};
use futures::sink::Sink;
use super::{SOF, ESC, ESC_SOF, ESC_ESC, CRC};

pub struct FrameSink<'a, Inner, Item, F> {
    inner: Inner,
    _item: PhantomData<Item>,
    encoder: F,
    buf: &'a mut [u8],
}

impl<'a, Inner, Item, F> FrameSink<'a, Inner, Item, F>
where Inner: Sink<SinkItem=u8>, F: FnMut(Item, &mut [u8]) {
    pub fn new(inner: Inner, encoder: F, buf: &'a mut [u8]) -> Self {
        Self {
            inner: inner,
            encoder: encoder,
            buf: buf,
            _item: PhantomData,
        }
    }
}

impl<'a, Inner, Item, F> Sink for FrameSink<'a, Inner, Item, F>
where Inner: Sink<SinkItem=u8>, F: FnMut(Item, &mut [u8]) {
    type SinkItem = Item;
    type SinkError = Inner::SinkError;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        unimplemented!()
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        unimplemented!()
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
    fn test() {
        run_test(
            [1,2].into_iter(),
            &[3,4]
        );
    }

    fn run_test<Item, I>(items: I, expected_data: &[u8])
    where Item: Debug + PartialEq, I: IntoIterator<Item=Item> {
        let mut expected = expected_data.iter();
        let sink = SyncSink::new(|item| {
            assert_eq!(item, *expected.next().unwrap());
        });
        let mut buf = [0u8; 4];
        let mut unit = FrameSink::new(sink, |item, buf| {
            // TODO: copy
        }, &mut buf);
        for item in items.into_iter() {
            assert_eq!(unit.start_send(item).unwrap(), AsyncSink::Ready);
            //unit.poll_complete().unwrap();
        }
    }
}
