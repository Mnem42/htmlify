use crate::parser::{self, MDItem};

#[test]
fn title_alignment(){
    let parser = parser::Parser::new(&[
        "# foo".to_string(),
        " #not a title".to_string(),
        " # bar".to_string(),
        "#not a title".to_string(),
        "Line 1".to_string()
    ]).collect::<Vec<_>>();
    
    assert_eq!(parser,vec![
        MDItem::Heading(1, "foo".to_string()),
        MDItem::Heading(0, "not a title".to_string()),
        MDItem::Heading(1, "bar".to_string()),
        MDItem::Heading(0, "not a title".to_string()),
        MDItem::Line("Line 1".to_string())
    ])
}

#[test]
fn title_sizes(){
    let parser = parser::Parser::new(&[
        "# H1".to_string(),
        "## H2".to_string(),
        "### H3".to_string(),
        "#### H4".to_string(),
        "##### H5".to_string(),
        "###### H6".to_string(),
        "####### The mythical H7".to_string(),
        "Line 1".to_string()
    ]).collect::<Vec<_>>();

    assert_eq!(parser,vec![
        MDItem::Heading(1, "H1".to_string()),
        MDItem::Heading(2, "H2".to_string()),
        MDItem::Heading(3, "H3".to_string()),
        MDItem::Heading(4, "H4".to_string()),
        MDItem::Heading(5, "H5".to_string()),
        MDItem::Heading(6, "H6".to_string()),
        MDItem::Heading(7, "The mythical H7".to_string()),
        MDItem::Line("Line 1".to_string())
    ])
}