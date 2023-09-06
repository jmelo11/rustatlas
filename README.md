# rustatlas

A rust version of Atlas

## Herramientas de Mercado

- [X] Indice Ibor/Overnight
- [ ] Accrual Ibor/Overnight (cupones flotantes)
- [ ] Curvas con base RF y spreads contantes/interpolados
- [ ] Curvas con modelos (nss, vasicek, etc)
- [X] Fixing en periodos distintos al indice
- [ ] Shock de curvas

## Copones

- [X] Cashflow simple
- [X] Cupon tasa fija
- [X] Cupon tasa flotante/ibor

## Productos

- [ ] Loans:
  - [ ] Fijos
    - [X] Bullet
    - [ ] Amortizable
    - [ ] Cero
    - [X] Cuotas iguales
    - [ ] Irregular
  - [ ] Flotantes
    - [X] Bullet
    - [ ] Amortizable
    - [ ] Cero
    - [ ] Irregular
  - [ ] Mixtos
- [ ] Cuentas corrientes
- [ ] Swaps
- [ ] Opciones
- [ ] Forwards

## Vistantes

- [ ] Tasas par
- [ ] NPV
- [ ] Fixings
- [ ] Accrual
- [ ] Agrupación

## Python

- [ ] Revisar port a python / POC

## Simulación

- [ ] Motor de rollover
  - [ ] Balance constante
  - [ ] Balance dinamico
- [ ] Avanzar MarketStore en T+1

## Datos mercado

- [ ] Cargar fixings de indices
- [ ] Cargar monedas
- [ ] Curvas de tasas UF/Colateral CLP

## Time

- [X] Crear calendarios
  - [X] NullCalendar
  - [X] WeekendsOnly
  - [ ] Chile
  - [ ] USA

- [X] Crear fechas
- [X] Crear schedule

## Software

- [ ] Revisar metodologia paralelismo
- [ ] Revisar performance DB
