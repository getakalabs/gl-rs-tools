use std::io::{self, Cursor, Read, Seek, SeekFrom};

#[derive(Clone)]
pub struct CustomCursor(Cursor<Vec<u8>>);

impl Read for CustomCursor {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl Seek for CustomCursor {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.0.seek(pos)
    }
}

impl CustomCursor {
    pub fn new(value: &[u8]) -> Self {
        Self(Cursor::new(value.to_owned()))
    }
}
