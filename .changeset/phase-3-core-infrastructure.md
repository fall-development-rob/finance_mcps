---
"@corp-finance/bindings": minor
"@corp-finance/mcp-server": minor
---

Phase 3: Core Infrastructure & PE Tools Expansion

## New Core Infrastructure

- **circular_solver**: Iterative convergence for circular references (interest ↔ debt ↔ cash)
  - Fixed-point iteration algorithm
  - Newton-Raphson method for faster convergence
  - Specialized cash/interest circular solver

- **time_value**: Time value of money calculations
  - NPV (Net Present Value)
  - IRR (Internal Rate of Return) using Newton-Raphson
  - XIRR (IRR with irregular periods and actual dates)
  - MOIC (Multiple on Invested Capital)
  - MOIC to IRR approximation

## New Fundamentals

- **sources_uses**: Transaction structuring
  - Build Sources & Uses tables for M&A/LBO transactions
  - Automatic percentage calculations
  - Balance checking (sources must equal uses)
  - Summary metrics (D/E ratio, equity %, total debt/equity)

## New PE/LBO Tools

- **value_bridge**: Returns attribution analysis
  - Decomposes equity returns into components:
    - EBITDA growth contribution
    - Multiple expansion/contraction
    - Deleveraging impact
  - Percentage attribution of each factor
  - Step-by-step bridge visualization

## Technical Changes

- Added chrono dependency for date handling
- 45 total tests (19 new Phase 3 tests), 100% pass rate
- New module structure with core/ and pe/ directories
- All functions maintain rust_decimal precision
