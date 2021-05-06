//! --- Day 2: I Was Told There Would Be No Math ---
//!
//! The elves are running low on wrapping paper, and so they need to submit an order for more.
//! They have a list of the dimensions (length l, width w, and height h) of each present, and only
//! want to order exactly as much as they need.
//!
//! Fortunately, every present is a box (a perfect right rectangular prism), which makes
//! calculating the required wrapping paper for each gift a little easier: find the surface area of
//! the box, which is `2*l*w + 2*w*h + 2*h*l`. The elves also need a little extra paper for each
//! present: the area of the smallest side.
//!
//! For example:
//!
//! - A present with dimensions `2x3x4` requires `2*6 + 2*12 + 2*8 = 52` square feet of wrapping
//!   paper plus `6` square feet of slack, for a total of `58` square feet.
//! - A present with dimensions `1x1x10` requires `2*1 + 2*10 + 2*10 = 42` square feet of wrapping
//!   paper plus `1` square foot of slack, for a total of `43` square feet.

use aoclib::{geometry::vector3::Vector3, parse};
use std::path::Path;
use thiserror::Error;

#[derive(PartialEq, Eq, Debug, parse_display::Display, parse_display::FromStr)]
#[display("{dimensions.x}x{dimensions.y}x{dimensions.z}")]
pub struct GiftBox {
    #[from_str(default)]
    dimensions: Vector3,
}

impl GiftBox {
    /// Construct a new GiftBox
    pub fn new(x: i32, y: i32, z: i32) -> Result<GiftBox, &'static str> {
        if x > 0 && y > 0 && z > 0 {
            Ok(GiftBox {
                dimensions: Vector3 { x, y, z },
            })
        } else {
            Err("Can't construct a box with dimension 0 or less!")
        }
    }

    /// Return the surface area.
    pub fn surface_area(&self) -> i32 {
        2 * ((self.dimensions.x * self.dimensions.z)
            + (self.dimensions.x * self.dimensions.y)
            + (self.dimensions.y * self.dimensions.z))
    }

    /// Return the margin: the area of the smallest side
    pub fn smallest_side(&self) -> i32 {
        [
            (self.dimensions.x * self.dimensions.z),
            (self.dimensions.x * self.dimensions.y),
            (self.dimensions.y * self.dimensions.z),
        ]
        .iter()
        .min()
        .cloned()
        .expect("non-empty input array; qed")
    }

    /// Return the paper requirement for this box
    ///
    /// Defined in the problem as the surface area plus the area of the smallest side.
    pub fn paper(&self) -> i32 {
        self.surface_area() + self.smallest_side()
    }

    pub fn volume(&self) -> i32 {
        self.dimensions.x * self.dimensions.y * self.dimensions.z
    }

    pub fn largest_dimension(&self) -> i32 {
        [self.dimensions.x, self.dimensions.y, self.dimensions.z]
            .iter()
            .max()
            .copied()
            .expect("non-empty array; qed")
    }

    pub fn smallest_side_perimeter(&self) -> i32 {
        2 * (self.dimensions.x + self.dimensions.y + self.dimensions.z - self.largest_dimension())
    }

    /// Return the ribbon requirement for this box
    ///
    /// Definted in the problem as the volume plus the perimeter of the smallest side.
    pub fn ribbon(&self) -> i32 {
        self.volume() + self.smallest_side_perimeter()
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let paper: i32 = parse::<GiftBox>(input)?
        .map(|gift_box| gift_box.paper())
        .sum();
    println!("total paper required: {}", paper);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let ribbon: i32 = parse::<GiftBox>(input)?
        .map(|gift_box| gift_box.ribbon())
        .sum();
    println!("total ribbon required: {}", ribbon);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_boxes() -> Vec<GiftBox> {
        let mut ret: Vec<GiftBox> = Vec::new();

        ret.push(
            GiftBox::new(1, 1, 1)
                .ok()
                .expect("Failed to construct GiftBox(1, 1, 1)"),
        );
        ret.push(
            GiftBox::new(2, 3, 4)
                .ok()
                .expect("Failed to construct GiftBox(2, 3, 4)"),
        );
        ret.push(
            GiftBox::new(1, 1, 10)
                .ok()
                .expect("Failed to construct GiftBox(1, 1, 10)"),
        );

        ret
    }

    #[test]
    fn test_surface_area() {
        let expected = vec![6, 52, 42];

        for (g, e) in get_boxes().iter().zip(expected) {
            assert_eq!(g.surface_area(), e);
        }
    }

    #[test]
    fn test_smallest_side() {
        let expected = vec![1, 6, 1];

        for (g, e) in get_boxes().iter().zip(expected) {
            assert_eq!(g.smallest_side(), e);
        }
    }

    #[test]
    fn test_paper() {
        let expected = vec![7, 58, 43];

        for (g, e) in get_boxes().iter().zip(expected) {
            assert_eq!(g.paper(), e);
        }
    }

    #[test]
    fn test_volume() {
        let expected = vec![1, 24, 10];

        for (g, e) in get_boxes().iter().zip(expected) {
            assert_eq!(g.volume(), e);
        }
    }

    #[test]
    fn test_smallest_side_perimeter() {
        let expected = vec![4, 10, 4];

        for (g, e) in get_boxes().iter().zip(expected) {
            assert_eq!(g.smallest_side_perimeter(), e);
        }
    }

    #[test]
    fn test_ribbon() {
        let expected = vec![5, 34, 14];

        for (g, e) in get_boxes().iter().zip(expected) {
            assert_eq!(g.ribbon(), e);
        }
    }
}
