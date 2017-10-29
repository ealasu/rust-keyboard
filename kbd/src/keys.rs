use core::mem;
use bit_field::BitField;
use byteorder::{ByteOrder, LittleEndian as LE};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Keys(u32);

pub type Index = u8;

impl Keys {
    /// Create a `Keys` with all keys off
    #[inline]
    pub fn none() -> Self {
        Keys(0)
    }

    /// Total number of keys
    #[inline]
    pub fn len() -> Index {
        (mem::size_of::<Self>() * 8) as u8
    }

    #[inline]
    pub fn read(buf: &[u8]) -> Self {
        Keys(LE::read_u32(buf))
    }

    #[inline]
    pub fn write(&self, buf: &mut [u8]) {
        LE::write_u32(buf, self.0);
    }

    #[inline]
    pub fn key(&self, index: Index) -> bool {
        self.0.get_bit(index)
    }

    #[inline]
    pub fn set_key(&mut self, index: Index, state: bool) {
        self.0.set_bit(index, state);
    }

    #[inline]
    pub fn on<'a>(&'a self) -> impl Iterator<Item=Index> + 'a {
        (0..Self::len()).filter(move |&i| self.key(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;

    #[test]
    fn len() {
        assert_eq!(Keys::len(), 32);
    }

    #[test]
    fn none() {
        let mut keys = Keys::none();
        for i in 0..Keys::len() {
            assert_eq!(keys.key(i), false);
        }
        assert_eq!(keys.on().count(), 0);
    }

    #[test]
    fn read_write() {
        let mut keys = Keys::none();
        let on = [0u8, 2, 5, 19, 30];
        for &i in on.iter() {
            keys.set_key(i, true);
        }
        let mut buf = [0u8; 4];
        keys.write(&mut buf);
        let keys = Keys::read(&buf);
        for i in 0..Keys::len() {
            let should_be_on = on.iter().any(|&j| j == i);
            assert_eq!(keys.key(i), should_be_on);
        }
        assert_eq!(keys.on().count(), on.len());
        for (actual, &expected) in keys.on().zip(on.iter()) {
            assert_eq!(actual, expected);
        }
    }
}
