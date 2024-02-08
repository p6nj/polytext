use std::iter::once;

use super::{Element, Event};
use derive_new::new;
use strum::IntoEnumIterator;

pub(crate) type Path = Vec<(Event, Element)>;

#[derive(new, Debug)]
pub(crate) struct LoopablePath {
    path: Path,
    loopson: usize,
}

pub(crate) fn paths(e: Element) -> Vec<LoopablePath> {
    fn extend(path: Path, next: Element) -> Vec<LoopablePath> {
        if let Some(i) = path
            .iter()
            .map(|(_, e)| e)
            .filter(|e| e == &&next)
            .enumerate()
            .map(|(i, _)| i)
            .next()
        {
            vec![LoopablePath::new(path, i)]
        } else {
            Event::iter()
                .flat_map(|event| {
                    let next = next + event;
                    extend(
                        {
                            let mut path = path.clone();
                            path.extend(once((event, next)));
                            path
                        },
                        next,
                    )
                })
                .collect()
        }
    }
    extend(Path::new(), e)
}
