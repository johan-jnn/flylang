use std::mem::take;

use crate::flylang::lexer::tokens::{Number, Token};

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
}

pub struct NumberRepresentation {
    pub negative: bool,
    pub integer: u64,
    pub decimal: Option<u64>,
    pub represented_as: NumberRepresentationBases,
}
impl From<Token<Number>> for NumberRepresentation {
    fn from(value: Token<Number>) -> Self {
        let mut integer = 0;
        let mut decimal = None;
        let mut negative = false;
        let code = value.location().code();

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
                '1' => integer += represented_as.multiplier(digit_index),
                '2' => integer += represented_as.multiplier(digit_index) * 2,
                '3' => integer += represented_as.multiplier(digit_index) * 3,
                '4' => integer += represented_as.multiplier(digit_index) * 4,
                '5' => integer += represented_as.multiplier(digit_index) * 5,
                '6' => integer += represented_as.multiplier(digit_index) * 6,
                '7' => integer += represented_as.multiplier(digit_index) * 7,
                '8' => integer += represented_as.multiplier(digit_index) * 8,
                '9' => integer += represented_as.multiplier(digit_index) * 9,
                _ => panic!("{} is not a valid number's digit.", digit),
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
