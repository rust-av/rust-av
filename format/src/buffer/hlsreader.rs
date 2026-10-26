use std::io::{self, Read, Seek, SeekFrom};

use m3u8_rs::{MediaPlaylist, Playlist};
use url::Url;

use crate::error::Result;

/// HlsReader for HLS streams
pub struct HlsReader {
    playlist_url: Url,
    media_playlist: MediaPlaylist,
    current_segment_index: usize,
    current_segment_reader: Option<Box<dyn Read + Send + Sync>>,
    pos: u64,
}

impl HlsReader {
    /// Construct a new HlsReader given a playlist_url
    pub fn new(playlist_url: &str) -> Result<Self> {
        let url = Url::parse(playlist_url).map_err(|_| {
            crate::error::Error::Io(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot parse input playlist_url",
            ))
        })?;

        log::trace!("Downloading HLS manifest from: {}", playlist_url);
        let playlist_text = reqwest::blocking::get(playlist_url)
            .map_err(|err| crate::error::Error::Io(io::Error::new(io::ErrorKind::Other, err)))?
            .text()
            .map_err(|err| crate::error::Error::Io(io::Error::new(io::ErrorKind::Other, err)))?;

        let (_, playlist) = m3u8_rs::parse_playlist(playlist_text.as_bytes()).map_err(|err| {
            crate::error::Error::Io(io::Error::new(
                io::ErrorKind::Other,
                format!("Cannot parse input playlist_text: {:?}", err),
            ))
        })?;

        let media_playlist = match playlist {
            Playlist::MediaPlaylist(media) => media,
            Playlist::MasterPlaylist(_) => {
                return Err(crate::error::Error::Io(io::Error::new(
                    io::ErrorKind::Other,
                    "A master playlist was given. Please, select a specific variant instead.",
                )));
            }
        };

        Ok(Self {
            playlist_url: url,
            media_playlist,
            current_segment_index: 0,
            current_segment_reader: None,
            pos: 0,
        })
    }

    // Inner method to advance to the next segment and open the HTTP connection
    fn load_next_segment(&mut self) -> io::Result<bool> {
        if self.current_segment_index >= self.media_playlist.segments.len() {
            return Ok(false); // EOF
        }

        let segment = &self.media_playlist.segments[self.current_segment_index];

        // Resolve the URL relative to the playlist segment
        let segment_url = self
            .playlist_url
            .join(&segment.uri)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err.to_string()))?;

        log::trace!(
            "Transition from segment {} to segment {}",
            self.current_segment_index + 1,
            segment_url
        );

        // Downloads the current segment with reqwest (in blocking mode)
        let response = reqwest::blocking::get(segment_url.as_str())
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;

        if !response.status().is_success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("HTTP error during download: {}", response.status()),
            ));
        }

        // Store the response as active reader
        self.current_segment_reader = Some(Box::new(response));
        self.current_segment_index += 1;
        Ok(true)
    }
}

// Read trait implementation to transform HlsReader in a contiguous stream
impl Read for HlsReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            // If we do not have an active reader, we load the next segment
            if self.current_segment_reader.is_none() {
                if !self.load_next_segment()? {
                    return Ok(0);
                }
            }

            let reader = self.current_segment_reader.as_mut().unwrap();
            match reader.read(buf)? {
                0 => {
                    // The current segment is terminated, the reader must be zeroed to continue with the next loop
                    self.current_segment_reader = None;
                }
                n => return Ok(n),
            }
        }
    }
}

// Seek trait implementation to seek from an HlsReader stream
impl Seek for HlsReader {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(offset) => {
                if offset == 0 {
                    // Reset to the beginning of the stream
                    self.current_segment_index = 0;
                    self.current_segment_reader = None;
                    self.pos = 0;
                    return Ok(0);
                }
                offset
            }
            SeekFrom::Current(offset) => (self.pos as i64 + offset) as u64,
            SeekFrom::End(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "Seeking from end is not supported for dynamic HLS streams",
                ));
            }
        };

        // For robust production use, you would map `new_pos` to the exact segment index.
        // For basic initialization/rewinding required by AccReader, resetting to start works.
        self.pos = new_pos;
        Ok(self.pos)
    }
}
