use super::Ddsk;

pub struct DdskSequence<R: rand::Rng> {
    rng: R,
}

impl<R: rand::Rng> DdskSequence<R> {
    pub fn new(rng: R) -> Self {
        Self { rng }
    }
}

impl<R: rand::Rng> Iterator for DdskSequence<R> {
    type Item = Ddsk;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rng.gen_bool(0.5) {
            Some(Ddsk::DD)
        } else {
            Some(Ddsk::SK)
        }
    }
}
