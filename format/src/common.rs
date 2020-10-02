use crate::data::rational::Rational64;
use crate::stream::Stream;

/// Global media file information.
#[derive(Debug, Clone)]
pub struct GlobalInfo {
    /// Duration of a media file.
    ///
    /// If `None`, the duration of a media file is not considered.
    pub duration: Option<u64>,
    /// Timebase associated to a media file.
    ///
    /// If `None`, the timebase of a media file is not considered.
    pub timebase: Option<Rational64>,
    /// List of streams present in a media file.
    pub streams: Vec<Stream>,
}

impl GlobalInfo {
    /// Adds a stream to the list of streams present in a media file.
    pub fn add_stream(&mut self, mut st: Stream) -> usize {
        let idx = self.streams.len();

        if st.id < 0 {
            st.id = st.index as isize;
        }
        st.index = idx;

        self.streams.push(st);

        idx
    }
}
