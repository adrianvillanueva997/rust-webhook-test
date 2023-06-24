use lazy_static::lazy_static;
use regex::Regex;

const UWU_EXPR: &'static str = "\\W*((?i)%s(?-i))\\W*";
static BAD_WORDS: &'static [&str] = &["uwu", "owo", ":v", ":3"];

lazy_static! {
    static ref RE: Regex = Regex::new(UWU_EXPR).unwrap();
}

pub fn check_message(msg: &str) -> bool {
    false
}
