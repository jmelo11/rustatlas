use crate::{
    cashflows::{
        cashflow::{Cashflow, CashflowType},
        traits::Payable,
    },
    time::date::Date,
};

pub trait Visit<T> {
    type Output;
    fn visit(&self, visitable: &mut T) -> Self::Output;
}

pub trait ConstVisit<T> {
    type Output;
    fn visit(&self, visitable: &T) -> Self::Output;
}

pub trait HasCashflows {
    fn cashflows(&self) -> &[Cashflow];

    fn mut_cashflows(&mut self) -> &mut [Cashflow];

    fn set_discount_curve_id(&mut self, id: usize) {
        self.mut_cashflows()
            .iter_mut()
            .for_each(|cf| cf.set_discount_curve_id(id));
    }
    fn set_forecast_curve_id(&mut self, id: usize) {
        self.mut_cashflows().iter_mut().for_each(|cf| match cf {
            Cashflow::FloatingRateCoupon(frcf) => frcf.set_forecast_curve_id(id),
            _ => (),
        });
    }

    fn next_cashflow(&self, reference_date: Date, cashflow_type: CashflowType) -> Option<Cashflow> {
        match cashflow_type {
            CashflowType::Disbursement => self
                .cashflows()
                .iter()
                .filter(|cf| match cf {
                    Cashflow::Disbursement(_) => true,
                    _ => false,
                })
                .filter(|cf| cf.payment_date() > reference_date)
                .min_by(|cf1, cf2| cf1.payment_date().cmp(&cf2.payment_date()))
                .cloned(),
            CashflowType::Redemption => self
                .cashflows()
                .iter()
                .filter(|cf| match cf {
                    Cashflow::Redemption(_) => true,
                    _ => false,
                })
                .filter(|cf| cf.payment_date() > reference_date)
                .min_by(|cf1, cf2| cf1.payment_date().cmp(&cf2.payment_date()))
                .cloned(),
            CashflowType::FixedRateCoupon => self
                .cashflows()
                .iter()
                .filter(|cf| match cf {
                    Cashflow::FixedRateCoupon(_) => true,
                    _ => false,
                })
                .filter(|cf| cf.payment_date() > reference_date)
                .min_by(|cf1, cf2| cf1.payment_date().cmp(&cf2.payment_date()))
                .cloned(),

            CashflowType::FloatingRateCoupon => self
                .cashflows()
                .iter()
                .filter(|cf| match cf {
                    Cashflow::FloatingRateCoupon(_) => true,
                    _ => false,
                })
                .filter(|cf| cf.payment_date() > reference_date)
                .min_by(|cf1, cf2| cf1.payment_date().cmp(&cf2.payment_date()))
                .cloned(),
        }
    }
}
