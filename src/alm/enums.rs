use serde::{Deserialize, Serialize};
use crate::{
    cashflows::{cashflow::{Cashflow, Side}, traits::InterestAccrual},
    instruments::{
        fixedrateinstrument::FixedRateInstrument, floatingrateinstrument::FloatingRateInstrument,
        traits::Structure,
    },
    time::{date::Date, enums::Frequency},
    visitors::traits::HasCashflows,
};
use crate::utils::errors::Result;

#[derive(Clone)]
pub enum Instrument {
    FixedRateInstrument(FixedRateInstrument),
    FloatingRateInstrument(FloatingRateInstrument),
}

impl HasCashflows for Instrument {
    fn cashflows(&self) -> &[Cashflow] {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.cashflows(),
            Instrument::FloatingRateInstrument(fri) => fri.cashflows(),
        }
    }

    fn mut_cashflows(&mut self) -> &mut [Cashflow] {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.mut_cashflows(),
            Instrument::FloatingRateInstrument(fri) => fri.mut_cashflows(),
        }
    }
}


impl InterestAccrual for Instrument {
    fn accrual_start_date(&self) -> Date {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.accrual_start_date(),
            Instrument::FloatingRateInstrument(fri) => fri.accrual_start_date(),
        }
    }

    fn accrual_end_date(&self) -> Date {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.accrual_end_date(),
            Instrument::FloatingRateInstrument(fri) => fri.accrual_end_date(),
        }
    }

    fn accrued_amount (&self, start_date: Date, end_date: Date) -> Result<f64> {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.accrued_amount(start_date, end_date),
            Instrument::FloatingRateInstrument(fri) => fri.accrued_amount(start_date, end_date),
        }
    }
}

impl Instrument {
    pub fn notional(&self) -> f64 {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.notional(),
            Instrument::FloatingRateInstrument(fri) => fri.notional(),
        }
    }

    pub fn start_date(&self) -> Date {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.start_date(),
            Instrument::FloatingRateInstrument(fri) => fri.start_date(),
        }
    }

    pub fn end_date(&self) -> Date {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.end_date(),
            Instrument::FloatingRateInstrument(fri) => fri.end_date(),
        }
    }

    pub fn id(&self) -> Option<usize> {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.id(),
            Instrument::FloatingRateInstrument(fri) => fri.id(),
        }
    }

    pub fn structure(&self) -> Structure {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.structure(),
            Instrument::FloatingRateInstrument(fri) => fri.structure(),
        }
    }

    pub fn payment_frequency(&self) -> Frequency {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.payment_frequency(),
            Instrument::FloatingRateInstrument(fri) => fri.payment_frequency(),
        }
    }

    pub fn side(&self) -> Side {
        match self {
            Instrument::FixedRateInstrument(fri) => fri.side(),
            Instrument::FloatingRateInstrument(fri) => fri.side(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RateType {
    Fixed,
    Floating,
}
