extern crate proc_macro;

use std::{env::current_dir, fs::read_dir};

use proc_macro::TokenStream;
use regex::Regex;

const MOD_TEMPLATE: &str = r##"
#[path="./{{mod_name}}/mod.rs"]
mod {{mod_name}};
"##;

const DAYS_FN_TEMPLATE: &str = r##"
pub fn days() -> [Box<dyn AocTask>; {{n_days}}] {
    [
        {{boxed_days}}
    ]
}
"##;

const BOX_DAY_TEMPLATE: &str = "Box::new({{mod_name}}::{{struct_name}})";

#[proc_macro]
pub fn aoc_register(_stream: TokenStream) -> TokenStream {
    let cwd = current_dir().unwrap();
    let pattern = Regex::new(r"^day\d\d$").unwrap();
    let mut days = Vec::new();
    for r in read_dir(cwd.join("src")).unwrap() {
        let dir_entry = r.unwrap();
        if !dir_entry.file_type().unwrap().is_dir() {
            continue;
        }
        let name = dir_entry.file_name().to_str().unwrap().to_owned();
        if !pattern.is_match(&name) {
            continue;
        }
        days.push(name);
    }
    days.sort();
    let mut mods = Vec::new();
    let mut boxed_days = Vec::new();
    for day in days {
        mods.push(MOD_TEMPLATE.replace("{{mod_name}}", &day));
        boxed_days.push(BOX_DAY_TEMPLATE.replace("{{mod_name}}", &day).replace(
            "{{struct_name}}",
            &(day.chars().next().unwrap().to_uppercase().to_string()
                + &day.chars().skip(1).collect::<String>()),
        ));
    }
    let days_fn = DAYS_FN_TEMPLATE
        .replace("{{n_days}}", &(boxed_days.len().to_string()))
        .replace("{{boxed_days}}", &boxed_days.join(","));

    format!("{}\n{}", mods.join("\n"), days_fn).parse().unwrap()
}
