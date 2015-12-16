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

const TOTAL_INGREDIENTS: u32 = 100;
