use std::ops::RangeInclusive;

pub type CharacterRange = [RangeInclusive<u32>];

// Numbers
pub const DECIMAL_RANGES: &CharacterRange = &[0x30..=0x39];
pub const BINARY_RANGES: &CharacterRange = &[0x30..=0x31];
pub const HEXADECIMAL_RANGES: &CharacterRange = &[0x30..=0x39, 0x41..=0x5a, 0x61..=0x7a];

// Ponctuations
pub const PONCTUATION_RANGES: &CharacterRange = &[
    0x21..=0x21,
    0x23..=0x23,
    0x25..=0x26,
    0x28..=0x2e,
    0x3a..=0x40,
    0x7b..=0x7e,
];

// Variables
pub const BANNED_FIRST_VARIABLE_CHARACTER_RANGES: &CharacterRange = &[0x30..=0x39];
pub const VARIABLE_CHARACTER_RANGES: &CharacterRange = &[
    0x30..=0x39,
    0x41..=0x5a,
    0x5f..=0x5f,
    0x61..=0x7a,
    0xa1..=u32::MAX,
];

// Checker
macro_rules! in_ranges {
    ($r:expr,$c:expr) => {
        $r.iter().any(|_r| _r.contains(&($c as u32)))
    };
}
pub(crate) use in_ranges;
