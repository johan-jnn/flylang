use flylang::flylang::{
    self as lang, FlyLang, lexer::tokens::representations::number::NumberRepresentation,
};

fn main() {
    let parsed = FlyLang::anonymous_parser(
        r#"
    -  0x0bf011
    "#,
        None,
    )
    .parse()
    .to_vec();

    let num = NumberRepresentation::from(parsed[0].location());
    dbg!(num);
}
