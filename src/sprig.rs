// credis to https://github.com/rajatjindal/handlebars-sprig

use chrono::{DateTime, Utc};
use handlebars::{handlebars_helper, Handlebars};
use rand::Rng;

pub fn add_math_helpers(x: &mut Handlebars) {
    handlebars_helper!(add: |a: isize, b: isize| a + b);
    handlebars_helper!(sub: |a: isize, b: isize| a - b);
    handlebars_helper!(mul: |a: isize, b: isize| a * b);
    handlebars_helper!(div: |a: isize, b: isize| a / b);
    handlebars_helper!(modulus: |a: isize, b: isize| a % b);
    handlebars_helper!(max: |a: isize, b: isize| if a > b { a } else { b });
    handlebars_helper!(min: |a: isize, b: isize| if a < b { a } else { b });
    handlebars_helper!(floor: |a: f64| a.floor());
    handlebars_helper!(ceil: |a: f64| a.ceil());
    handlebars_helper!(round: |a: f64| a.round());
    handlebars_helper!(rand_int:  | | rand::thread_rng().gen::<usize>());

    x.register_helper("add", Box::new(add));
    x.register_helper("sub", Box::new(sub));
    x.register_helper("mul", Box::new(mul));
    x.register_helper("div", Box::new(div));
    x.register_helper("mod", Box::new(modulus));
    x.register_helper("max", Box::new(max));
    x.register_helper("floor", Box::new(floor));
    x.register_helper("ceil", Box::new(ceil));
    x.register_helper("round", Box::new(round));
    x.register_helper("rand_int", Box::new(rand_int));
}

pub fn add_str_helpers(x: &mut Handlebars) {
    handlebars_helper!(upper: |s: String| s.to_uppercase());
    handlebars_helper!(lower: |s: String| s.to_lowercase());
    handlebars_helper!(trunc: |l: usize, s: String| {
        let mut data = s.clone();
        data.truncate(l);
        data
    });
    handlebars_helper!(abbrev: |l: usize, s: String| {
        let mut data = s.clone();
        if l <= 3 {
            data.truncate(l);
            data
        } else if data.len() <= l {
            data
        } else {
            let l = l - 3;
            data.truncate(l);
            format!("{}...", data)
        }
    });
    handlebars_helper!(trim: |s:String| s.trim());
    handlebars_helper!(plural: |count: usize, sing: String, plur: String| if count == 1 {
        sing
    } else {
        plur
    });

    handlebars_helper!(join: |delimiter: String, elements: Vec<String>|{
        elements.join(delimiter.as_str())
    });

    handlebars_helper!(split: |delimiter: String, input: String|{
        input.split(delimiter.as_str()).collect::<Vec<&str>>()
    });

    handlebars_helper!(splitn: |delimiter: String, count: usize, input: String|{
        input.splitn(count, delimiter.as_str()).collect::<Vec<&str>>()
    });

    handlebars_helper!(sort_alpha: |input: Vec<String>|{
        let mut data = input.clone();
        data.sort();
        data
    });

    handlebars_helper!(trim_suffix: |suffix: String, input: String| {
        let result = match input.strip_suffix(suffix.as_str()) {
            Some(val) => val,
            _ => input.as_str(),
        };

        result
    });

    handlebars_helper!(trim_prefix: |prefix: String, input: String|{
        let result = match input.strip_suffix(prefix.as_str()) {
            Some(val) => val,
            _ => input.as_str(),
        };

        result
    });

    handlebars_helper!(trim_all: |substr: String, input: String|{
        let firstleg = match input.strip_suffix(substr.as_str()) {
            Some(val) => val,
            _ => input.as_str(),
        };

        let result = match firstleg.strip_prefix(substr.as_str()) {
            Some(val) => val,
            _ => firstleg,
        };

        result
    });

    x.register_helper("upper", Box::new(upper));
    x.register_helper("lower", Box::new(lower));
    x.register_helper("trunc", Box::new(trunc));
    x.register_helper("abbrev", Box::new(abbrev));
    x.register_helper("plural", Box::new(plural));
    x.register_helper("trim", Box::new(trim));
    x.register_helper("join", Box::new(join));
    x.register_helper("split", Box::new(split));
    x.register_helper("splitn", Box::new(splitn));
    x.register_helper("sort_alpha", Box::new(sort_alpha));
    x.register_helper("trim_suffix", Box::new(trim_suffix));
    x.register_helper("trim_prefix", Box::new(trim_prefix));
    x.register_helper("trim_all", Box::new(trim_all));
}

pub fn add_date_helpers(x: &mut Handlebars) {
    handlebars_helper!(date_format: |format_string: String, date: DateTime<Utc>| {
        date.format(format_string.as_str()).to_string()
    });
    handlebars_helper!(now: |format_string: String| {
        let date = Utc::now();
        date.format(format_string.as_str()).to_string()
    });

    // Formatting dates: https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers
    x.register_helper("date_format", Box::new(date_format));
    x.register_helper("now", Box::new(now));
}
