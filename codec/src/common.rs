/*
pub trait Descriptor {
    type Description;
    type Described;

    fn create(&self) -> Self::Described;
    fn describe<'a>(&'a self) -> &'a Self::Description;
}
*/

pub trait CodecList: Sized {
    type D: ?Sized;

    fn new() -> Self;

    // TODO more lookup functions
    fn by_name(&self, name: &str) -> Option<&'static Self::D>;

    fn append(&mut self, desc: &'static Self::D);

    fn from_list(descs: &[&'static Self::D]) -> Self {
        let mut c = Self::new();
        for &desc in descs {
            c.append(desc);
        }

        c
    }
}
