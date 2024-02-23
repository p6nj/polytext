use derive_new::new;

use super::{Element, Event};

struct Pixel {
    from: Element,
    path: Vec<Event>,
}

#[derive(new)]
struct PixelPath {
    from: Element,
    path: Vec<Vec<Event>>,
    /// Other paths this one is compatible with.
    /// This field will be empty on the first Pixel.
    // TODO: replace with pointers??
    works_with: Vec<PixelPath>,
}

trait PixelPathFilter<'a>: Iterator<Item = &'a PixelPath> + Sized {
    fn filter_compatible(self, with: PixelPath) -> impl Iterator<Item = &'a PixelPath> {
        self.filter(|p| todo!())
    }
}

impl<'a, T> PixelPathFilter<'a> for T where T: Iterator<Item = &'a PixelPath> + Sized {}

fn test() {
    let a: Vec<&PixelPath> = PixelPath::new(Element::Block, vec![], vec![])
        .works_with
        .iter()
        .filter_compatible(PixelPath::new(Element::Block, vec![], vec![]))
        .collect();
}
