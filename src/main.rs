use clap::Parser;
use handlebars::Handlebars;
use std::{fs, io};

/// Comando que permite aplicar variables de un archivo JSON a una plantilla de handlebars (.hbs).
#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = None,
)]
struct AppArgs {
    /// Archivo de plantilla
    template: String,
    /// Archivo de variables
    vars: String,
    /// Uses handlebars' strict mode
    #[arg(long)]
    strict: bool,
}

fn main() -> io::Result<()> {
    let args = AppArgs::parse();
    let mut handlebars = Handlebars::new();
    let template =
        fs::read_to_string(&args.template).expect("template file not found / couldn't be opened");
    let vars = fs::read_to_string(&args.vars).expect("vars file not found / couldn't be opened");
    handlebars
        .register_template_string("template", template)
        .unwrap();

    let vars = serde_json::from_str::<serde_json::Value>(&vars)?;
    if args.strict {
        handlebars.set_strict_mode(true);
    }
    println!("{}", handlebars.render("template", &vars).unwrap());
    Ok(())
}
