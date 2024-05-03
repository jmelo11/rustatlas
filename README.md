RustAtlas
=========

**RustAtlas** is a high-performance quantitative finance library written in Rust, designed for precision and speed in financial calculations.

Usage Example
-------------

The following example shows how to create a fixed rate instrument with a notional of $1,000.0, a 5% annual rate, and monthly payments:

```rust
        use rustatlas::prelude::*;
        
        // Define basic parameters
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(2, TimeUnit::Months);
        let rate = InterestRate::new(
            0.05,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let notional = 1_000.0;

        // Create a fixed rate instrument
        let instrument = MakeFixedRateInstrument::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_payment_frequency(Frequency::Monthly)
            .with_rate(rate)
            .with_notional(notional)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .equal_payments()
            .build()?;

        assert_eq!(instrument.notional(), notional);
        assert_eq!(instrument.rate(), rate);
        assert_eq!(instrument.payment_frequency(), Frequency::Monthly);
        assert_eq!(instrument.start_date(), start_date);
        assert_eq!(instrument.end_date(), end_date);

        // Show cashflows
        instrument
            .cashflows()
            .iter()
            .for_each(|cf| println!("{}", cf));
```

Various financial instruments are available, such as loans, bonds, swaps, and options. The library also includes tools for market data, analytics, and simulation. Pricing can be achieved through visitors. See the example below:

```rust
        use rustatlas::prelude::*;

        let market_store = create_store(); // Auxiliary function (see examples folder)
        let ref_date = market_store.reference_date();

        let start_date = ref_date;
        let end_date = start_date + Period::new(10, TimeUnit::Years);
        let notional = 100_000.0;
        let rate = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Thirty360,
        );

        let mut instrument = MakeFixedRateInstrument::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_rate(rate)
            .with_payment_frequency(Frequency::Semiannual)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .bullet()
            .with_discount_curve_id(Some(2))
            .with_notional(notional)
            .build()?;

        let indexer = IndexingVisitor::new();
        indexer.visit(&mut instrument)?;

        let model = SimpleModel::new(&market_store);
        let data = model.gen_market_data(&indexer.request())?;

        let npv_visitor = NPVConstVisitor::new(&data, true);
        let npv = npv_visitor.visit(&instrument)?;
        assert_ne!(npv, 0.0);
```

For more examples, see the [examples](examples) folder.

Features
--------

### Market Tools

* **Ibor/Overnight Indices** - Implemented
* **Accrual for Ibor/Overnight** (floating coupons) - Implemented
* **Interest Rate Curves**
  * Risk-free basis and constant/interpolated spreads - Implemented
  * Curves with models (Nelson-Siegel-Svensson, Vasicek, etc.) - Planned
* **Fixing Period Adjustments** - Implemented
* **Curve Shock Analysis** - Implemented

### Coupons

* **Simple Cashflow** - Implemented
* **Fixed Rate Coupon** - Implemented
* **Floating Rate/Ibor Coupon** - Implemented

### Financial Products

* **Loans**
  * Fixed:
    * Bullet - Implemented
    * Amortizing - Implemented
    * Zero-coupon - Implemented
    * Equal installments - Implemented
    * Irregular - Implemented
  * Floating:
    * Bullet - Implemented
    * Amortizing - Implemented
    * Zero-coupon - Implemented
    * Irregular - Implemented
  * Mixed - Planned
* **Current Accounts** - Implemented
* **Time Deposits** - Implemented
* **Bonds** - Implemented
* **Swaps** - Implemented
* **Options** - Planned
* **Forwards** - Planned

### Analytics

* **Par Rates** - Implemented
* **Net Present Value (NPV)** - Implemented
* **Fixings** - Implemented
* **Accrual Calculations** - Implemented
* **Grouping** - Implemented
* **Currency Transformation in Cashflows** - Implemented
* **Zero Spread (Z-spread)** - Implemented

### Simulation Tools

* **Rollover Engine** - Implemented
* **Advance MarketStore to T+1** - Implemented

### Market Data

* **Load Index Fixings** - Implemented
* **Load Currencies** - Implemented
* **Interest Rate Curves for UF/Collateral CLP** - Implemented

### Time Utilities

* **Calendar Creation**
  * NullCalendar - Implemented
  * WeekendsOnly - Implemented
  * Chile - Planned
  * USA - Implemented
* **Date Creation** - Implemented
* **Schedule Creation** - Implemented

### Rust-specific Improvements

* **Panic Handling and Error Replacement** - Implemented
* **Error Unification** - Implemented
* **Compile time automatic differentiation** - Planned (see <https://github.com/rust-lang/rust/issues/124509>)

### Issue Tracking

* **Weekend Fixings**
  * Feature and Unit Tests (UT) - Implemented
* **Grace Periods in Loans**
  * Feature and Unit Tests (UT) - Implemented
* **Automatic Currency Conversion**
  * Feature and Unit Tests (UT) - Implemented

Contributing
------------

Contributions to RustAtlas are welcome! If you have suggestions for improvements or have identified issues, please open an issue or submit a pull request.

License
-------

RustAtlas is released under the MIT License. Details can be found in the [LICENSE](LICENSE) file.

Contact
-------

For more details or business inquiries, please contact <jmelo@live.cl>.
