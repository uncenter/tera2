use std::fmt::Debug;

use tera::parsing::lexer::tokenize;

fn main() {
    let source = r#"
{% extends "base.html" %}
"#;
    for token in tokenize(source) {
        println!("{:?}", token);
    }

    // let parser = tera::parsing::parser::Parser::new(&source);
    // let ast = parser.parse().expect("should parse correctly");
    // println!("{:?}", ast.nodes.first().unwrap());
    // for node in ast.nodes {
    //     println!("{}", node);
    // }
}
