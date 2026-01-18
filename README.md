# Corp Finance MCP - Rust/TypeScript Stack

A high-precision corporate finance MCP (Model Context Protocol) server built with Rust for calculations and TypeScript for the interface layer.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    MCP Client (Claude)                  │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              TypeScript MCP Server                      │
│              (@modelcontextprotocol/sdk)                │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼ Zod Validation
┌─────────────────────────────────────────────────────────┐
│                  napi-rs Bindings                       │
│              (packages/bindings/)                       │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼ JSON Serialization
┌─────────────────────────────────────────────────────────┐
│              Rust Core Calculations                     │
│           (crates/corp-finance-core/)                   │
│              Using rust_decimal                         │
└─────────────────────────────────────────────────────────┘
```

## Structure

```
corp-finance-mcp/
├── crates/
│   └── corp-finance-core/     # Rust calculations (rust_decimal for precision)
│       ├── src/
│       │   ├── wacc.rs        # WACC calculator
│       │   ├── credit_metrics.rs  # Credit analysis
│       │   ├── dcf.rs         # DCF model
│       │   ├── debt_capacity.rs   # Debt capacity analysis
│       │   ├── covenant.rs    # Covenant compliance
│       │   ├── fundamentals/  # Phase 2: Fundamentals
│       │   │   ├── three_statement_model.rs
│       │   │   ├── equity_enterprise_bridge.rs
│       │   │   ├── diluted_shares.rs
│       │   │   └── accounting_flows.rs
│       │   └── valuation/     # Phase 2: Valuation
│       │       ├── football_field.rs
│       │       └── paper_lbo.rs
│       └── Cargo.toml
├── packages/
│   ├── bindings/              # napi-rs to expose Rust to Node
│   │   ├── src/lib.rs
│   │   └── Cargo.toml
│   └── mcp-server/            # TypeScript MCP server
│       ├── src/
│       │   ├── index.ts       # MCP server entry point
│       │   ├── tools.ts       # Tool handlers
│       │   └── schemas.ts     # Zod validation schemas
│       └── package.json
├── Cargo.toml                 # Workspace root
└── package.json               # npm workspace root
```

## Phase 1 Tools (Valuation & Analysis)

1. **wacc_calculator** - Calculate Weighted Average Cost of Capital
2. **credit_metrics** - Analyze credit metrics (leverage, coverage ratios)
3. **dcf_model** - Discounted Cash Flow valuation
4. **debt_capacity** - Calculate debt capacity based on EBITDA multiples
5. **covenant_compliance** - Check debt covenant compliance

## Phase 2 Tools (Fundamentals & Advanced Valuation)

### Fundamentals
6. **three_statement_model** - Build linked Income Statement, Balance Sheet, and Cash Flow projections
7. **equity_enterprise_bridge** - Convert between Equity Value and Enterprise Value with bridge items
8. **diluted_shares** - Calculate fully diluted shares using treasury stock method (options, RSUs, convertibles)
9. **accounting_flow** - Analyze transaction impact across all three financial statements ("walk me through" questions)

### Valuation
10. **football_field** - Create valuation range summary across DCF, Comps, and Precedents
11. **paper_lbo** - Quick mental math LBO analysis with IRR calculation

## Data Flow

```
MCP Request → Zod Validation → napi Binding → Rust Calculation → JSON → MCP Response
```

## Key Dependencies

### Rust
- `rust_decimal` - High-precision decimal arithmetic
- `serde` - Serialization/deserialization
- `thiserror` - Error handling

### Bindings
- `napi-rs` - Rust-Node.js bindings

### TypeScript
- `@modelcontextprotocol/sdk` - MCP protocol implementation
- `zod` - Runtime type validation

## Principle

**Rust owns all math, TypeScript owns the interface.**

All financial calculations are performed in Rust using `rust_decimal` for precision. The TypeScript layer validates inputs with Zod and exposes tools via MCP.

## Installation

```bash
# Install dependencies
npm install

# Build everything (Rust → Bindings → TypeScript)
npm run build
```

## Development

```bash
# Run tests
npm test

# Watch mode for TypeScript
npm run dev
```

## Usage

The MCP server runs on stdio and can be used with any MCP client:

```bash
node packages/mcp-server/dist/index.js
```

### Example Tool Call (WACC)

```json
{
  "equity_value": "700000",
  "debt_value": "300000",
  "cost_of_equity": "12.5",
  "cost_of_debt": "6.0",
  "tax_rate": "25.0"
}
```

Response:
```json
{
  "wacc": "10.1",
  "equity_weight": "0.7",
  "debt_weight": "0.3",
  "after_tax_cost_of_debt": "4.5"
}
```

## Building from Source

### Prerequisites
- Rust 1.70+
- Node.js 18+
- npm or pnpm

### Build Steps

```bash
# 1. Build Rust core
cargo build --release

# 2. Build napi bindings
cd packages/bindings
npm run build
cd ../..

# 3. Build TypeScript server
cd packages/mcp-server
npm run build
cd ../..
```

Or use the workspace command:

```bash
npm run build
```

## License

MIT
