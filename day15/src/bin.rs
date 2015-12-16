extern crate util;
use util::get_multiline_input;

extern crate day15lib;
use day15lib::parse_ingredients;
use day15lib::recipe::Recipe;

fn main() {
    let lines = get_multiline_input("Ingredients (EOF term'd):").unwrap();
    let ings = parse_ingredients(&lines);
    let basic_recipe = Recipe::new(ings);
    let best_goodness_recipe = basic_recipe.climb_goodness();

    println!("Best recipe: {}", best_goodness_recipe);
}
