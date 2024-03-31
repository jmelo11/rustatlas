use std::{cell::RefCell, collections::HashMap, hash::Hash};

use crate::{
    cashflows::{
        cashflow::Cashflow,
        traits::{InterestAccrual, Payable},
    },
    core::traits::{HasCurrency, HasDiscountCurveId, HasForecastCurveId},
    currencies::enums::Currency,
    instruments::fixedrateinstrument::FixedRateInstrument,
    rates::interestrate::{InterestRate, RateDefinition},
    time::date::Date,
};

use super::traits::{ConstVisit, HasCashflows};
use crate::utils::errors::Result;

/// # SimpleCashlowGroup
/// Struct that defines a cashflow group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleCashlowGroup {
    pub currency: Currency,
    pub discount_curve_id: Option<usize>,
    pub payment_date: Date,
}

impl Hash for SimpleCashlowGroup {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.currency.hash(state);
        self.discount_curve_id.hash(state);
    }
}

/// # FixedRateCashflowGroup
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixedRateCashflowGroup {
    pub currency: Currency,
    pub accrual_start_date: Date,
    pub accrual_end_date: Date,
    pub discount_curve_id: usize,
    pub rate_definition: RateDefinition,
}

impl Hash for FixedRateCashflowGroup {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.currency.hash(state);
        self.accrual_start_date.hash(state);
        self.accrual_end_date.hash(state);
        self.discount_curve_id.hash(state);
        self.rate_definition.hash(state);
    }
}

/// # FloatingRateCashflowGroup
/// Struct that defines a floating rate cashflow group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FloatingRateCashflowGroup {
    pub currency: Currency,
    pub accrual_start_date: Date,
    pub accrual_end_date: Date,
    pub fixing_date: Date,
    pub discount_curve_id: usize,
    pub forecast_curve_id: usize,
    pub rate_definition: RateDefinition,
}

impl Hash for FloatingRateCashflowGroup {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.currency.hash(state);
        self.accrual_start_date.hash(state);
        self.accrual_end_date.hash(state);
        self.fixing_date.hash(state);
        self.discount_curve_id.hash(state);
        self.forecast_curve_id.hash(state);
        self.rate_definition.hash(state);
    }
}

/// # CashflowCompressorConstVisitor
/// This visitor is used to compress cashflows into groups to reduce the number of cashflows that need to be processed.
pub struct CashflowCompressorConstVisitor {
    disbursements: RefCell<HashMap<SimpleCashlowGroup, Cashflow>>,
    redemptions: RefCell<HashMap<SimpleCashlowGroup, Cashflow>>,
    fixed_rate_coupons: RefCell<HashMap<FixedRateCashflowGroup, Cashflow>>,
    floating_rate_coupons: RefCell<HashMap<FloatingRateCashflowGroup, Cashflow>>,
}

impl CashflowCompressorConstVisitor {
    pub fn new() -> Self {
        Self {
            disbursements: RefCell::new(HashMap::new()),
            redemptions: RefCell::new(HashMap::new()),
            fixed_rate_coupons: RefCell::new(HashMap::new()),
            floating_rate_coupons: RefCell::new(HashMap::new()),
        }
    }

    pub fn compress() {}
}

impl ConstVisit<FixedRateInstrument> for CashflowCompressorConstVisitor {
    type Output = Result<()>;

    fn visit(&self, visitable: &FixedRateInstrument) -> Self::Output {
        visitable
            .cashflows()
            .iter()
            .try_for_each(|&cf| -> Result<()> {
                match cf {
                    Cashflow::Disbursement(disbursement) => {
                        let group = SimpleCashlowGroup {
                            currency: disbursement.currency()?,
                            discount_curve_id: Some(disbursement.discount_curve_id()?),
                            payment_date: disbursement.payment_date(),
                        };
                        let mut disbursements = self.disbursements.borrow_mut();
                        disbursements
                            .entry(group)
                            .and_modify(|pos| {
                                if let Cashflow::Disbursement(pos) = pos {
                                    pos.set_amount(
                                        pos.amount().unwrap() + disbursement.amount().unwrap(),
                                    );
                                }
                            })
                            .or_insert(Cashflow::Disbursement(disbursement.clone()));
                    }
                    Cashflow::Redemption(redemption) => {
                        let group = SimpleCashlowGroup {
                            currency: redemption.currency()?,
                            discount_curve_id: Some(redemption.discount_curve_id()?),
                            payment_date: redemption.payment_date(),
                        };
                        let mut redemptions = self.redemptions.borrow_mut();
                        redemptions
                            .entry(group)
                            .and_modify(|pos| {
                                if let Cashflow::Redemption(pos) = pos {
                                    pos.set_amount(
                                        pos.amount().unwrap() + redemption.amount().unwrap(),
                                    );
                                }
                            })
                            .or_insert(Cashflow::Redemption(redemption.clone()));
                    }
                    Cashflow::FixedRateCoupon(cf) => {
                        let group = FixedRateCashflowGroup {
                            currency: cf.currency()?,
                            accrual_start_date: cf.accrual_start_date().unwrap(),
                            accrual_end_date: cf.accrual_end_date().unwrap(),
                            discount_curve_id: cf.discount_curve_id()?,
                            rate_definition: cf.rate().rate_definition(),
                        };
                        let mut fixed_rate_coupons = self.fixed_rate_coupons.borrow_mut();
                        fixed_rate_coupons
                            .entry(group)
                            .and_modify(|pos| {
                                if let Cashflow::FixedRateCoupon(pos) = pos {
                                    let interest = pos.amount().unwrap() + cf.amount().unwrap();
                                    let notional = pos.notional() + cf.notional();
                                    let compound_factor = (notional + interest) / notional;
                                    let t = cf.rate().day_counter().year_fraction(
                                        cf.accrual_start_date().unwrap(),
                                        cf.accrual_end_date().unwrap(),
                                    );
                                    let new_rate = InterestRate::implied_rate(
                                        compound_factor,
                                        cf.rate().rate_definition().day_counter(),
                                        cf.rate().rate_definition().compounding(),
                                        cf.rate().rate_definition().frequency(),
                                        t,
                                    )
                                    .unwrap();
                                    pos.set_rate(new_rate);
                                    pos.set_notional(notional);
                                }
                            })
                            .or_insert(Cashflow::FixedRateCoupon(cf.clone()));
                    }
                    Cashflow::FloatingRateCoupon(cf) => {
                        let group = FloatingRateCashflowGroup {
                            currency: cf.currency()?,
                            accrual_start_date: cf.accrual_start_date().unwrap(),
                            accrual_end_date: cf.accrual_end_date().unwrap(),
                            fixing_date: cf.fixing_date(),
                            discount_curve_id: cf.discount_curve_id()?,
                            forecast_curve_id: cf.forecast_curve_id()?,
                            rate_definition: cf.rate_definition(),
                        };
                        let mut floating_rate_coupons = self.floating_rate_coupons.borrow_mut();
                        floating_rate_coupons
                            .entry(group)
                            .and_modify(|pos| {
                                if let Cashflow::FloatingRateCoupon(pos) = pos {
                                    let interest = pos.amount().unwrap() + cf.amount().unwrap();
                                    let notional = pos.notional() + cf.notional();
                                    let compound_factor = (notional + interest) / notional;
                                    let t = cf.rate_definition().day_counter().year_fraction(
                                        cf.accrual_start_date().unwrap(),
                                        cf.accrual_end_date().unwrap(),
                                    );
                                    let new_rate = InterestRate::implied_rate(
                                        compound_factor,
                                        cf.rate_definition().day_counter(),
                                        cf.rate_definition().compounding(),
                                        cf.rate_definition().frequency(),
                                        t,
                                    )
                                    .unwrap();
                                    pos.set_spread(new_rate.rate());
                                    pos.set_notional(notional);
                                }
                            })
                            .or_insert(Cashflow::FloatingRateCoupon(cf.clone()));
                    }
                }
                Ok(())
            })?;
        Ok(())
    }
}
