use serde::{Deserialize, Serialize};

use crate::{
    cashflows::cashflow::Cashflow,
    instruments::{
        fixedrateinstrument::FixedRateInstrument, floatingrateinstrument::FloatingRateInstrument,
    },
    visitors::traits::HasCashflows, time::date::Date,
};

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
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RateType {
    Fixed,
    Floating,
}
