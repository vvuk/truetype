use std::io::{Read, Seek, SeekFrom};

use Result;

/// A type that can read.
pub trait Tape: Read + Seek + Sized {
    #[doc(hidden)]
    #[inline]
    fn jump(&mut self, position: u64) -> Result<u64> {
        self.seek(SeekFrom::Start(position))
    }

    #[doc(hidden)]
    #[inline]
    fn peek<T: Value>(&mut self) -> Result<T> {
        self.stay(|tape| Value::read(tape))
    }

    #[doc(hidden)]
    #[inline]
    fn position(&mut self) -> Result<u64> {
        self.seek(SeekFrom::Current(0))
    }

    #[doc(hidden)]
    #[inline(always)]
    fn take<T: Value>(&mut self) -> Result<T> {
        Value::read(self)
    }

    #[doc(hidden)]
    fn stay<F, T>(&mut self, mut body: F) -> Result<T> where F: FnMut(&mut Self) -> Result<T> {
        let position = try!(self.position());
        let result = body(self);
        try!(self.jump(position));
        result
    }
}

/// A type that can be read.
pub trait Value: Sized {
    /// Read a value.
    fn read<T: Tape>(&mut T) -> Result<Self>;
}

impl<T: Read + Seek> Tape for T {}

macro_rules! value {
    ($name:ident, 1) => (impl Value for $name {
        fn read<T: Tape>(tape: &mut T) -> Result<Self> {
            Ok(read!(tape, 1))
        }
    });
    ($name:ident, $size:expr) => (impl Value for $name {
        fn read<T: Tape>(tape: &mut T) -> Result<Self> {
            Ok($name::from_be(read!(tape, $size)))
        }
    });
}

value!(i16, 2);
value!(u16, 2);
value!(u32, 4);
value!(i64, 8);
