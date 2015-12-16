//! A model of a recipe ingredient

use std::fmt;

use std::str::FromStr;
use std::collections::HashMap;

extern crate util;
use self::util::parse::Parser;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Ingredient {
    pub name: String,
    pub capacity: i32,
    pub durability: i32,
    pub flavor: i32,
    pub texture: i32,
    pub calories: i32,
}

impl Ingredient {
    pub fn parse_line(line: &str) -> Option<Ingredient> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        let parser = Parser::default()
                         .require_at_least(Some(11))
                         .require_fewer_than(Some(12))
                         .fixed_tokens({
                             let mut h = HashMap::new();
                             h.insert(1, "capacity".to_string());
                             h.insert(3, "durability".to_string());
                             h.insert(5, "flavor".to_string());
                             h.insert(7, "texture".to_string());
                             h.insert(9, "calories".to_string());
                             h
                         });

        if let Ok(parse_result) = parser.parse(line) {
            let ref name = parse_result.tokens[0];
            let mut name = name.clone();
            name.pop(); // eliminate the trailing colon
            let ref cap_s = parse_result.tokens[1];
            let mut cap_s = cap_s.clone();
            cap_s.pop(); // eliminate trailing commas
            let ref dur_s = parse_result.tokens[2];
            let mut dur_s = dur_s.clone();
            dur_s.pop();
            let ref fla_s = parse_result.tokens[3];
            let mut fla_s = fla_s.clone();
            fla_s.pop();
            let ref tex_s = parse_result.tokens[4];
            let mut tex_s = tex_s.clone();
            tex_s.pop();
            let ref cal_s = parse_result.tokens[5];

            Some(Ingredient {
                name: name.clone(),
                capacity: i32::from_str(&cap_s).unwrap(),
                durability: i32::from_str(&dur_s).unwrap(),
                flavor: i32::from_str(&fla_s).unwrap(),
                texture: i32::from_str(&tex_s).unwrap(),
                calories: i32::from_str(&cal_s).unwrap(),
            })
        } else {
            None
        }
    }
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ingredient ({})>", self.name)
    }
}
