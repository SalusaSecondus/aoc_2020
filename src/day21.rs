use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
struct Ingredient {
    name: String,
    allergens: HashSet<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct Day21Input {
    food: Vec<Food>,
    ingredients: HashMap<String, Ingredient>,
    allergens: HashSet<String>,
}

impl Day21Input {
    fn parse_input(file_name: &str) -> Result<Day21Input> {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"^([^(]+) \((.*)\)").unwrap();
        }
        let mut food = vec![];
        let mut ingredients = HashMap::new();
        let mut allergens = HashSet::new();

        for line in crate::read_file(file_name)? {
            let line = line?;
            let c = LINE_RE.captures(&line).context("Invalid line")?;
            let ingredient_list = c.get(1).context("Missing ingredients")?.as_str();
            let allergen_list = c.get(2).context("Missing allergens")?.as_str();

            let mut food_ingredients = HashSet::new();
            let mut food_allergens = HashSet::new();
            for i in ingredient_list.split(' ') {
                food_ingredients.insert(i.to_owned());
                if !ingredients.contains_key(i) {
                    let ingredient = Ingredient {
                        name: i.to_owned(),
                        allergens: HashSet::new(),
                    };
                    ingredients.insert(i.to_owned(), ingredient);
                }
            }

            for a in allergen_list.split(' ') {
                let mut a = a;
                if a.ends_with(',') {
                    a = &a[..a.len() - 1];
                }
                allergens.insert(a.to_owned());
                food_allergens.insert(a.to_owned());
            }
            food.push(Food {
                ingredients: food_ingredients,
                allergens: food_allergens,
            });
        }

        // Now, go through and backfill all of the ingredients with allergens
        for i in ingredients.values_mut() {
            i.allergens.extend(allergens.iter().map(|s| s.to_owned()));
        }
        Ok(Day21Input {
            food,
            ingredients,
            allergens,
        })
    }

    fn find_allergens(&mut self) -> Result<()> {
        // Safe ingredients
        for ingredient in self.ingredients.values_mut() {
            for f in &self.food {
                if !f.ingredients.contains(&ingredient.name) {
                    for a in &f.allergens {
                        ingredient.allergens.remove(a);
                    }
                }
            }
        }
        let mut handled = HashSet::new();
        loop {
            let mut to_remove = Option::None;
            for ingredient in self.ingredients.values() {
                if !handled.contains(&ingredient.name) && ingredient.allergens.len() == 1 {
                    handled.insert(ingredient.name.clone());
                    to_remove = Some(
                        ingredient
                            .allergens
                            .iter()
                            .next()
                            .context("Missing allergen")?
                            .clone(),
                    );
                    break;
                }
            }
            if let Some(to_remove) = to_remove {
                for ingredient in self.ingredients.values_mut() {
                    if !handled.contains(&ingredient.name) {
                        ingredient.allergens.remove(&to_remove);
                    }
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    fn get_canonical_dangers(&self) -> Result<Vec<String>> {
        let mut result = vec![];

        for i in self.ingredients.values() {
            if i.allergens.len() == 1 {
                result.push((&i.name, i.allergens.iter().next().context("No allergen?")?));
            }
        }

        result.sort_by(|a, b| a.1.cmp(b.1));
        Ok(result.iter().map(|a| a.0.to_owned()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day21_smoke1() -> Result<()> {
        let mut input = Day21Input::parse_input("day21_smoke.txt")?;

        // for i in input.ingredients.values() {
        //     println!("{:?}", i);
        // }
        input.find_allergens()?;
        // println!();
        // for i in input.ingredients.values() {
        //     println!("{:?}", i);
        // }

        let mut safe_ingredients = vec![];
        for i in input.ingredients.values() {
            if i.allergens.is_empty() {
                safe_ingredients.push(&i.name);
            }
        }

        let mut count = 0;
        for f in &input.food {
            for s in &safe_ingredients {
                if f.ingredients.contains(*s) {
                    count += 1;
                }
            }
        }
        assert_eq!(5, count);
        let dangers = input.get_canonical_dangers()?;
        println!("Day 21 smoke. Dangers: {:?}", dangers);
        assert_eq!(vec!["mxmxvkd", "sqjhc", "fvjkl"], dangers);
        Ok(())
    }

    #[test]
    fn day21_1() -> Result<()> {
        let mut input = Day21Input::parse_input("day21.txt")?;
        input.find_allergens()?;

        let mut safe_ingredients = vec![];
        for i in input.ingredients.values() {
            if i.allergens.is_empty() {
                safe_ingredients.push(&i.name);
            }
        }

        let mut count = 0;
        for f in &input.food {
            for s in &safe_ingredients {
                if f.ingredients.contains(*s) {
                    count += 1;
                }
            }
        }
        assert_eq!(2280, count);
        let dangers = input.get_canonical_dangers()?;
        println!("Day 21.2 Dangers: {:?}", dangers);
        assert_eq!(
            vec!["vfvvnm", "bvgm", "rdksxt", "xknb", "hxntcz", "bktzrz", "srzqtccv", "gbtmdb"],
            dangers
        );
        Ok(())
    }
}
