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
//!
//! # Code Examples

#[derive(PartialEq, Eq, Debug)]
pub struct GiftBox {
    // dimensions
    _x: i32,
    _y: i32,
    _z: i32,

    // surface areas
    xy: i32,
    xz: i32,
    yz: i32,
}

impl GiftBox {
    /// Construct a new GiftBox
    pub fn new(x: i32, y: i32, z: i32) -> Result<GiftBox, &'static str> {
        if x > 0 && y > 0 && z > 0 {
            Ok(GiftBox {
                _x:  x,
                _y:  y,
                _z:  z,
                xy: x*y,
                xz: x*z,
                yz: y*z,
            })
        } else {
            Err("Can't construct a box with dimension 0 or less!")
        }
    }

    /// Parse a new GiftBox from a string of the type "1x2x3"
    ///
    /// Formally, this expects 3 integers separated by two x's, with no spaces or other characters.
    pub fn parse(input: &str) -> Result<GiftBox, &'static str> {
        // split on 'x'
        let ivec: Vec<&str> = input.split('x').collect();
        if ivec.len() != 3 {return Err("input did not contain two 'x' chars; could not parse");}

        let x = ivec[0].parse::<i32>();
        let y = ivec[1].parse::<i32>();
        let z = ivec[2].parse::<i32>();

        if let (Ok(x), Ok(y), Ok(z)) = (x, y, z) {
            GiftBox::new(x, y, z)
        } else {
            Err("Failed to parse input as i32")
        }
    }

    /// Return the surface area.
    pub fn surface_area(&self) -> i32 {
        2 * (self.xz + self.xy + self.yz)
    }

    /// Return the margin: the area of the smallest side
    pub fn smallest_side(&self) -> i32 {
        std::cmp::min(self.xy, std::cmp::min(self.xz, self.yz))
    }

    /// Return the paper requirement for this box
    pub fn paper(&self) -> i32 {
        self.surface_area() + self.smallest_side()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_boxes() -> Vec<GiftBox> {
        let mut ret: Vec<GiftBox> = Vec::new();

        ret.push(GiftBox::new(1, 1, 1).ok().expect("Failed to construct GiftBox(1, 1, 1)")  );
        ret.push(GiftBox::new(2, 3, 4).ok().expect("Failed to construct GiftBox(2, 3, 4)")  );
        ret.push(GiftBox::new(1, 1, 10).ok().expect("Failed to construct GiftBox(1, 1, 10)"));

        ret
    }

    #[test]
    fn test_parse() {
        let inputs = vec!["1x1x1", "2x3x4", "1x1x10"];

        for (g, i) in get_boxes().iter().zip(inputs) {
            assert_eq!(g, &GiftBox::parse(i).ok().unwrap());
        }
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
}
