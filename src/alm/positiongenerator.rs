use crate::{
    cashflows::cashflow::Side,
    currencies::enums::Currency,
    instruments::{
        fixedrateinstrument::FixedRateInstrument, floatingrateinstrument::FloatingRateInstrument,
        makefixedrateloan::MakeFixedRateLoan, makefloatingrateloan::MakeFloatingRateLoan,
        traits::Structure,
    },
    rates::interestrate::InterestRate,
    time::{date::Date, enums::Frequency, period::Period},
};

pub enum Instrument {
    FixedRateInstrument(FixedRateInstrument),
    FloatingRateInstrument(FloatingRateInstrument),
}

#[derive(Debug, Clone, Copy)]
pub enum RateType {
    Fixed,
    Floating,
}

pub struct PositionGenerator {
    amount: f64,
    start_date: Date,
    configs: Vec<PositionConfig>,
}

pub struct PositionConfig {
    weight: f64,
    structure: Structure,
    payment_frequency: Frequency,
    tenor: Period,
    currency: Currency,
    side: Side,
    rate_type: RateType,
    rate: Option<InterestRate>,
    spread: Option<f64>,
}

impl PositionConfig {
    pub fn new(
        weight: f64,
        structure: Structure,
        payment_frequency: Frequency,
        tenor: Period,
        currency: Currency,
        side: Side,
        rate_type: RateType,
    ) -> PositionConfig {
        PositionConfig {
            weight,
            structure,
            payment_frequency,
            tenor,
            currency,
            side,
            rate_type,
            rate: None,
            spread: None,
        }
    }

    pub fn with_rate(mut self, rate: InterestRate) -> PositionConfig {
        self.rate = Some(rate);
        self
    }

    pub fn with_spread(mut self, spread: f64) -> PositionConfig {
        self.spread = Some(spread);
        self
    }

    pub fn rate(&self) -> Option<InterestRate> {
        self.rate
    }

    pub fn spread(&self) -> Option<f64> {
        self.spread
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }

    pub fn structure(&self) -> Structure {
        self.structure
    }

    pub fn payment_frequency(&self) -> Frequency {
        self.payment_frequency
    }

    pub fn tenor(&self) -> Period {
        self.tenor
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn rate_type(&self) -> RateType {
        self.rate_type
    }
}

impl PositionGenerator {
    pub fn new(amount: f64, start_date: Date) -> PositionGenerator {
        PositionGenerator {
            amount,
            start_date,
            configs: Vec::new(),
        }
    }

    pub fn with_amount(mut self, amount: f64) -> PositionGenerator {
        self.amount = amount;
        self
    }

    pub fn with_start_date(mut self, start_date: Date) -> PositionGenerator {
        self.start_date = start_date;
        self
    }

    pub fn with_configs(mut self, configs: Vec<PositionConfig>) -> PositionGenerator {
        self.configs = configs;
        self
    }

    pub fn generate_position(&self, config: &PositionConfig) -> Instrument {
        let structure = config.structure();
        let notional = self.amount * config.weight();

        match config.rate_type() {
            RateType::Floating => {
                let instrument =
                    MakeFloatingRateLoan::new(self.start_date, self.start_date + config.tenor())
                        .with_notional(notional)
                        .build();
                Instrument::FloatingRateInstrument(instrument)
            }
            RateType::Fixed => {
                let rate = config.rate().unwrap();
                let builder = MakeFixedRateLoan::new()
                    .with_notional(notional)
                    .with_start_date(self.start_date)
                    .with_tenor(config.tenor())
                    .with_structure(structure);
                if config.rate().is_some() {
                    Instrument::FixedRateInstrument(builder.with_rate(rate).build())
                } else {
                    Instrument::FixedRateInstrument(builder.build())
                }
            }
        }
    }

    pub fn generate(&self) -> Vec<Instrument> {
        let positions = self
            .configs
            .iter()
            .map(|config| self.generate_position(config))
            .collect();
        return positions;
    }
}
