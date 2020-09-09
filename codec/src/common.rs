/// Defines a series of methods to interact with a list of codec descriptors.
pub trait CodecList: Sized {
    /// The type of the structure used to describe a codec.
    type D: ?Sized;

    /// Creates a new codec list.
    fn new() -> Self;

    // TODO more lookup functions
    /// Search by name whether a codec descriptor is in the codec list and
    /// returns it.
    ///
    /// If the requested codec descriptor is not in the list,
    /// `None` is returned.
    fn by_name(&self, name: &str) -> Option<&'static Self::D>;

    /// Appends a codec to the list.
    fn append(&mut self, desc: &'static Self::D);

    /// Creates a new codec list starting from a list of codec descriptors.
    fn from_list(descs: &[&'static Self::D]) -> Self {
        let mut c = Self::new();
        for &desc in descs {
            c.append(desc);
        }

        c
    }
}
