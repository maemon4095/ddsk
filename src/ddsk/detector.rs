use super::Ddsk;

enum SingleDdskDetector {
    Initial,
    DD,
    SK0,
    SK1,
    Final,
}

impl SingleDdskDetector {
    fn new() -> Self {
        Self::Initial
    }

    fn is_ddsk_sequence(&self) -> bool {
        match self {
            SingleDdskDetector::Final => true,
            _ => false,
        }
    }

    fn next(&mut self, ddsk: Ddsk) -> bool {
        match self {
            SingleDdskDetector::Initial | SingleDdskDetector::Final => match ddsk {
                Ddsk::DD => {
                    *self = Self::DD;
                    true
                }
                Ddsk::SK => {
                    *self = Self::Initial;
                    false
                }
            },
            SingleDdskDetector::DD => match ddsk {
                Ddsk::DD => {
                    *self = Self::DD;
                    false
                }
                Ddsk::SK => {
                    *self = Self::SK0;
                    true
                }
            },
            SingleDdskDetector::SK0 => match ddsk {
                Ddsk::DD => {
                    *self = Self::DD;
                    false
                }
                Ddsk::SK => {
                    *self = Self::SK1;
                    true
                }
            },
            SingleDdskDetector::SK1 => match ddsk {
                Ddsk::DD => {
                    *self = Self::DD;
                    false
                }
                Ddsk::SK => {
                    *self = Self::Final;
                    true
                }
            },
        }
    }

    fn reset(&mut self) {
        *self = SingleDdskDetector::Initial;
    }
}

pub struct DdskDetector {
    repetition_threshold: usize,
    current_repetition: usize,
    detector: SingleDdskDetector,
}

impl DdskDetector {
    pub fn with_repetition_threshold(repetition_threshold: usize) -> Self {
        Self {
            repetition_threshold,
            current_repetition: 0,
            detector: SingleDdskDetector::new(),
        }
    }

    pub fn is_ddsk_sequence(&self) -> bool {
        self.current_repetition == self.repetition_threshold
    }

    pub fn next(&mut self, ddsk: Ddsk) {
        if !self.detector.next(ddsk) {
            self.current_repetition = 0;
        }
        if self.detector.is_ddsk_sequence() {
            self.current_repetition += 1;
        }
    }

    pub fn reset(&mut self) {
        self.current_repetition = 0;
        self.detector.reset();
    }
}
