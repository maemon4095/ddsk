mod ddsk;
mod iterator_middleware;
mod iterator_terminate_with;

use ddsk::{DdskLoveInjector, DdskSequence, DdskWithLove};
use iterator_middleware::IteratorWithMiddlewareExtension;
use iterator_terminate_with::IteratorTerminateWithExtension;

fn main() {
    DdskSequence::new(rand::thread_rng())
        .with_middleware(DdskLoveInjector::with_repetition_threshold(3))
        .terminate_with(DdskWithLove::LoveInjection)
        .for_each(|token| print!("{}", token));

    println!()
}
