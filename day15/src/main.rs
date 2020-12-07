use util::get_multiline_input;

use day15::parse_ingredients;
use day15::recipe::Recipe;

fn main() {
    let lines = get_multiline_input("Ingredients (EOF term'd):").unwrap();
    let ings = parse_ingredients(&lines);
    let basic_recipe = Recipe::new(ings);
    let best_goodness_recipe = basic_recipe.climb_goodness();

    println!("Best recipe: {}", best_goodness_recipe);

    if let Some(five_hundred) = basic_recipe.exhaust_goodness_constrained(500) {
        println!("Best recipe with 500 calories: {}", five_hundred);
    } else {
        println!("Couldn't find a recipe to match the constraint!");
    }
}
