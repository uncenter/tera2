fn main() {
    let source = r#"
{# foo #}
{% extends "base.html" %}
{# bar #}
"#;
    for token in tokenize(source) {
        println!("{:?}", token);
    }
}
