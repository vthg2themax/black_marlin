use black_marlin::TemplateOnce;
use black_marlin_macros::TemplateOnce;

#[derive(TemplateOnce)]
struct NoTemplate {
    var: usize
}

fn main() {
    println!("{}", NoTemplate { var: 1996 }.render_once().unwrap());
}
