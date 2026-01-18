#!/usr/bin/env node

/**
 * Integration test for Corp Finance MCP
 * Tests the complete data flow: Input → Zod → napi → Rust → JSON → Output
 */

import { waccCalculator, creditMetrics, dcfModel, debtCapacity, covenantCompliance } from '../packages/bindings/index.js';

console.log('🧪 Corp Finance MCP Integration Tests\n');
console.log('Testing complete data flow: Input → Zod → napi → Rust → JSON → Output\n');

let passed = 0;
let failed = 0;

function test(name, fn) {
  try {
    fn();
    console.log(`✓ ${name}`);
    passed++;
  } catch (error) {
    console.log(`✗ ${name}`);
    console.error(`  Error: ${error.message}`);
    failed++;
  }
}

// Test 1: WACC Calculator
test('WACC Calculator', () => {
  const input = JSON.stringify({
    equity_value: "700000",
    debt_value: "300000",
    cost_of_equity: "12.5",
    cost_of_debt: "6.0",
    tax_rate: "25.0"
  });

  const result = waccCalculator(input);
  const output = JSON.parse(result);

  // Compare as numbers to handle decimal precision
  if (parseFloat(output.wacc) !== 10.1) {
    throw new Error(`Expected WACC 10.1, got ${output.wacc}`);
  }
});

// Test 2: Credit Metrics
test('Credit Metrics', () => {
  const input = JSON.stringify({
    ebitda: "100000",
    total_debt: "250000",
    interest_expense: "15000",
    ebit: "80000",
    current_assets: "150000",
    current_liabilities: "100000",
    total_assets: "500000"
  });

  const result = creditMetrics(input);
  const output = JSON.parse(result);

  if (parseFloat(output.debt_to_ebitda) !== 2.5) {
    throw new Error(`Expected debt_to_ebitda 2.5, got ${output.debt_to_ebitda}`);
  }
});

// Test 3: DCF Model
test('DCF Model', () => {
  const input = JSON.stringify({
    free_cash_flows: ["10000", "11000", "12100", "13310", "14641"],
    discount_rate: "10.0",
    terminal_growth_rate: "2.5"
  });

  const result = dcfModel(input);
  const output = JSON.parse(result);

  if (!output.enterprise_value || parseFloat(output.enterprise_value) <= 0) {
    throw new Error(`Invalid enterprise value: ${output.enterprise_value}`);
  }
});

// Test 4: Debt Capacity
test('Debt Capacity', () => {
  const input = JSON.stringify({
    ebitda: "50000",
    target_leverage_multiple: "4.5",
    existing_debt: "180000",
    cash_balance: "25000"
  });

  const result = debtCapacity(input);
  const output = JSON.parse(result);

  if (parseFloat(output.maximum_debt) !== 225000) {
    throw new Error(`Expected maximum_debt 225000, got ${output.maximum_debt}`);
  }
});

// Test 5: Covenant Compliance
test('Covenant Compliance - Passing', () => {
  const input = JSON.stringify({
    tests: [
      {
        name: "Max Leverage",
        covenant_type: "maximum",
        limit: "5.0",
        actual: "4.2"
      },
      {
        name: "Min Interest Coverage",
        covenant_type: "minimum",
        limit: "2.5",
        actual: "3.1"
      }
    ]
  });

  const result = covenantCompliance(input);
  const output = JSON.parse(result);

  if (!output.overall_compliant) {
    throw new Error(`Expected overall_compliant true, got ${output.overall_compliant}`);
  }
});

// Test 6: Covenant Compliance - Failing
test('Covenant Compliance - Failing', () => {
  const input = JSON.stringify({
    tests: [
      {
        name: "Max Leverage",
        covenant_type: "maximum",
        limit: "5.0",
        actual: "5.5"
      }
    ]
  });

  const result = covenantCompliance(input);
  const output = JSON.parse(result);

  if (output.overall_compliant) {
    throw new Error(`Expected overall_compliant false for violation, got ${output.overall_compliant}`);
  }

  if (output.violations.length === 0) {
    throw new Error(`Expected violations array to have items`);
  }
});

// Summary
console.log(`\n${'='.repeat(50)}`);
console.log(`Tests passed: ${passed}`);
console.log(`Tests failed: ${failed}`);
console.log(`${'='.repeat(50)}`);

if (failed > 0) {
  process.exit(1);
}

console.log('\n✅ All integration tests passed!');
console.log('✅ Data flow verified: Input → Zod → napi → Rust → JSON → Output');
