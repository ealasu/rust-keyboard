macro_rules! try_poll {
    ($e:expr) => (match $e {
        Ok(::futures::Async::Ready(Some(t))) => t,
        Ok(::futures::Async::Ready(None)) => return Ok(::futures::Async::Ready(None)),
        Ok(::futures::Async::NotReady) => return Ok(::futures::Async::NotReady),
        Err(e) => return Err(From::from(e)),
    })
}

macro_rules! try_send {
    ($e:expr) => (match $e {
        Ok(::futures::AsyncSink::Ready) => {},
        Ok(::futures::AsyncSink::NotReady(_)) => return Ok(::futures::Async::NotReady),
        Err(e) => return Err(From::from(e)),
    })
}
