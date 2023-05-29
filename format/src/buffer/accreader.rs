//! `AccReader` is like a `BufReader`, but supports partial consumption.
//!
//! Import new data with `fill_buf`, get the current buffer with
//! `current_slice`, and indicate through the `consume` method how many bytes
//! were used.

use crate::buffer::Buffered;
use std::cmp;
use std::io;
use std::io::{BufRead, Read, Result, Seek, SeekFrom};
use std::iter;
use std::iter::Iterator;

/// Partial consumption buffer for any reader.
pub struct AccReader<R> {
    inner: R,
    buf: Vec<u8>,
    pos: usize,
    end: usize,
    // Position in the stream of the buffer's beginning
    index: usize,
}

impl<R: Read + Seek> AccReader<R> {
    /// Creates a new `AccReader` instance.
    pub fn new(inner: R) -> AccReader<R> {
        AccReader::with_capacity(4096, inner)
    }

    /// Creates a new `AccReader` instance of a determined capacity
    /// for a reader.
    pub fn with_capacity(cap: usize, inner: R) -> AccReader<R> {
        AccReader {
            inner,
            buf: iter::repeat(0).take(cap).collect::<Vec<_>>(),
            pos: 0,
            end: 0,
            index: 0,
        }
    }

    /// Gets a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Gets a mutable reference to the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Unwraps the `AccReader`, returning the underlying reader.
    ///
    /// Note that any leftover data in the internal buffer is lost.
    pub fn into_inner(self) -> R {
        self.inner
    }

    /// Resets the buffer to the current position.
    ///
    /// All data before the current position is lost.
    pub fn reset_buffer_position(&mut self) {
        log::trace!(
            "resetting buffer at pos: {} capacity: {}",
            self.pos,
            self.end
        );
        if self.end - self.pos > 0 {
            log::trace!("copying {} to beginning of buffer", self.end - self.pos);
            self.buf.copy_within(self.pos..self.end, 0);
        }
        self.end -= self.pos;
        self.pos = 0;
    }

    /// Returns buffer data.
    pub fn current_slice(&self) -> &[u8] {
        log::trace!("current slice pos: {}, cap: {}", self.pos, self.end);
        &self.buf[self.pos..self.end]
    }

    /// Returns buffer capacity.
    pub fn capacity(&self) -> usize {
        self.end - self.pos
    }
}

impl<R: Read + Seek + Send + Sync> Buffered for AccReader<R> {
    fn data(&self) -> &[u8] {
        &self.buf[self.pos..self.end]
    }
    fn grow(&mut self, len: usize) {
        let l = self.buf.len() + len;
        self.buf.resize(l, 0);
    }
}

impl<R: Read + Seek> Read for AccReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        log::trace!(
            "read pos: {} cap: {} buflen: {}",
            self.pos,
            self.end,
            buf.len()
        );
        if buf.len() < self.end - self.pos {
            match (&self.buf[self.pos..(self.pos + buf.len())]).read(buf) {
                Ok(len) => {
                    self.consume(len);
                    Ok(len)
                }
                Err(e) => Err(e),
            }
        } else {
            // If we don't have any buffered data and we're doing a massive read
            // (larger than our internal buffer), bypass our internal buffer
            // entirely.
            if buf.len() > self.buf.len() {
                match (&self.buf[self.pos..self.end]).read(buf) {
                    Ok(len) => {
                        let total_len = self.inner.read(&mut buf[(self.end - self.pos)..])? + len;

                        self.consume(total_len);
                        self.reset_buffer_position();

                        Ok(total_len)
                    }
                    Err(e) => Err(e),
                }
            } else {
                let nread = {
                    let mut rem = self.fill_buf()?;
                    rem.read(buf)?
                };
                self.consume(nread);
                Ok(nread)
            }
        }
    }
}

impl<R: Read + Seek> BufRead for AccReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        // trace!("fillbuf current: {:?}", str::from_utf8(&self.buf[self.pos..self.end]).unwrap());
        if self.pos != 0 || self.end != self.buf.len() {
            self.reset_buffer_position();
            log::trace!("buffer reset ended");
            let read = self.inner.read(&mut self.buf[self.end..])?;
            self.end += read;
            log::trace!(
                "new pos: {} and cap: {} -> current: {:?}",
                self.pos,
                self.end,
                &self.buf[self.pos..self.end]
            );
        }
        Ok(&self.buf[self.pos..self.end])
    }

    fn consume(&mut self, amt: usize) {
        log::trace!("consumed {} bytes", amt);
        self.pos = cmp::min(self.pos + amt, self.end);
        self.index += amt;
    }
}

impl<R: Read + Seek> Seek for AccReader<R> {
    fn seek(&mut self, mut pos: SeekFrom) -> Result<u64> {
        match pos {
            SeekFrom::Start(sz) => {
                let mv = sz as usize;
                if mv >= self.index && mv < self.index + self.end - self.pos {
                    self.pos += mv - self.index;
                    self.index = mv;

                    return Ok(mv as u64);
                }
            }
            SeekFrom::End(_) => {}
            SeekFrom::Current(sz) => {
                let remaining = self.end - self.pos;

                if sz >= 0 {
                    if sz as usize <= remaining {
                        self.index += sz as usize;
                        self.pos += sz as usize;
                        return Ok(self.index as u64);
                    } else {
                        pos = SeekFrom::Current(sz - remaining as i64);
                    }
                }
            }
        };

        match self.inner.seek(pos) {
            Ok(sz) => {
                self.index = sz as usize;
                self.pos = 0;
                self.end = 0;
                self.fill_buf()?;
                Ok(sz)
            }
            Err(e) => Err(e),
        }
    }
}
// impl<R> fmt::Debug for AccReader<R> where R: fmt::Debug {
// fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
// fmt.debug_struct("AccReader")
// .field("reader", &self.inner)
// .field("buffer", &format_args!("{}/{}", self.end - self.pos, self.buf.len()))
// .finish()
// }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffered;
    use std::io::{BufRead, Cursor};
    use std::ops::Range;

    fn assert_read_acc(bytes: &[u8], capacity: usize, ranges: &[Range<usize>]) {
        let c = Cursor::new(bytes);
        let mut vec = vec![0u8; bytes.len()];
        let mut acc = AccReader::with_capacity(capacity, c);

        for r in ranges {
            acc.read_exact(&mut vec[r.clone()]).unwrap();
        }

        assert_eq!(bytes, &vec);
    }

    #[test]
    fn same_capacity_full_read() {
        let buf = (0u8..).take(20).collect::<Vec<u8>>();

        assert_read_acc(&buf, 20, &[0..buf.len()]);
    }

    #[test]
    fn split_read_1() {
        let buf = (0u8..).take(31).collect::<Vec<u8>>();

        assert_read_acc(&buf, 20, &[0..10, 10..buf.len()]);
    }

    #[test]
    fn split_read_2() {
        let buf = (0u8..).take(31).collect::<Vec<u8>>();

        assert_read_acc(&buf, 20, &[0..3, 3..buf.len()]);
    }

    #[test]
    fn seek_within_capacity() {
        let buf = (0u8..).take(30).collect::<Vec<u8>>();
        let c = Cursor::new(&buf[..]);

        let mut acc = AccReader::with_capacity(15, c);

        assert_eq!(5, acc.seek(SeekFrom::Current(5)).unwrap());
        assert_eq!(10, acc.seek(SeekFrom::Current(5)).unwrap());
        assert_eq!(15, acc.seek(SeekFrom::Current(5)).unwrap());
    }

    #[test]
    fn seek_across_capacity() {
        let buf = (0u8..).take(30).collect::<Vec<u8>>();
        let c = Cursor::new(&buf[..]);

        let mut acc = AccReader::with_capacity(15, c);

        assert_eq!(5, acc.seek(SeekFrom::Current(5)).unwrap());
        assert_eq!(20, acc.seek(SeekFrom::Current(15)).unwrap());
        assert_eq!(5, acc.seek(SeekFrom::Start(5)).unwrap());
    }

    #[test]
    fn seek_and_read() {
        let len = 30;
        let buf = (0u8..).take(len).collect::<Vec<u8>>();
        let c = Cursor::new(&buf[..]);

        let mut acc = AccReader::with_capacity(5, c);

        assert_eq!(0, acc.stream_position().unwrap());

        for i in 0..30 {
            assert_eq!(i, read_byte(&mut acc).unwrap() as u64);
            assert_eq!(i + 1, acc.stream_position().unwrap());
        }
    }

    fn read_byte<R: Read + Seek>(acc: &mut AccReader<R>) -> io::Result<u8> {
        let mut byte = [0];
        acc.read_exact(&mut byte)?;
        Ok(byte[0])
    }

    #[test]
    fn reader_test() {
        let buf = b"AAAA\nAAAB\nAAACAAADAAAEAAAF\ndabcdEEEE";
        let c = Cursor::new(&buf[..]);

        let acc = AccReader::with_capacity(20, c);

        assert_eq!(4, acc.lines().count());
    }

    #[test]
    fn grow() {
        let buf = b"abcdefghilmnopqrst";
        let c = Cursor::new(&buf[..]);

        let mut acc = AccReader::with_capacity(4, c);
        acc.fill_buf().unwrap();
        assert_eq!(b"abcd", acc.data());
        acc.consume(2);
        assert_eq!(b"cd", acc.data());
        acc.grow(4);
        assert_eq!(b"cd", acc.data());
        acc.fill_buf().unwrap();
        assert_eq!(b"cdefghil", acc.data());
    }
}
