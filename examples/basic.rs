extern crate xxshim;
use xxshim::*;

exex!(
    "My First xxshim ExEx!",
    async |x| { println!("{x:?}") },
    async |x, y| { println!("{x:?} {y:?}") },
    async |x| { println!("{x:?}") }
);
