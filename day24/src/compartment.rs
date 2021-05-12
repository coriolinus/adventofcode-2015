use std::convert::TryFrom;

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

impl TryFrom<usize> for Compartment {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Compartment::Footwell),
            1 => Ok(Compartment::LeftSaddle),
            2 => Ok(Compartment::RightSaddle),
            3 => Ok(Compartment::Trunk),
            _ => Err(()),
        }
    }
}
