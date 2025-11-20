use flylang::flylang::FlyLang;

mod literals;

#[cfg(test)]
pub mod tests {
    use super::*;

    const SCRIPTS_LABEL: Option<&str> = Some("tests-global");

    #[test]
    fn parser_with_raw_works() {
        FlyLang::anonymous_parser(r#""#, SCRIPTS_LABEL);
    }
    #[test]
    fn parser_with_file_works() {
        FlyLang::parser(FlyLang::path("tests/scripts/empty.fly".to_string()));
    }
    #[test]
    #[should_panic]
    fn unknown_path_not_working() {
        FlyLang::parser(FlyLang::path("".to_string()));
    }
}
