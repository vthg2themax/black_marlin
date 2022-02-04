use black_marlin::TemplateOnce;
use black_marlin_macros::TemplateOnce;

struct Player<'a> {
    name: &'a str,
    score: u32,
}

#[derive(TemplateOnce)]
#[template(path = "unbalanced_brace.stpl")]
struct UnbalancedBrace {
    players: Vec<Player>,
}

fn main() {
    println!(
        "{}",
        UnclosedDelimiter {
            players: vec![Player { name: "Hanako", score: 97 }]
        }
        .render_once()
        .unwrap()
    )
}
