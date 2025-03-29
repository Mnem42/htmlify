use crate::parser;

#[test]
fn parser_test(){
    let parser = parser::Parser::new(&[
        "# foo".to_string(),
        " #bar".to_string(),
        " # baz".to_string(),
        "#not a title".to_string(),
        "Line 1".to_string()
    ]).collect::<Vec<_>>();
    println!("{:?}",parser);
    panic!("Check.");
}