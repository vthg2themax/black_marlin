use black_marlin::TemplateOnce;
use black_marlin_macros::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "foo.stpl", escape=true)]
#[template(escape = false)]
struct InvalidOptionValue {
    name: String
}

fn main() {
    println!("{}", InvalidOptionValue { name: "Hanako".to_owned() }.render_once().unwrap());
}
