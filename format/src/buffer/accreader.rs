//! AccReader is like a BufReader, but supports partial consumption
//! import new data with `fill_buf`, get the current buffer with
//! `current_slice`, and indicate with `consume` how many bytes
//! were used


use std::io;
use std::io::{BufRead, Read, Result, Seek, SeekFrom};
use std::iter::Iterator;
use std::cmp;
use std::iter;
use crate::buffer::Buffered;

pub struct AccReader<R> {
    inner: R,
    buf: Vec<u8>,
    pos: usize,
    end: usize,
    // position in the stream of the buffer's beginning
    index: usize,
}

impl<R: Read + Seek> AccReader<R> {
    pub fn new(inner: R) -> AccReader<R> {
        AccReader::with_capacity(4096, inner)
    }

    pub fn with_capacity(cap: usize, inner: R) -> AccReader<R> {
        let mut buf = Vec::with_capacity(cap);
        buf.extend(iter::repeat(0).take(cap));
        AccReader {
            inner: inner,
            buf: buf,
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

    /// Unwraps this `AccReader`, returning the underlying reader.
    ///
    /// Note that any leftover data in the internal buffer is lost.
    pub fn into_inner(self) -> R {
        self.inner
    }

    pub fn reset_buffer_position(&mut self) {
        trace!("resetting buffer at pos: {} capacity: {}", self.pos, self.end);
        if self.end - self.pos > 0 {
            for i in 0..(self.end - self.pos) {
                trace!("buf[{}] = buf[{}]", i, self.pos + i);
                self.buf[i] = self.buf[self.pos + i];
            }
        }
        self.end = self.end - self.pos;
        self.pos = 0;
    }

    pub fn current_slice(&self) -> &[u8] {
        trace!("current slice pos: {}, cap: {}", self.pos, self.end);
        &self.buf[self.pos..self.end]
    }

    pub fn capacity(&self) -> usize {
        self.end - self.pos
    }
}

impl<R: Read + Seek + Send> Buffered for AccReader<R> {
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
        trace!("read pos: {} cap: {} buflen: {}", self.pos, self.end, buf.len());
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
                        self.consume(len);
                        self.inner.read(&mut buf[self.end..])
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
            trace!("buffer reset ended");
            let read = self.inner.read(&mut self.buf[self.end..])?;
            self.end += read;
            trace!("new pos: {} and cap: {} -> current: {:?}", self.pos, self.end, &self.buf[self.pos..self.end]);
        }
        Ok(&self.buf[self.pos..self.end])
    }

    fn consume(&mut self, amt: usize) {
        trace!("consumed {} bytes", amt);
        self.pos = cmp::min(self.pos + amt, self.end);
        self.index += amt;
    }
}

impl<R: Read + Seek> Seek for AccReader<R> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
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
                if sz >= 0 && sz as usize <= self.end - self.pos {
                    self.index = sz as usize;
                    self.pos += sz as usize;
                    return Ok(sz as u64);
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
    use std::io::{Cursor, BufRead};
    use crate::buffer::Buffered;

    #[test]
    fn acc_reader_test() {
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
        acc.consume(2);
        acc.grow(4);
        assert_eq!(b"cd", acc.data());
        acc.fill_buf().unwrap();
        assert_eq!(b"cdefghil", acc.data());
    }
}
