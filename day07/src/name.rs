use super::parse::{is_just_letters, Parseable};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Name {
    val: String,
}

impl Name {
    pub fn get(&self) -> &str {
        &self.val
    }
}

impl Parseable for Name {
    type P = Name;
    fn parse(v: &str) -> Option<Name> {
        if is_just_letters(v) {
            Some(Name { val: v.to_string() })
        } else {
            None
        }
    }
}
