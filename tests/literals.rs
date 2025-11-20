use flylang::flylang::FlyLang;
#[cfg(test)]
pub mod tests {
    use flylang::flylang::{
        lexer::tokens::representations::number::NumberRepresentation,
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

        dbg!(&parsed);
        assert_eq!(parsed.len(), 1);
        // ? The parsed element is a reverse(sign) of number(0.9874)
        // todo Replace reverse(sign) of number(x) to number(x) with sign
        // assert!(matches!(
        //     parsed[0].kind(),
        //     Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Number))
        // ));

        let num: f64 = NumberRepresentation::from(parsed[0].location()).into();
        assert_eq!(num, -0.9874f64);
    }
    /* #endregion */
}
