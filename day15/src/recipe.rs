use std::fmt;

use std::collections::hash_map::Iter;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use super::ingredient::Ingredient;

const TOTAL_INGREDIENTS: u16 = 100;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Recipe {
    pub ingredients: HashMap<Ingredient, u16>,
}

impl Recipe {
    pub fn new(ings: Vec<Ingredient>) -> Recipe {
        let default_qty = TOTAL_INGREDIENTS / ings.len() as u16;
        let mut used = 0;

        let mut ret = HashMap::new();

        for ing in ings {
            let qty = if default_qty <= (TOTAL_INGREDIENTS - used) {
                default_qty
            } else {
                TOTAL_INGREDIENTS - used
            };

            ret.insert(ing, qty);
            used += qty;
        }

        Recipe { ingredients: ret }
    }

    pub fn goodness(&self) -> i32 {
        let mut capacity: i32 = 0;
        let mut durability: i32 = 0;
        let mut flavor: i32 = 0;
        let mut texture: i32 = 0;

        for (ing, qty) in &self.ingredients {
            capacity += *qty as i32 * ing.capacity;
            durability += *qty as i32 * ing.durability;
            flavor += *qty as i32 * ing.flavor;
            texture += *qty as i32 * ing.texture;
        }

        for quality in &[capacity, durability, flavor, texture] {
            if *quality < 0 {
                return 0;
            }
        }

        capacity * durability * flavor * texture
    }

    pub fn neighbors(&self) -> Neighbors {
        Neighbors::new(self)
    }

    pub fn climb_goodness(&self) -> Recipe {
        if self.ingredients.len() < 2 {
            // no neighbors can exist
            return self.clone();
        }
        // for 2 or more ingredients, at least one neighbor must exist

        let mut best_recipe = self.to_owned();
        let mut prev_best_recipe = self.to_owned();
        let mut best_goodness = self.goodness();

        loop {
            // println!("");
            // println!("Neighbors of {}:", prev_best_recipe);
            for n in prev_best_recipe.neighbors() {
                // println!(" - {}", n);
                if n.goodness() > best_goodness {
                    best_recipe = n.to_owned();
                    best_goodness = n.goodness();
                }
            }
            if best_recipe == prev_best_recipe {
                // we've found at least a local maximum
                break;
            } else {
                prev_best_recipe = best_recipe.to_owned();
            }
        }
        best_recipe
    }

    pub fn calories(&self) -> i32 {
        self.ingredients
            .iter()
            .map(|(ing, &qty)| ing.calories * qty as i32)
            .fold(0, |acc, val| acc + val)
    }

    pub fn exhaust_goodness_constrained(&self, calories: i32) -> Option<Recipe> {
        let mut best_constrained_recipe = if self.calories() == calories {
            Some(self.to_owned())
        } else {
            None
        };
        if self.ingredients.len() < 2 {
            // no neighbors can exist
            return best_constrained_recipe;
        }
        // for 2 or more ingredients, at least one neighbor must exist

        let mut visited = HashSet::new();
        let mut future = Vec::new();
        let mut to_examine = self.neighbors().collect::<Vec<_>>();

        loop {
            for recipe in to_examine {
                // skip work if we can
                if visited.contains(&recipe) {
                    continue;
                }
                visited.insert(recipe.to_owned());

                // add future work
                future.extend(recipe.neighbors());

                // check this recipe
                if recipe.calories() == calories
                    && (best_constrained_recipe.is_none()
                        || recipe.goodness() > best_constrained_recipe.clone().unwrap().goodness())
                {
                    best_constrained_recipe = Some(recipe.clone());
                }
            }

            // reset to_examine to a list of items not already seen
            to_examine = future
                .iter()
                .filter(|r| !visited.contains(r))
                .cloned()
                .collect::<Vec<_>>();
            if to_examine.len() == 0 {
                break;
            }
            future = Vec::new();
        }

        best_constrained_recipe
    }
}

impl Hash for Recipe {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.ingredients.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = write!(f, "Result<");
        if self.ingredients.len() > 0 {
            res = res.and(write!(f, "{{"));

            for (k, v) in self.ingredients.iter() {
                res = res.and(write!(f, "{}: {}, ", k, v));
            }

            res = res.and(write!(f, "}}"));
        }
        res.and(write!(f, ", {}>", self.goodness()))
    }
}

pub struct Neighbors<'a> {
    recipe: &'a Recipe,
    incr_it: Iter<'a, Ingredient, u16>,
    decr_it: Iter<'a, Ingredient, u16>,
    incr_t: Option<(Ingredient, u16)>,
}

impl<'a> Neighbors<'a> {
    fn new(recipe: &Recipe) -> Neighbors {
        let mut n = Neighbors {
            recipe: recipe,
            incr_it: recipe.ingredients.iter(),
            decr_it: recipe.ingredients.iter(),
            incr_t: None,
        };
        n.increment_incr();
        n
    }

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
