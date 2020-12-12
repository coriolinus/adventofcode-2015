use crate::{
    recipe::{Recipe, TOTAL_INGREDIENTS},
    Ingredient,
};
use std::collections::hash_map::Iter;

pub struct Neighbors<'a> {
    recipe: &'a Recipe,
    incr_it: Iter<'a, Ingredient, i32>,
    decr_it: Iter<'a, Ingredient, i32>,
    incr_t: Option<(Ingredient, i32)>,
}

impl<'a> From<&'a Recipe> for Neighbors<'a> {
    fn from(recipe: &'a Recipe) -> Self {
        let mut n = Neighbors {
            recipe: recipe,
            incr_it: recipe.ingredients.iter(),
            decr_it: recipe.ingredients.iter(),
            incr_t: None,
        };
        n.increment_incr();
        n
    }
}

impl<'a> Neighbors<'a> {
    fn increment_incr(&mut self) {
        self.incr_t = match self.incr_it.next() {
            Some((ing, iq)) => Some((ing.clone(), iq + 1)),
            _ => None,
        };
    }
}

impl<'a> Iterator for Neighbors<'a> {
    type Item = Recipe;

    fn next(&mut self) -> Option<Recipe> {
        // println!("   | Called `Neighbors::next()`");
        if self.incr_t.is_none() {
            // println!("   |  self.incr_t is None; ending. ");
            return None;
        }
        let (incr, inc_q) = self.incr_t.to_owned().unwrap();
        // println!("   |  Ingredient to increase & qty: {}, {}", incr, inc_q);

        // ensure we're not trying to increase any quantity over the max
        if inc_q > TOTAL_INGREDIENTS {
            // println!("   |  qty was too high. Incrementing incr_t and recursing.");
            self.increment_incr();
            return self.next();
        }

        // get the ingredient to decrease
        let decr_t = self.decr_it.next();

        // reset and get the next increasing ingredient if we're out of ones to decrease
        if decr_t == None {
            // println!("   |  Out of items to decrease. Increasing incr_t and recursing.");
            self.increment_incr();
            // Don't forget to reset the iterator of items to decrease!
            self.decr_it = self.recipe.ingredients.iter();
            return self.next();
        }

        // here, incr and decr are not None
        let (decr, &dec_q) = decr_t.unwrap();
        // println!("   |  Ingredient to decrease and cur qty: {}, {}", decr, dec_q);

        // continue iteration if we find the same ingredient we're increasing
        if *decr == incr {
            // println!("   |  Decrease item was the same as Increase item. Recursing.");
            return self.next();
        }

        // ensure we're not trying to reduce any quantity below zero
        if dec_q >= 1 {
            // actually decrement the quantity of dec_q
            let dec_q = dec_q - 1;
            // println!("   |  Using {} as dec_q", dec_q);

            // finally construct a new Recipe
            let mut recipe = self.recipe.clone();
            recipe.ingredients.insert(incr.clone(), inc_q);
            recipe.ingredients.insert(decr.clone(), dec_q);
            assert_eq!(
                recipe.ingredients.values().fold(0, |acc, v| acc + v),
                TOTAL_INGREDIENTS
            );

            // println!("   | Successfully found next neighbor. Returning.");
            Some(recipe)
        } else {
            // println!("   |  Couldn't decrease item qty. Recursing.");
            // oops, just get the next thing
            self.next()
        }
    }
}
