use super::{Element, Event};

struct Pixel {
    from: Element,
    path: Vec<Event>,
}

struct PixelPath {
    /// The asked states this path goes though
    states: Vec<bool>,
    from: Element,
    path: Vec<Vec<Event>>,
    /// Other paths this one is compatible with.
    /// This field will be empty on the first Pixel.
    // TODO: replace with pointers??
    works_with: Vec<PixelPath>,
}

impl PixelPath {
    fn filter_compatible<'a>(&self, paths: &'a Vec<PixelPath>) -> Vec<&'a PixelPath> {
        paths.iter().filter(|p| todo!()).collect()
    }
}
