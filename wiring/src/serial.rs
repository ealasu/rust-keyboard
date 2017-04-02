use futures::{Poll, Async, AsyncSink, StartSend};
use futures::stream::Stream;
use futures::sink::Sink;

pub struct Serial;

impl Stream for Serial {
    type Item = u8;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        #[allow(unused_unsafe)]
        let v = unsafe { sys::serial_read() };
        if v < 0 {
            Ok(Async::NotReady)
        } else {
            Ok(Async::Ready(Some(v as u8)))
        }
    }
}

impl Sink for Serial {
    type SinkItem = u8;
    type SinkError = ();

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        // TODO: make async
        unsafe {
            sys::serial_write(item);
        }
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::Ready(()))
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        self.poll_complete()
    }
}

mod sys {
    extern {
        pub fn serial_write(b: u8) -> u8;
    }

    #[cfg(target_arch = "msp430")]
    pub mod platform {
        extern {
            pub fn serial_read() -> i16;
        }
    }

    #[cfg(target_arch = "arm")]
    pub mod platform {
        extern {
            pub fn serial_read() -> i32;
        }
    }

    #[cfg(not(any(target_arch = "arm", target_arch = "msp430")))]
    pub mod platform {
        pub fn serial_read() -> i32 { unimplemented!() }
    }

    pub use self::platform::*;
}
