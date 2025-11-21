use flylang::flylang::FlyLang;
#[cfg(test)]
pub mod tests {
    use flylang::flylang::{
        lexer::tokens::representations::number::{NumberRepresentation, NumberRepresentationBases},
        parser::ast::{
            expressions::{Expressions, literals::ParsedLiterals},
            instructions::Instructions,
        },
    };

    use super::*;

    const SCRIPTS_LABEL: Option<&str> = Some("tests-literals");

    /* #region Booleans */
    #[test]
    fn true_value() {
        let parsed = FlyLang::anonymous_parser(r#"true"#, SCRIPTS_LABEL)
            .parse()
            .to_vec();

        assert_eq!(parsed.len(), 1);
        assert!(matches!(
            parsed[0].kind(),
            Instructions::ValueOf(Expressions::Literal(ParsedLiterals::True))
        ));
    }

    #[test]
    fn false_value() {
        let parsed = FlyLang::anonymous_parser(r#"false"#, SCRIPTS_LABEL)
            .parse()
            .to_vec();

        assert_eq!(parsed.len(), 1);
        assert!(matches!(
            parsed[0].kind(),
            Instructions::ValueOf(Expressions::Literal(ParsedLiterals::False))
        ));
    }
    /* #endregion */

    #[test]
    fn empty_value() {
        let parsed = FlyLang::anonymous_parser(r#"()"#, SCRIPTS_LABEL)
            .parse()
            .to_vec();

        assert_eq!(parsed.len(), 1);
        assert!(matches!(
            parsed[0].kind(),
            Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Empty))
        ));
    }

    #[test]
    fn word_value() {
        let parsed = FlyLang::anonymous_parser(r#"hello"#, SCRIPTS_LABEL)
            .parse()
            .to_vec();

        assert_eq!(parsed.len(), 1);
        assert!(matches!(
            parsed[0].kind(),
            Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Word))
        ));
    }

    /* #region Numbers */
    #[test]
    fn number_integer() {
        let parsed = FlyLang::anonymous_parser(r#"10_1568"#, SCRIPTS_LABEL)
            .parse()
            .to_vec();

        assert_eq!(parsed.len(), 1);
        assert!(matches!(
            parsed[0].kind(),
            Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Number))
        ));

        let num: f64 = NumberRepresentation::from(parsed[0].location()).into();
        assert_eq!(num, 101568f64);
    }

    #[test]
    fn number_neg_integer() {
        let parsed = FlyLang::anonymous_parser(r#"-68"#, SCRIPTS_LABEL)
            .parse()
            .to_vec();

        assert_eq!(parsed.len(), 1);
        assert!(matches!(
            parsed[0].kind(),
            Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Number))
        ));

        let num: f64 = NumberRepresentation::from(parsed[0].location()).into();
        assert_eq!(num, -68f64);
    }

    #[test]
    fn number_float() {
        let parsed = FlyLang::anonymous_parser(r#".15_8"#, SCRIPTS_LABEL)
            .parse()
            .to_vec();

        assert_eq!(parsed.len(), 1);
        assert!(matches!(
            parsed[0].kind(),
            Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Number))
        ));

        let num: f64 = NumberRepresentation::from(parsed[0].location()).into();
        assert_eq!(num, 0.158f64);
    }

    #[test]
    fn number_neg_float() {
        let parsed = FlyLang::anonymous_parser(r#"-.9874"#, SCRIPTS_LABEL)
            .parse()
            .to_vec();

        assert_eq!(parsed.len(), 1);
        assert!(matches!(
            parsed[0].kind(),
            Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Number))
        ));

        let num: f64 = NumberRepresentation::from(parsed[0].location()).into();
        assert_eq!(num, -0.9874f64);
    }
    /* #endregion */

    #[test]
    fn number_bin() {
        let parsed = FlyLang::anonymous_parser(r#"0b10110"#, SCRIPTS_LABEL)
            .parse()
            .to_vec();

        assert_eq!(parsed.len(), 1);
        assert!(matches!(
            parsed[0].kind(),
            Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Number))
        ));

        let number = NumberRepresentation::from(parsed[0].location());
        assert!(matches!(
            number.represented_as,
            NumberRepresentationBases::Binary
        ));

        let value: f64 = number.into();
        assert_eq!(value, 22f64);
    }

    #[test]
    fn number_bin_neg() {
        let parsed = FlyLang::anonymous_parser(r#"-0b110"#, SCRIPTS_LABEL)
            .parse()
            .to_vec();

        assert_eq!(parsed.len(), 1);
        assert!(matches!(
            parsed[0].kind(),
            Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Number))
        ));

        let value: f64 = NumberRepresentation::from(parsed[0].location()).into();
        assert_eq!(value, -6f64);
    }

    #[test]
    #[should_panic]
    fn number_bin_invalid_char() {
        FlyLang::anonymous_parser(r#"0b2"#, SCRIPTS_LABEL).parse();
        FlyLang::anonymous_parser(r#"0bf"#, SCRIPTS_LABEL).parse();
        FlyLang::anonymous_parser(r#"0b5"#, SCRIPTS_LABEL).parse();
    }

    #[test]
    #[should_panic]
    fn number_bin_decimal_disallowed() {
        let parsed = FlyLang::anonymous_parser(r#"0b101.101"#, SCRIPTS_LABEL)
            .parse()
            .to_vec();

        assert_eq!(parsed.len(), 1);
        assert!(matches!(
            parsed[0].kind(),
            Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Number))
        ));
    }

    #[test]
    fn number_hex() {
        let parsed = FlyLang::anonymous_parser(r#"0x5FaDef"#, SCRIPTS_LABEL)
            .parse()
            .to_vec();

        assert_eq!(parsed.len(), 1);
        assert!(matches!(
            parsed[0].kind(),
            Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Number))
        ));

        let number = NumberRepresentation::from(parsed[0].location());
        assert!(matches!(
            number.represented_as,
            NumberRepresentationBases::Hexadecimal
        ));

        let value: f64 = number.into();
        assert_eq!(value, 6270447f64);
    }

    #[test]
    #[should_panic]
    fn number_hex_invalid_char() {
        FlyLang::anonymous_parser(r#"0bGu"#, SCRIPTS_LABEL).parse();
        FlyLang::anonymous_parser(r#"0b5n"#, SCRIPTS_LABEL).parse();
    }

    #[test]
    #[should_panic]
    fn number_hex_decimal_disallowed() {
        let parsed = FlyLang::anonymous_parser(r#"0xeff.a55"#, SCRIPTS_LABEL)
            .parse()
            .to_vec();

        assert_eq!(parsed.len(), 1);
        assert!(matches!(
            parsed[0].kind(),
            Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Number))
        ));
    }
}
