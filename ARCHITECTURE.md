# Architecture: Corp Finance MCP

## Design Principles

1. **Precision First**: All financial calculations use `rust_decimal` to ensure accurate decimal arithmetic
2. **Clear Separation**: Rust handles computation, TypeScript handles I/O and protocol
3. **Type Safety**: End-to-end type safety from Zod validation to Rust types
4. **Performance**: Native Rust performance with zero-copy FFI via napi-rs

## Layer Breakdown

### Layer 1: Rust Core (`crates/corp-finance-core`)

**Responsibility**: Financial calculations

**Key Features**:
- Pure Rust implementation
- Uses `rust_decimal::Decimal` for all monetary values
- Comprehensive error handling with `thiserror`
- Fully tested with unit tests
- No external dependencies except math libraries

**Modules**:
- `wacc.rs` - Weighted Average Cost of Capital
- `credit_metrics.rs` - Credit analysis (leverage ratios, coverage)
- `dcf.rs` - Discounted Cash Flow valuation
- `debt_capacity.rs` - Debt capacity analysis
- `covenant.rs` - Covenant compliance checking

### Layer 2: NAPI Bindings (`packages/bindings`)

**Responsibility**: Bridge between Rust and Node.js

**Key Features**:
- Uses `napi-rs` for safe FFI
- String-based JSON interface (simple and reliable)
- Minimal transformation logic
- Error propagation from Rust to JS

**Data Flow**:
```
JS String (JSON) → Rust serde_json → Rust types → Calculation → Rust types → JSON String → JS
```

### Layer 3: TypeScript MCP Server (`packages/mcp-server`)

**Responsibility**: MCP protocol and input validation

**Key Features**:
- Implements MCP protocol via `@modelcontextprotocol/sdk`
- Zod schemas for runtime type validation
- Tool definitions with JSON schemas
- Error handling and user-friendly messages

**Components**:
- `index.ts` - MCP server setup and routing
- `tools.ts` - Tool handlers (one per financial function)
- `schemas.ts` - Zod validation schemas

## Data Flow Example

### WACC Calculation Flow

1. **MCP Request** (from Claude or other client)
```json
{
  "name": "wacc_calculator",
  "arguments": {
    "equity_value": "700000",
    "debt_value": "300000",
    "cost_of_equity": "12.5",
    "cost_of_debt": "6.0",
    "tax_rate": "25.0"
  }
}
```

2. **Zod Validation** (in TypeScript)
```typescript
const validatedInput = WaccInputSchema.parse(args);
// Ensures all required fields present and correct types
```

3. **JSON Serialization**
```typescript
const inputJson = JSON.stringify(validatedInput);
// Convert to string for Rust
```

4. **NAPI Binding Call**
```typescript
const resultJson = wacc_calculator(inputJson);
// Call native Rust function
```

5. **Rust Deserialization**
```rust
let input: WaccInput = serde_json::from_str(&input)?;
```

6. **Rust Calculation**
```rust
let output = calculate_wacc(input)?;
// Pure Rust calculation with rust_decimal
```

7. **Rust Serialization**
```rust
serde_json::to_string(&output)?
```

8. **MCP Response**
```json
{
  "wacc": "10.1",
  "equity_weight": "0.7",
  "debt_weight": "0.3",
  "after_tax_cost_of_debt": "4.5"
}
```

## Error Handling

### Rust Layer
- Custom `FinanceError` enum with `thiserror`
- Specific error variants (InvalidInput, DivisionByZero, etc.)
- Propagated via `Result<T, FinanceError>`

### Bindings Layer
- Converts Rust errors to NAPI errors
- Preserves error messages

### TypeScript Layer
- Zod validation errors for malformed input
- McpError for protocol-level errors
- User-friendly error messages

## Testing Strategy

### Rust Tests
- Unit tests for each calculation module
- Test edge cases (zero values, negative numbers)
- Test precision of decimal calculations

### Integration Tests (Future)
- End-to-end tests through napi bindings
- Test TypeScript → Rust → TypeScript roundtrip

## Build Process

1. **Cargo Build**: Compiles Rust core library
2. **NAPI Build**: Compiles bindings and generates `.node` native module
3. **TypeScript Build**: Compiles TS to JS with type definitions

## Why This Architecture?

### Rust for Calculations
- **Precision**: `rust_decimal` provides exact decimal arithmetic
- **Performance**: Native speed for complex calculations
- **Safety**: Rust's type system prevents common bugs
- **Testability**: Pure functions, easy to test

### NAPI-RS for Bindings
- **Type Safety**: Auto-generates TypeScript types
- **Performance**: Zero-copy where possible
- **Stability**: Battle-tested in production systems

### TypeScript for MCP
- **Ecosystem**: Rich MCP SDK and tooling
- **Flexibility**: Easy to add new tools and features
- **Developer Experience**: Great IDE support

### Zod for Validation
- **Runtime Safety**: Catches invalid input before Rust
- **Type Inference**: TypeScript types from schemas
- **Error Messages**: User-friendly validation errors

## Future Enhancements

### Phase 2 Tools (Planned)
- LBO model
- Merger model
- Comparable company analysis
- Precedent transactions

### Optimizations
- Caching for repeated calculations
- Batch calculation support
- Streaming for large datasets

### Additional Features
- Sensitivity analysis
- Monte Carlo simulations
- Scenario modeling
