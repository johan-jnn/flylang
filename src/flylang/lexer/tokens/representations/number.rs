use std::mem::take;

use crate::flylang::{
    lexer::ranges::{BINARY_RANGES, CharacterRange, DECIMAL_RANGES, HEXADECIMAL_RANGES, in_ranges},
    module::slice::LangModuleSlice,
};

#[derive(Debug)]
pub enum NumberRepresentationBases {
    Binary,
    Decimal,
    Hexadecimal,
}
impl NumberRepresentationBases {
    pub fn multiplier(&self, digit_index: usize) -> u64 {
        let pow_with = |base: u64| base.pow(digit_index as u32);
        match self {
            Self::Binary => pow_with(2),
            Self::Decimal => pow_with(10),
            Self::Hexadecimal => pow_with(16),
        }
    }

    pub fn range(&self) -> &'static CharacterRange {
        match self {
            Self::Binary => BINARY_RANGES,
            Self::Decimal => DECIMAL_RANGES,
            Self::Hexadecimal => HEXADECIMAL_RANGES,
        }
    }

    pub fn convert_digit(&self, digit: char) -> Option<u64> {
        if !in_ranges!(self.range(), digit) {
            return None;
        };

        Some(match digit {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'a' => 10,
            'b' => 11,
            'c' => 12,
            'd' => 13,
            'e' => 14,
            'f' => 15,
            _ => panic!(),
        })
    }
}

pub struct NumberRepresentation {
    pub negative: bool,
    pub integer: u64,
    pub decimal: Option<u64>,
    pub represented_as: NumberRepresentationBases,
}
impl Into<f64> for NumberRepresentation {
    fn into(self) -> f64 {
        let mut num = 0f64;
        num += self.integer as f64;

        num += if let Some(dec) = self.decimal {
            (dec as f64) / 10f64.powf(dec.to_string().len() as f64)
        } else {
            0f64
        };

        if self.negative {
            num *= -1f64;
        }
        num
    }
}

impl From<&LangModuleSlice> for NumberRepresentation {
    fn from(value: &LangModuleSlice) -> Self {
        let mut integer = 0;
        let mut decimal = None;
        let mut negative = false;
        let code = value.code();

        let represented_as = if code.starts_with("0b") {
            NumberRepresentationBases::Binary
        } else if code.starts_with("0x") {
            NumberRepresentationBases::Hexadecimal
        } else {
            NumberRepresentationBases::Decimal
        };

        // Index shifting in case the number is a float
        let mut index_shift = 0;
        let mut base_declarated = false;
        for (index, digit) in code.trim().chars().rev().enumerate() {
            assert!(
                !negative,
                "The '-' (negative) symbol is not at the begining of the number."
            );
            assert!(
                !base_declarated || (index == code.len().saturating_sub(1) && digit == '0'),
                "The base declaration is not well placed."
            );

            let digit_index = index.saturating_sub(index_shift);
            match digit {
                '.' => {
                    assert!(decimal.is_none(), "Found 2 '.' characters in the number.");
                    decimal = Some(take(&mut integer));
                    index_shift = index;
                }
                '-' => {
                    negative = true;
                }
                '_' => {
                    index_shift += 1;
                }
                'x' | 'b' | 'd' => {
                    base_declarated = true;
                }
                '0' => {}
                _ => {
                    let Some(dec_value) = represented_as.convert_digit(digit) else {
                        panic!(
                            "{} is not a valid number's digit (at least in base {:?}.).",
                            digit, represented_as
                        )
                    };

                    integer += dec_value * represented_as.multiplier(digit_index);
                }
            }
        }

        Self {
            negative,
            integer,
            decimal,
            represented_as,
        }
    }
}
