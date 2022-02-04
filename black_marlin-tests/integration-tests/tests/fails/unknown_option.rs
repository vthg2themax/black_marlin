use black_marlin::TemplateOnce;
use black_marlin_macros::TemplateOnce;

#[derive(TemplateOnce)]
#[template(patth = "foo.stpl")]
struct UnknownOption {
    name: String
}

fn main() {
    println!("{}", UnknownOption { name: "Hanako".to_owned() }.render_once().unwrap());
}
