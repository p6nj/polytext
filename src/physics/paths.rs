use super::{Element, Event};
use strum::IntoEnumIterator;

struct Pixel {
    from: Element,
    path: Vec<Event>,
}

#[derive(Clone)]
struct PixelPath {
    from: Element,
    path: Vec<Vec<Event>>,
    /// Other paths this one is compatible with.
    /// This field will be empty on the first Pixel.
    // TODO: replace with pointers??
    compatible: Vec<PixelPath>,
}

fn find_paths(e: &Element, states: &[bool]) -> Vec<Vec<Vec<Event>>> {
    todo!()
}

// this is so weird
trait Captures<U> {}
impl<T: ?Sized, U> Captures<U> for T {}

impl PixelPath {
    fn new(from: Element, path: Vec<Vec<Event>>) -> Self {
        Self {
            from,
            path,
            compatible: vec![],
        }
    }
    fn find<'b, 'a>(
        states: &'b [bool],
        previous: impl Iterator<Item = &'a PixelPath> + 'b + Clone,
    ) -> impl Iterator<Item = Vec<PixelPath>> + Captures<&'b [bool]> {
        Element::iter()
            .filter(|e| e.visible() == states[0])
            .map(move |e| {
                find_paths(&e, states)
                    .iter()
                    .cloned()
                    .map(|p| PixelPath::new(e, p).find_compatible(previous.clone()))
                    .collect()
            })
    }
    fn find_compatible<'a>(self, previous: impl Iterator<Item = &'a PixelPath>) -> Self {
        Self {
            compatible: previous.filter_compatible(&self).cloned().collect(),
            ..self
        }
    }
}

trait PixelPathFilter<'a>: Iterator<Item = &'a PixelPath> + Sized {
    fn filter_compatible(self, with: &PixelPath) -> impl Iterator<Item = &'a PixelPath> {
        self.filter(|&p| todo!())
    }
}

impl<'a, T> PixelPathFilter<'a> for T where T: Iterator<Item = &'a PixelPath> + Sized {}

// fn test() {
//     let a: Vec<&PixelPath> = PixelPath::new(Element::Block, vec![], vec![])
//         .compatible
//         .iter()
//         .filter_compatible(PixelPath::new(Element::Block, vec![], vec![]))
//         .collect();
// }
