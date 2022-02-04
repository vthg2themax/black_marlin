use black_marlin::TemplateOnce;
use black_marlin_macros::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "missing_semicolon.stpl")]
struct MissingSemicolon {}

fn main() {
    println!("{}", (MissingSemicolon {}).render_once().unwrap());
}
