use std::ops::*;
use num_traits::*;

pub type Digit = u64;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BigInt {
    digits: Box<[Digit]>,
}

impl Add<&BigInt> for &BigInt {
    type Output = BigInt;

    fn add(self, other: &BigInt) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
