#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Compartment {
    Footwell,
    LeftSaddle,
    RightSaddle,
    Trunk,
}

impl Default for Compartment {
    fn default() -> Self {
        Compartment::Footwell
    }
}
