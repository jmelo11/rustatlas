use crate::{
    rates::{
        enums::Compounding,
        traits::{HasReferenceDate, YieldProvider, YieldProviderError},
    },
    time::{date::Date, enums::Frequency},
};

#[derive(Clone)]
pub struct SpreadedTermStructure<T: YieldProvider, U: YieldProvider> {
    date_reference: Date, // reference_date
    spread_curve: T,
    base_curve: U,
}

impl<T: YieldProvider, U: YieldProvider> SpreadedTermStructure<T, U> {
    pub fn new(spread_curve: T, base_curve: U) -> SpreadedTermStructure<T, U> {
        SpreadedTermStructure {
            date_reference: base_curve.reference_date(),
            spread_curve,
            base_curve,
        }
    }

    pub fn spread_curve(&self) -> &T {
        return &self.spread_curve;
    }

    pub fn base_curve(&self) -> &U {
        return &self.base_curve;
    }
}

impl<T: YieldProvider, U: YieldProvider> HasReferenceDate for SpreadedTermStructure<T, U> {
    fn reference_date(&self) -> Date {
        return self.date_reference;
    }
}

impl<T: YieldProvider, U: YieldProvider> YieldProvider for SpreadedTermStructure<T, U> {
    fn discount_factor(&self, date: Date) -> Result<f64, YieldProviderError> {
        let spread_discount_factor = self.spread_curve.discount_factor(date)?;
        let base_discount_factor = self.base_curve.discount_factor(date)?;

        //let add_df = 1.0/(1.0/spread_discount_factor + 1.0/base_discount_factor -1.0);
        let add_df= spread_discount_factor*base_discount_factor;

        return Ok(add_df);
    }

    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> Result<f64, YieldProviderError> {
        let spread_forward_rate = self.spread_curve.forward_rate(start_date, end_date, comp, freq)?;
        let base_forward_rate = self.base_curve.forward_rate(start_date, end_date, comp, freq)?;
        return Ok(spread_forward_rate + base_forward_rate);
    }

}

mod test {
    use crate::{
        rates::{
            interestrate::InterestRate,
            yieldtermstructure::flatforwardtermstructure::FlatForwardTermStructure, enums::Compounding,
        },
        time::{date::Date, daycounter::DayCounter, enums::Frequency}, prelude::{HasReferenceDate, YieldProvider},
    };

    use super::SpreadedTermStructure;

    
    #[test]
    fn test_reference_date() {
        let spread_curve = FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            InterestRate::new(
                0.01,
                Compounding::Compounded,
                Frequency::Annual,
                DayCounter::Actual360,
            ),
        );

        let base_curve = FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            InterestRate::new(
                0.02,
                Compounding::Compounded,
                Frequency::Annual,
                DayCounter::Actual360,
            ),
        );
        let spreaded_curve = SpreadedTermStructure::new(spread_curve, base_curve);
        assert!(spreaded_curve.reference_date() == Date::new(2020, 1, 1));

    }

    #[test]
    fn test_forward_rate(){

        let spread_curve = FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            InterestRate::new(
                0.01,
                Compounding::Compounded,
                Frequency::Annual,
                DayCounter::Actual360,
            ),
        );

        let base_curve = FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            InterestRate::new(
                0.02,
                Compounding::Compounded,
                Frequency::Annual,
                DayCounter::Actual360,
            ),
        );
        let spreaded_curve = SpreadedTermStructure::new(spread_curve, base_curve);
        
        let fr = spreaded_curve.forward_rate(Date::new(2020, 1, 1), Date::new(2022, 1, 1), Compounding::Compounded, Frequency::Annual);
        println!("fr: {:?}", fr);
        assert_eq!(fr.unwrap(),0.03);

        
    }

    #[test]
    fn test_discount_factor(){
            let spread_curve = FlatForwardTermStructure::new(
                Date::new(2020, 1, 1),
                InterestRate::new(
                    0.01,
                    Compounding::Compounded,
                    Frequency::Annual,
                    DayCounter::Actual360,
                ),
            );
    
            let base_curve = FlatForwardTermStructure::new(
                Date::new(2020, 1, 1),
                InterestRate::new(
                    0.02,
                    Compounding::Compounded,
                    Frequency::Annual,
                    DayCounter::Actual360,
                ),
            );
            let spreaded_curve = SpreadedTermStructure::new(spread_curve, base_curve);
            
            let df = spreaded_curve.discount_factor(Date::new(2021, 1, 1));
            println!("df: {:?}", df);

            assert_eq!(df.unwrap(), 0.9702040771633191);


    }
}