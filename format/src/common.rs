use data::rational::Rational64;
use stream::Stream;

#[derive(Debug, Clone)]
pub struct GlobalInfo {
    pub duration: Option<u64>,
    pub timebase: Option<Rational64>,
    pub streams: Vec<Stream>,
}

impl GlobalInfo {
    pub fn add_stream(&mut self, mut st: Stream) {
        st.index = self.streams.len();
        if st.id < 0 {
            st.id = st.index as isize;
        }
        self.streams.push(st);
    }
}
