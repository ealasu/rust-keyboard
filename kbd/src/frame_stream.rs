use futures::{Poll, Async, AsyncSink, StartSend};
use futures::stream::Stream;

const SOF: u8 = 0b01111110;
const ESC: u8 = 0b01111101;
const ESC_SOF: u8 = 0b01011110;
const ESC_ESC: u8 = 0b01011101;

pub struct FrameStream<'a, Inner: Stream> {
    inner: Inner,
    buf: &'a mut [u8],
}

impl<'a, Inner: Stream> FrameStream<'a, Inner> {
    pub fn new(inner: Inner, buf: &'a mut [u8]) -> Self {
        FrameStream {
            inner: inner,
            buf: buf,
        }
    }
}

impl<'a, Inner: Stream> Stream for FrameStream<'a, Inner> {
    type Item = &'a [u8];
    type Error = Inner::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        unimplemented!()
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
        let inner = stream::iter::<_, _, ()>(data.iter().map(|it| Ok(it)));
        let mut buf = [0u8; 3];
        let mut unit = FrameStream::new(inner, &mut buf);
        for &expected_frame in expected.iter() {
            let res = unit.poll().unwrap();
            if let Async::Ready(Some(actual_frame)) = res {
                assert_eq!(actual_frame, expected_frame);
            } else {
                panic!("expected {:?}, got {:?}", expected_frame, res);
            }
        }
        assert_eq!(unit.poll().unwrap(), Async::Ready(None));
    }
}
