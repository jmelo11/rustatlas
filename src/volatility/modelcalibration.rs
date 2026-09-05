use serde::{Deserialize, Serialize};

use crate::{indices::marketindex::MarketIndex, volatility::volatilityindexing::Strike};

/// Specifies which market data object to calibrate against.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CalibrationSource {
    /// Calibrate to caplet vols from a 2-D volatility surface.
    Surface {
        /// Market index identifying the surface to read from.
        market_index: MarketIndex,
    },
    /// Calibrate to swaption vols from a 3-D volatility cube.
    Cube {
        /// Market index identifying the cube to read from.
        market_index: MarketIndex,
    },
}

/// Configuration for model calibration (e.g. Hull-White to caplet/swaption vols).
///
/// Quote identifiers follow the same convention as
/// [`CurveConfiguration`](crate::rates::bootstrapping::curveconfiguration::CurveConfiguration):
/// each string is resolved against a [`QuoteSelector`](crate::quotes::quoteselector::QuoteSelector)
/// to obtain the market quote and instrument details.
///
/// # Strike / moneyness resolution
///
/// Market strikes are generally not known when the configuration is written,
/// so `quote_ids` can simply list **all available market quotes** (multiple
/// strikes per expiry). When a [`strike`](Self::strike) specification is
/// provided, the system resolves the calibration basket itself:
///
/// * quotes are collapsed to one calibration instrument per (expiry, tenor)
///   pillar, and
/// * each instrument's strike is replaced by the specification —
///   [`Strike::Atm`], [`Strike::Relative`] (e.g. ATM + 2%), or
///   [`Strike::Absolute`] — which curve-aware models (Hull-White, LGM)
///   resolve against the forward rate at each pillar before sampling the
///   surface/cube.
///
/// Without a `strike`, each quote's own strike is used verbatim (one
/// calibration instrument per quote id).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelCalibrationConfiguration {
    /// Which vol surface or cube to calibrate against.
    source: CalibrationSource,
    /// Quote identifiers for the calibration instruments (caplets, swaptions, or both).
    quote_ids: Vec<String>,
    /// Optional strike/moneyness override applied to every calibration
    /// instrument (deduplicating expiry pillars).
    #[serde(default)]
    strike: Option<Strike>,
    /// Mean-reversion speed.
    alpha: f64,
}

impl ModelCalibrationConfiguration {
    /// Creates a new calibration configuration.
    #[must_use]
    pub const fn new(source: CalibrationSource, quote_ids: Vec<String>, alpha: f64) -> Self {
        Self {
            source,
            quote_ids,
            strike: None,
            alpha,
        }
    }

    /// Sets the strike/moneyness override.
    #[must_use]
    pub const fn with_strike(mut self, strike: Strike) -> Self {
        self.strike = Some(strike);
        self
    }

    /// Returns the calibration source.
    #[must_use]
    pub const fn source(&self) -> &CalibrationSource {
        &self.source
    }

    /// Returns the calibration quote identifiers.
    #[must_use]
    pub fn quote_ids(&self) -> &[String] {
        &self.quote_ids
    }

    /// Returns the strike/moneyness override, if any.
    #[must_use]
    pub const fn strike(&self) -> Option<Strike> {
        self.strike
    }

    /// Returns the mean-reversion speed.
    #[must_use]
    pub const fn alpha(&self) -> f64 {
        self.alpha
    }
}
