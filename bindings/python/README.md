# quantsupport (Python bindings)

PyO3 bindings for the [quantsupport](https://github.com/jmelo11/quantsupport) Rust
derivative pricing and risk analytics library.

## Install (development)

```bash
pip install maturin
maturin develop -m bindings/python/Cargo.toml --release
```

## Usage

Market data and configurations are typed objects created from dicts
(`from_dict`) or JSON files (`from_json`), deserialized directly by the Rust
library's serde implementations. The `PricingContext` is a context manager:
entering the `with` block bootstraps curves, volatility surfaces/cubes and
simulations, and starts the AD tape; exiting releases all constructed market
data and rewinds the tape.

```python
import quantsupport as qs

quotes = qs.QuoteStore.from_json("quotes.json")
curves = qs.CurveConfiguration.from_json("curve_specs.json")   # list
fx = qs.FxStore.from_dict([{"base": "CLP", "quote": "USD", "rate": 1 / 900}])
discounting = qs.DiscountingConfig(currency=qs.Currency.USD, index=qs.MarketIndex.SOFR)

ref = quotes.reference_date            # qs.Date
swap = qs.Swap(
    identifier="USD_IRS_5Y",
    start_date=ref,
    maturity_date=ref + "5Y",
    notional=10_000_000.0,
    fixed_rate=0.0378,
    currency=qs.Currency.USD,
    market_index=qs.MarketIndex.SOFR,
    side=qs.Side.LongReceive,
)

with qs.PricingContext(quotes=quotes, curves=curves, fx=fx, discounting=discounting) as ctx:
    # explore the bootstrapped market data
    sofr = ctx.curve(qs.MarketIndex.SOFR)
    print(sofr.nodes())                 # DataFrame: date / discount_factor
    print(sofr.discount_factor(ref + "5Y"))
    print(sofr.forward_rate(ref + "1Y", ref + "2Y",
                            qs.Compounding.Simple, qs.Frequency.Annual))

    res = ctx.evaluate(swap, [qs.Request.Value, qs.Request.Cashflows, qs.Request.Sensitivities])
    print(res.price)            # NPV (float)
    print(res.sensitivities)    # pandas DataFrame: pillar / value
    print(res.cashflows)        # pandas DataFrame
```

Every enum argument also accepts its string name (`currency="USD"`,
`side="pay"`, `requests=["Value"]`, ...).

### XVA

The engine configuration covers only the simulation setup (models, paths,
seed, frequency). Collateral treatment and credit/funding conditions are per
client: each `NettingSet` carries its own `CsaTerms`.

```python
config = qs.XvaConfig.from_json("xva_config.json")

csa_a = qs.CsaTerms(collateral_index="SOFR", collateral_currency="USD",
                    credit_spread=0.010, recovery=0.40, funding_spread=0.005)
csa_b = qs.CsaTerms.from_json("client_b_csa.json")

with qs.PricingContext(quotes=quotes, curves=curves, fx=fx, discounting=discounting) as ctx:
    result = ctx.run_xva(config, netting_sets=[
        qs.NettingSet("clientA", [swap], csa_a),
        qs.NettingSet("clientB", [xccy], csa_b),
    ])
    print(result.xva_values)      # DataFrame: netting_set / measure / value
    print(result.sensitivities)   # DataFrame: parameter / value
    for p in result.exposures:    # per-netting-set EPE/ENE/EE profiles
        print(p.netting_set)
        print(p.to_dataframe())   # DataFrame: date / epe / ene / ee
```

See [examples/tour.ipynb](examples/tour.ipynb) for a guided notebook covering
all components (dates, enums, market data, curve exploration, pricing, XVA).

## API surface (v1)

Enums (typed classes; every argument also accepts the string name, and each
enum has a case-insensitive `parse`):

- `Currency` — with ISO metadata (`code`, `name`, `symbol`, `precision`, `numeric_code`)
- `MarketIndex` — rate-index constants (`MarketIndex.SOFR`, ...) plus factories
  `equity(name)`, `fx_pair(base, quote)`, `collateral(base, quote)`, `other(name)`
- `Side`, `Compounding`, `Frequency`, `DayCounter`, `TimeUnit`,
  `BusinessDayConvention`, `Request`, `VolatilityType`, `SmileType`

Time:

- `Date(y, m, d)` / `Date.parse(s)` — arithmetic with days, `Period` or period
  strings (`date + "6M"`), `weekday()`, `end_of_month()`, `to_datetime()`
- `Period(n, TimeUnit)` / `Period.parse("1Y6M")` / `Period.from_frequency(f)`
- `Calendar(name)` — `is_business_day`, `adjust`, `advance`,
  `business_days_between`, `holiday_list`, `business_day_list`
- `DayCounter.year_fraction(start, end)` / `day_count(start, end)`

Market data & configuration (each with `from_dict` / `from_json`, plus
`to_dict` on configurations):

- `QuoteStore` — quotes with a reference date (`reference_date`,
  `identifiers()`, `to_dataframe()`)
- `FixingStore` — historical index fixings
- `FxStore` — FX spot rates (also `FxStore()` + `.add(base, quote, rate)`)
- `CurveConfiguration`, `VolatilitySurfaceConfiguration`,
  `VolatilityCubeConfiguration`, `SimulationConfiguration`
- `DiscountingConfig(currency, index)` — base-curve definition of the context

Pricing:

- `PricingContext(quotes, curves, fixings=None, fx=None, volatility_surfaces=None, volatility_cubes=None, simulations=None, discounting=None)` — context manager
- inputs: `ctx.reference_date`, `ctx.quotes`, `ctx.curve_configurations`,
  `ctx.volatility_surface_configurations`, `ctx.volatility_cube_configurations`,
  `ctx.simulation_configurations`
- constructed outputs (after `initialize()` / inside the `with` block):
  - `ctx.curves()` / `ctx.curve(index)` → `DiscountCurve`
    (`nodes()`, `pillars()`, `discount_factor(date)`, `forward_rate(...)`)
  - `ctx.volatility_surfaces()` / `ctx.volatility_surface(index)` → `VolatilitySurface`
  - `ctx.volatility_cubes()` / `ctx.volatility_cube(index)` → `VolatilityCube`
  - `ctx.simulations()` / `ctx.simulation(index)` → `Simulation`
    (`dates()`, `n_paths`, `dt`, `paths()` DataFrame)
- `Swap`, `CrossCurrencySwap` — trade specifications with typed getters
- `ctx.evaluate(trade, requests)` → `EvaluationResults` (`price`, `fair_rate`, `sensitivities` DataFrame, `cashflows` DataFrame)

XVA:

- `XvaConfig` — simulation/model setup only
- `CsaTerms` — per-client collateral treatment + credit/funding parameters
- `NettingSet(name, trades, csa)` — one client's trades under its CSA
- `ctx.run_xva(config, netting_sets)` → `XvaResult` (`xva_values` DataFrame, `sensitivities` DataFrame, `exposures`)
- `QuantSupportError` — exception raised on library errors

All JSON schemas match the Rust examples in `examples/*/data/`. More
instrument types can be added in `bindings/python/src/trades.rs` following the
existing pattern.
