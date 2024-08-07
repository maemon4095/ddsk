use crate::iterator_middleware::IteratorMiddleware;

use super::{Ddsk, DdskDetector, DdskWithLove};

pub struct DdskLoveInjector {
    detector: DdskDetector,
}

impl DdskLoveInjector {
    pub fn with_repetition_threshold(repetition_threshold: usize) -> Self {
        Self {
            detector: DdskDetector::with_repetition_threshold(repetition_threshold),
        }
    }
}

impl IteratorMiddleware for DdskLoveInjector {
    type InputItem = Ddsk;
    type OutputItem = DdskWithLove;

    fn next(
        &mut self,
        mut iter: impl Iterator<Item = Self::InputItem>,
    ) -> Option<Self::OutputItem> {
        if self.detector.is_ddsk_sequence() {
            self.detector.reset();
            return Some(DdskWithLove::LoveInjection);
        }

        let Some(token) = iter.next() else {
            return None;
        };
        self.detector.next(token);
        Some(token.into())
    }
}
