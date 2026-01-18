# Quick Start Guide

## Prerequisites

- Rust 1.70+ (`rustc --version`)
- Node.js 18+ (`node --version`)
- npm 9+ (`npm --version`)

## Installation & Build

### Option 1: Hivemind Build (Recommended)

```bash
# Run the orchestrated hivemind build
./scripts/hivemind-build.sh
```

This will:
1. Build Rust core with tests
2. Build NAPI bindings
3. Build TypeScript MCP server
4. Verify all artifacts

### Option 2: Manual Build

```bash
# Install root dependencies
npm install

# Build everything
npm run build
```

### Option 3: Step-by-Step

```bash
# 1. Build and test Rust core
cargo build --release
cargo test

# 2. Build NAPI bindings
cd packages/bindings
npm install
npm run build
cd ../..

# 3. Build TypeScript MCP server
cd packages/mcp-server
npm install
npm run build
cd ../..
```

## Running the Server

```bash
node packages/mcp-server/dist/index.js
```

The server runs on stdio and implements the MCP protocol.

## Testing Tools

### 1. WACC Calculator

```json
{
  "name": "wacc_calculator",
  "arguments": {
    "equity_value": "1000000",
    "debt_value": "500000",
    "cost_of_equity": "15.0",
    "cost_of_debt": "5.0",
    "tax_rate": "21.0"
  }
}
```

### 2. Credit Metrics

```json
{
  "name": "credit_metrics",
  "arguments": {
    "ebitda": "100000",
    "total_debt": "400000",
    "interest_expense": "20000",
    "ebit": "85000",
    "current_assets": "200000",
    "current_liabilities": "150000",
    "total_assets": "800000"
  }
}
```

### 3. DCF Model

```json
{
  "name": "dcf_model",
  "arguments": {
    "free_cash_flows": ["50000", "55000", "60500", "66550", "73205"],
    "discount_rate": "12.0",
    "terminal_growth_rate": "3.0"
  }
}
```

### 4. Debt Capacity

```json
{
  "name": "debt_capacity",
  "arguments": {
    "ebitda": "75000",
    "target_leverage_multiple": "4.0",
    "existing_debt": "250000",
    "cash_balance": "50000"
  }
}
```

### 5. Covenant Compliance

```json
{
  "name": "covenant_compliance",
  "arguments": {
    "tests": [
      {
        "name": "Max Total Leverage",
        "covenant_type": "maximum",
        "limit": "5.0",
        "actual": "4.2"
      },
      {
        "name": "Min Interest Coverage",
        "covenant_type": "minimum",
        "limit": "3.0",
        "actual": "4.5"
      }
    ]
  }
}
```

## Development Mode

```bash
# Watch mode for TypeScript (auto-rebuild on changes)
cd packages/mcp-server
npm run dev
```

## Troubleshooting

### Rust compilation errors
```bash
rustup update
cargo clean
cargo build --release
```

### NAPI binding errors
```bash
cd packages/bindings
rm -rf node_modules package-lock.json
npm install
npm run build
```

### TypeScript errors
```bash
cd packages/mcp-server
rm -rf node_modules dist package-lock.json
npm install
npm run build
```

## Project Structure

```
corp-finance-mcp/
├── crates/corp-finance-core/    # Rust calculations
├── packages/
│   ├── bindings/                # NAPI-RS bindings
│   └── mcp-server/              # TypeScript MCP server
└── scripts/                     # Build orchestration
```

## Architecture

See [ARCHITECTURE.md](./ARCHITECTURE.md) for detailed architecture documentation.

## Next Steps

1. **Connect to Claude Desktop**: Add this server to your Claude Desktop config
2. **Explore Tools**: Try each of the 5 Phase 1 tools
3. **Extend**: Add your own financial calculations in Rust
4. **Deploy**: Package for production use

## Support

For issues and questions:
- Check [README.md](./README.md) for detailed documentation
- Review [ARCHITECTURE.md](./ARCHITECTURE.md) for design details
- Submit issues on GitHub
