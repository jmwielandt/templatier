use clap::Parser;
use handlebars::{handlebars_helper, Handlebars, JsonTruthy};
use serde_json::Value;
use std::fs;

use crate::customhelper::{IsDefined, IsDefinedPass, IsUndefined};

mod customhelper;
mod sprig;

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

// trait Booly {
//     fn booly_value(&self) -> bool;
// }

// impl Booly for Value {
//     fn booly_value(&self) -> bool {
//         self.is_truthy(include_zero)
//     }
// }

// fn or_(a: impl Booly, b: impl Booly) -> bool {
//     a.value() || b.value()
// }

const TPLT: &str = "template";

fn main() -> anyhow::Result<()> {
    let args = AppArgs::parse();
    let template =
        fs::read_to_string(&args.template).expect("template file not found / couldn't be opened");
    let vars = fs::read_to_string(&args.vars).expect("vars file not found / couldn't be opened");

    let handlebars: Handlebars = build_hb_registry(&template, args.strict)?;

    let vars = serde_json::from_str::<serde_json::Value>(&vars)?;
    match handlebars.render(TPLT, &vars) {
        Ok(out) => {
            println!("{out}")
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    Ok(())
}

fn build_hb_registry(template: &str, strict_mode: bool) -> anyhow::Result<Handlebars> {
    let mut handlebars = Handlebars::new();

    handlebars.register_template_string(TPLT, template)?;
    handlebars.set_strict_mode(strict_mode);

    // add sprig helpers
    sprig::add_math_helpers(&mut handlebars);
    sprig::add_str_helpers(&mut handlebars);
    sprig::add_date_helpers(&mut handlebars);

    // add my extra helpers
    handlebars_helper!(or:|a: Value, b: Value, {include_zero: bool = false}| { a.is_truthy(include_zero) || b.is_truthy(include_zero) });

    handlebars_helper!(and: |a: bool, b: bool| a && b);
    handlebars_helper!(not: |a: bool| !a);
    handlebars_helper!(xor: |a: bool, b: bool| !(a && b) && (a || b));

    handlebars.register_helper("or", Box::new(or));
    handlebars.register_helper("and", Box::new(and));
    handlebars.register_helper("not", Box::new(not));
    handlebars.register_helper("xor", Box::new(xor));
    handlebars.register_helper("isdef", Box::new(IsDefined));
    handlebars.register_helper("isdef_pass", Box::new(IsDefinedPass));
    handlebars.register_helper("isundef", Box::new(IsUndefined));

    Ok(handlebars)
}

#[cfg(test)]
mod tests {
    const TPLT: &str = super::TPLT;
    use serde_json::json;

    use crate::build_hb_registry;

    #[test]
    fn simple_template() {
        let tpl = "{{name}}";
        let hb = build_hb_registry(tpl, false).expect("couldn't build template");

        let values = json!({"name": "john"});
        assert_eq!(
            hb.render(TPLT, &values).expect("couldn't render template"),
            "john"
        );
    }

    #[test]
    fn sprig_test_simple_add() {
        let tpl = "{{add p1.age p2.age}}";
        let hb = build_hb_registry(tpl, false).expect("couldn't build template");

        let values = json!({
            "p1": {
                "age": 10,
            },
            "p2": {
                "age": 20,
            }
        });
        assert_eq!(
            hb.render(TPLT, &values).expect("couldn't render template"),
            "30"
        );
    }

    // #[test]
    // fn sprig_test_undefined_p2_age() {
    //     let tpl = "{{add p1.age p2.age includeZeros=true}}";
    //     let hb = build_hb_registry(tpl, false).expect("couldn't register template");
    //     let values = json!({
    //         "p1": {
    //             "age": 10,
    //         },
    //         "p2": {}
    //     });
    //     assert_eq!(
    //         hb.render(TPLT, &values).expect("couldn't render template"),
    //         "10"
    //     );
    // }

    #[test]
    fn test_simple_or() {
        println!("holaaa");
        let tpl = "{{or p1 p2}}";
        let hb = build_hb_registry(tpl, false).expect("couldn't build template");

        let values = json!({
            "p1": true,
            "p2": false
        });
        assert_eq!(
            hb.render(TPLT, &values).expect("couldn't render template"),
            "true"
        );
    }

    #[test]
    fn test_less_simple_or() {
        // let tpl = "{{or p1 p2 includeZeros=true}}";
        let tpl = "{{or p1 p2}}";
        let hb = build_hb_registry(tpl, false).expect("couldn't build template");

        let values = json!({
            "p1": true,
            // "p2": false
        });
        assert_eq!(
            hb.render(TPLT, &values).expect("couldn't render template"),
            "true"
        );
    }

    #[test]
    fn simple_if_statement() {
        let tpl = "{{#if something}}hallo{{/if}}";
        let hb = build_hb_registry(tpl, false).expect("couldn't build template");

        let values = json!({
            "something": true
        });
        assert_eq!(
            hb.render(TPLT, &values).expect("couldn't render template"),
            "hallo"
        );

        let values = json!({
            "something": false
        });
        assert_eq!(
            hb.render(TPLT, &values).expect("couldn't render template"),
            ""
        );
    }

    #[test]
    fn semicomplex_if_statement() {
        let tpl = "{{#if (or a b)}}hallo{{/if}}";
        let hb = build_hb_registry(tpl, false).expect("couldn't build template");

        let values = json!({
            "a": true,
            "b": false,
        });
        assert_eq!(
            hb.render(TPLT, &values).expect("couldn't render template"),
            "hallo"
        );

        let values = json!({
            "a": false,
            "b": false,
        });
        assert_eq!(
            hb.render(TPLT, &values).expect("couldn't render template"),
            ""
        );
    }

    #[test]
    fn complex_if_statement() {
        // (a or b) and c
        let tpl = "{{#if (and (or a b) c)}}hallo{{/if}}";
        let hb = build_hb_registry(tpl, false).expect("couldn't build template");

        let values = json!({
            "a": true,
            "b": false,
            "c": true,
        });
        assert_eq!(
            hb.render(TPLT, &values).expect("couldn't render template"),
            "hallo"
        );

        let values = json!({
            "a": false,
            "b": false,
            "c": false,
        });
        assert_eq!(
            hb.render(TPLT, &values).expect("couldn't render template"),
            ""
        );

        let values = json!({
            "a": true,
            "b": true,
            "c": false,
        });
        assert_eq!(
            hb.render(TPLT, &values).expect("couldn't render template"),
            ""
        );
    }

    // #[test]
    // fn complex_if_statement_with_undefinitions() {
    //     let tpl = "{{#if (and (or a b) c) includeZeros=true}}hallo{{/if}}";
    //     let hb = build_hb_registry(tpl, false).expect("couldn't build template");

    //     let values = json!({
    //         "a": true,
    //         "b": false,
    //         "c": true,
    //     });
    //     assert_eq!(
    //         hb.render(TPLT, &values).expect("couldn't render template"),
    //         "hallo"
    //     );

    //     let values = json!({
    //         "a": false,
    //         "b": false,
    //         "c": false,
    //     });
    //     assert_eq!(
    //         hb.render(TPLT, &values).expect("couldn't render template"),
    //         ""
    //     );

    //     let values = json!({
    //         "a": true,
    //         "c": true,
    //     });
    //     assert_eq!(
    //         hb.render(TPLT, &values).expect("couldn't render template"),
    //         "hallo"
    //     );
    // }
}
