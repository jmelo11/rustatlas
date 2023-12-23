# rustatlas

A rust version of Atlas

## Herramientas de Mercado

- [X] Indice Ibor/Overnight
- [X] Accrual Ibor/Overnight (cupones flotantes)
- [X] Curvas con base RF y spreads contantes/interpolados
- [ ] Curvas con modelos (nss, vasicek, etc)
- [X] Fixing en periodos distintos al indice
- [X] Shock de curvas

## Copones

- [X] Cashflow simple
- [X] Cupon tasa fija
- [X] Cupon tasa flotante/ibor

## Productos

- [ ] Loans:
  - [X] Fijos
    - [X] Bullet
    - [X] Amortizable
    - [X] Cero
    - [X] Cuotas iguales
    - [X] Irregular
  - [X] Flotantes
    - [X] Bullet
    - [X] Amortizable
    - [X] Cero
    - [X] Irregular
  - [ ] Mixtos
- [X] Cuentas corrientes
- [X] Depositos a plazo
- [ ] Bonos
- [ ] Swaps
- [ ] Opciones
- [ ] Forwards

## Vistantes

- [X] Tasas par
- [X] NPV
- [X] Fixings
- [X] Accrual
- [X] Agrupación
- [ ] Cashflows con transformacion de monedas
- [X] Zspread

## Simulación

- [X] Motor de rollover
- [X] Avanzar MarketStore en T+1

## Datos mercado

- [X] Cargar fixings de indices
- [X] Cargar monedas
- [ ] Curvas de tasas UF/Colateral CLP

## Time

- [X] Crear calendarios
  - [X] NullCalendar
  - [X] WeekendsOnly
  - [ ] Chile
  - [ ] USA

- [X] Crear fechas
- [X] Crear schedule

## Rust

- [X] Corregir panics/remplazar por errores
- [X] Unificar errores

### Incidencias

- [X] Fixings en fines de semana: se debe incorporar un interpolador
  - [X] Feature
  - [X] UT
- [X] Periodos de gracia en creditos
  - [X] Feature
  - [X] UT
- [ ] Convertir monedas de forma automatica
  - [ ] Feature
  - [ ] UT
