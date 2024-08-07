mod detector;
mod love_injector;
mod sequence;

pub use detector::DdskDetector;
pub use love_injector::DdskLoveInjector;
pub use sequence::DdskSequence;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ddsk {
    DD,
    SK,
}

impl Display for Ddsk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ddsk::DD => f.write_str("ドド"),
            Ddsk::SK => f.write_str("スコ"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DdskWithLove {
    DD,
    SK,
    LoveInjection,
}

impl Display for DdskWithLove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DdskWithLove::DD => f.write_str("ドド"),
            DdskWithLove::SK => f.write_str("スコ"),
            DdskWithLove::LoveInjection => f.write_str("ラブ注入♡"),
        }
    }
}

impl From<Ddsk> for DdskWithLove {
    fn from(value: Ddsk) -> Self {
        match value {
            Ddsk::DD => DdskWithLove::DD,
            Ddsk::SK => DdskWithLove::SK,
        }
    }
}
