RustAtlas
=========

**RustAtlas** is a high-performance quantitative finance library written in Rust, designed for precision and speed in financial calculations.

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
