use crate::cashflows::cashflow::Cashflow;

pub trait Visit<T, U> {
    type Output;
    fn visit(&self, visitable: &mut T) -> Self::Output;
}

pub trait ConstVisit<T, U> {
    type Output;
    fn visit(&self, visitable: &T) -> Self::Output;
}

pub trait HasCashflows {
    fn cashflows(&self) -> &[Cashflow];
    fn mut_cashflows(&mut self) -> &mut [Cashflow];
    fn set_discount_curve_id(&mut self, id: Option<usize>) {
        self.mut_cashflows()
            .iter_mut()
            .for_each(|cf| cf.set_discount_curve_id(id));
    }
    fn set_forecast_curve_id(&mut self, id: Option<usize>) {
        self.mut_cashflows().iter_mut().for_each(|cf| match cf {
            Cashflow::FloatingRateCoupon(frcf) => frcf.set_forecast_curve_id(id),
            _ => (),
        });
    }
}
