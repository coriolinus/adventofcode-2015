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

pub mod neighbors;
pub mod recipe;
use recipe::Recipe;

use aoc2015::parse;
use std::path::Path;
use thiserror::Error;

/// A model of a recipe ingredient
#[derive(PartialEq, Eq, Hash, Clone, Debug, parse_display::Display, parse_display::FromStr)]
#[display("{name}: capacity {capacity}, durability {durability}, flavor {flavor}, texture {texture}, calories {calories}")]
pub struct Ingredient {
    pub name: String,
    pub capacity: i32,
    pub durability: i32,
    pub flavor: i32,
    pub texture: i32,
    pub calories: i32,
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let basic_recipe: Recipe = parse(input)?.collect();
    let best_recipe = basic_recipe.climb_goodness();
    println!("best recipe goodness: {}", best_recipe.goodness());
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    const CONSTRAINT: i32 = 500;
    let basic_recipe: Recipe = parse(input)?.collect();
    let best_recipe = basic_recipe
        .exhaust_goodness_constrained(CONSTRAINT)
        .ok_or(Error::NoSuchRecipe(CONSTRAINT))?;
    println!(
        "best recipe goodness (constrained to {} calories): {}",
        CONSTRAINT,
        best_recipe.goodness()
    );
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no recipe found which matches constraint: {0} calories")]
    NoSuchRecipe(i32),
}

#[cfg(test)]
mod tests {
    use crate::{recipe::Recipe, Ingredient};

    const EXAMPLE: &str = "
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    ";

    fn example() -> impl Iterator<Item = Ingredient> {
        EXAMPLE.trim().split('\n').map(|line| line.parse().unwrap())
    }

    #[test]
    fn test_from_str() {
        assert_eq!(example().count(), 2);
    }

    #[test]
    fn test_hill_climb_example() {
        let recipe = example().collect::<Recipe>().climb_goodness();
        let get_ingredient = |name: &str| {
            recipe.ingredients.iter().find_map(|(ingredient, qty)| {
                if ingredient.name == name {
                    Some(*qty)
                } else {
                    None
                }
            })
        };

        assert_eq!(recipe.goodness(), 62842880);
        assert_eq!(get_ingredient("Butterscotch").unwrap(), 44);
        assert_eq!(get_ingredient("Cinnamon").unwrap(), 56);
    }

    #[test]
    fn test_exhaust_example_constrained() {
        let recipe = example()
            .collect::<Recipe>()
            .exhaust_goodness_constrained(500);
        let recipe = recipe.unwrap();

        let get_ingredient = |name: &str| {
            recipe.ingredients.iter().find_map(|(ingredient, qty)| {
                if ingredient.name == name {
                    Some(*qty)
                } else {
                    None
                }
            })
        };

        assert_eq!(recipe.goodness(), 57600000);
        assert_eq!(get_ingredient("Butterscotch").unwrap(), 40);
        assert_eq!(get_ingredient("Cinnamon").unwrap(), 60);
    }
}
