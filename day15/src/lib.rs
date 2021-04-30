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

pub mod recipe;
use recipe::Recipe;

use aoclib::parse;
use std::path::Path;
use thiserror::Error;

/// A model of a recipe ingredient
#[derive(PartialEq, Eq, Clone, Debug, parse_display::Display, parse_display::FromStr)]
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
