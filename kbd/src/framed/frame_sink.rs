use core::marker::PhantomData;
use futures::{Poll, Async, AsyncSink};
use futures::sink::Sink;
use super::{SOF, ESC, ESC_SOF, ESC_ESC, CRC};

pub struct FrameSink<Inner, Item, F> {
    inner: Inner,
    _item: PhantomData<Item>,
    encoder: F,
}

impl<Inner, Item, F> FrameSink<Inner, Item, F>
where Inner: Sink<SinkItem=u8>, F: FnMut(Item, &mut [u8]) {
    pub fn new(inner: Inner, encoder: F) -> Self {
        Self {
            inner: inner,
            encoder: encoder,
            _item: PhantomData,
        }
    }
}

impl<Inner, Item, F> Sink for FrameSink<Inner, Item, F>
where Inner: Sink<SinkItem=u8>, F: FnMut(Item, &mut [u8]) {
    type SinkItem = Item;
    type SinkError = Inner::SinkError;

    fn start_send(&mut self, item: Self::SinkItem) -> Result<AsyncSink<Self::SinkItem>, Self::SinkError> {
        unimplemented!()
    }
    fn poll_complete(&mut self) -> Result<Async<()>, Self::SinkError> {
        unimplemented!()
    }
    fn close(&mut self) -> Result<Async<()>, Self::SinkError> {
        unimplemented!()
    }
}
