use core::marker::PhantomData;
use futures::{Poll, Async, AsyncSink, StartSend};
use futures::sink::Sink;

pub struct SyncSink<Item, F> {
    _item: PhantomData<Item>,
    f: F,
}

impl<Item,F> SyncSink<Item,F>
where F: FnMut(Item) {
    pub fn new(f: F) -> Self {
        Self {
            _item: PhantomData,
            f: f,
        }
    }
}

impl<Item,F> Sink for SyncSink<Item,F>
where F: FnMut(Item) {
    type SinkItem = Item;
    type SinkError = ();

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        (self.f)(item);
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::Ready(()))
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::Ready(()))
    }
}
