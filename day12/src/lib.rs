use aoc2015::parse;
use std::path::Path;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let numbers_in: i64 = parse::<serde_json::Value>(input)?
        .map(|value| sum_of_numbers_in(&value, &|_| true))
        .sum();
    println!("numbers in the input: {}", numbers_in);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let numbers_in: i64 = parse::<serde_json::Value>(input)?
        .map(|value| {
            sum_of_numbers_in(&value, &|obj| {
                !obj.values()
                    .any(|value| value.as_str().map(|s| s == "red").unwrap_or_default())
            })
        })
        .sum();
    println!("non-red numbers in the input: {}", numbers_in);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// sum up integers in a json value
///
/// we can represent these as integers, as there are no decimal points in the input
fn sum_of_numbers_in(
    value: &serde_json::Value,
    filter_objects: &dyn Fn(&serde_json::Map<String, serde_json::Value>) -> bool,
) -> i64 {
    let sum_inner = |value: &serde_json::Value| sum_of_numbers_in(value, filter_objects);

    match value {
        serde_json::Value::Number(n) => n.as_i64().unwrap_or_default(),
        serde_json::Value::Array(values) => values.iter().map(sum_inner).sum(),
        serde_json::Value::Object(object) => {
            if filter_objects(object) {
                object.values().map(sum_inner).sum()
            } else {
                0
            }
        }
        _ => 0,
    }
}
