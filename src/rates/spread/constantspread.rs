
use crate::{
    rates::traits::Spread,
    time::{date::Date},
};


pub struct ConstantSpread {
    spread: f64,
}

impl ConstantSpread {
    pub fn new(spread: f64) -> ConstantSpread {
        ConstantSpread {
            spread,
        }
    }

    pub fn spread(&self) -> f64 {
        return self.spread;
    }

}

impl Spread<ConstantSpread> for ConstantSpread {
    fn return_spread_to_date(&self, year_fraction: f64) -> f64 {
        return self.spread;
    }
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_constant_spread() {
        let spread = ConstantSpread::new(0.01);
        assert_eq!(spread.spread(), 0.01);
    }

}
