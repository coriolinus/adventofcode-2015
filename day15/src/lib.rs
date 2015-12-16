//! # Day 15: Science for Hungry People
//!
//! Today, you set out on the task of perfecting your milk-dunking cookie recipe. All you have to
//! do is find the right balance of ingredients.
//!
//! Your recipe leaves room for exactly `100` teaspoons of ingredients. You make a list of the
//! remaining ingredients you could use to finish the recipe (your puzzle input) and their
//! properties per teaspoon:
//!
//! - `capacity` (how well it helps the cookie absorb milk)
//! - `durability` (how well it keeps the cookie intact when full of milk)
//! - `flavor` (how tasty it makes the cookie)
//! - `texture` (how it improves the feel of the cookie)
//! - `calories` (how many calories it adds to the cookie)
//!
//! You can only measure ingredients in whole-teaspoon amounts accurately, and you have to be
//! accurate so you can reproduce your results in the future. The total score of a cookie can be
//! found by adding up each of the properties (negative totals become 0) and then multiplying
//! together everything except calories.
//!
//! This program hill-climbs to a local maximum and hopes for the best.




pub mod ingredient;
use ingredient::Ingredient;
pub mod recipe;

pub fn parse_ingredients(lines: &str) -> Vec<Ingredient> {
    let mut v = Vec::new();

    for line in lines.split('\n') {
        let ing = Ingredient::parse_line(line);
        if ing.is_some() {
            v.push(ing.unwrap());
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::parse_ingredients;
    use super::recipe::Recipe;

    fn get_example() -> String {
        let mut s = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\n".to_string();
        s.push_str("Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3\n");
        s
    }

    #[test]
    fn test_parse_ingredients() {
        let ings = parse_ingredients(&get_example());
        assert_eq!(ings.len(), 2);
    }

    #[test]
    fn test_hill_climb_example() {
        let ings = parse_ingredients(&get_example());
        let ref bs = ings[0]; // butterscotch
        let ref cm = ings[1]; // cinnamon

        let recipe = Recipe::new(ings.to_owned()).climb_goodness();

        assert_eq!(recipe.goodness(), 62842880);
        assert_eq!(recipe.ingredients.get(bs).unwrap(), &44);
        assert_eq!(recipe.ingredients.get(cm).unwrap(), &56);
    }
}
