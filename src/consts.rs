use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref SLUG_REGEX: Regex = Regex::new(r"^[a-z0-9-]*$").unwrap();
    pub static ref PASS_REGEX: Regex =
        Regex::new(r"^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9])(?=.*?[#?!@$%^&*-]).{8,}$").unwrap();
    pub static ref SPACE_REGEX: Regex = Regex::new(r"\s+").unwrap();
}
