use black_marlin::TemplateOnce;
use black_marlin_macros::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "unclosed_delimiter.stpl")]
struct UnclosedDelimiter {
    content: String,
}

fn main() {
    println!(
        "{}",
        UnclosedDelimiter {
            content: String::from("Hello, world!")
        }
        .render_once()
        .unwrap()
    )
}
