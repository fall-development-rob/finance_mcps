#!/usr/bin/env node

import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from '@modelcontextprotocol/sdk/types.js';
import {
  handleWaccCalculator,
  handleCreditMetrics,
  handleDcfModel,
  handleDebtCapacity,
  handleCovenantCompliance,
  handleCalculateNpv,
  handleCalculateIrr,
  handleCalculateMoic,
  handleSourcesAndUses,
  handleValueBridge,
} from './tools.js';
import {
  handleThreeStatementModel,
  handleEquityEnterpriseBridge,
  handleDilutedShares,
  handleAccountingFlow,
  handleFootballField,
  handlePaperLbo,
} from './tools-phase2.js';
import { PHASE2_TOOLS } from './tool-definitions-phase2.js';
import { PHASE3_TOOLS } from './tool-definitions-phase3.js';

// Tool definitions (Phase 1)
const PHASE1_TOOLS = [
  {
    name: 'wacc_calculator',
    description: 'Calculate Weighted Average Cost of Capital (WACC) for corporate valuation',
    inputSchema: {
      type: 'object',
      properties: {
        equity_value: {
          type: ['number', 'string'],
          description: 'Market value of equity',
        },
        debt_value: {
          type: ['number', 'string'],
          description: 'Market value of debt',
        },
        cost_of_equity: {
          type: ['number', 'string'],
          description: 'Cost of equity as percentage (e.g., 12.5 for 12.5%)',
        },
        cost_of_debt: {
          type: ['number', 'string'],
          description: 'Cost of debt as percentage',
        },
        tax_rate: {
          type: ['number', 'string'],
          description: 'Corporate tax rate as percentage',
        },
      },
      required: ['equity_value', 'debt_value', 'cost_of_equity', 'cost_of_debt', 'tax_rate'],
    },
  },
  {
    name: 'credit_metrics',
    description: 'Calculate key credit metrics for corporate debt analysis including leverage ratios and rating indication',
    inputSchema: {
      type: 'object',
      properties: {
        ebitda: {
          type: ['number', 'string'],
          description: 'EBITDA (Earnings Before Interest, Taxes, Depreciation, and Amortization)',
        },
        total_debt: {
          type: ['number', 'string'],
          description: 'Total debt outstanding',
        },
        interest_expense: {
          type: ['number', 'string'],
          description: 'Annual interest expense',
        },
        ebit: {
          type: ['number', 'string'],
          description: 'EBIT (Earnings Before Interest and Taxes)',
        },
        current_assets: {
          type: ['number', 'string'],
          description: 'Current assets',
        },
        current_liabilities: {
          type: ['number', 'string'],
          description: 'Current liabilities',
        },
        total_assets: {
          type: ['number', 'string'],
          description: 'Total assets',
        },
      },
      required: ['ebitda', 'total_debt', 'interest_expense', 'ebit', 'current_assets', 'current_liabilities', 'total_assets'],
    },
  },
  {
    name: 'dcf_model',
    description: 'Calculate Discounted Cash Flow (DCF) valuation with terminal value using perpetual growth method',
    inputSchema: {
      type: 'object',
      properties: {
        free_cash_flows: {
          type: 'array',
          items: {
            type: ['number', 'string'],
          },
          description: 'Array of projected free cash flows',
        },
        discount_rate: {
          type: ['number', 'string'],
          description: 'Discount rate as percentage (typically WACC)',
        },
        terminal_growth_rate: {
          type: ['number', 'string'],
          description: 'Terminal growth rate as percentage',
        },
      },
      required: ['free_cash_flows', 'discount_rate', 'terminal_growth_rate'],
    },
  },
  {
    name: 'debt_capacity',
    description: 'Calculate debt capacity based on EBITDA multiples, commonly used in leveraged finance and M&A',
    inputSchema: {
      type: 'object',
      properties: {
        ebitda: {
          type: ['number', 'string'],
          description: 'EBITDA',
        },
        target_leverage_multiple: {
          type: ['number', 'string'],
          description: 'Target leverage multiple (e.g., 4.5x)',
        },
        existing_debt: {
          type: ['number', 'string'],
          description: 'Existing debt amount',
        },
        cash_balance: {
          type: ['number', 'string'],
          description: 'Available cash balance',
        },
      },
      required: ['ebitda', 'target_leverage_multiple', 'existing_debt', 'cash_balance'],
    },
  },
  {
    name: 'covenant_compliance',
    description: 'Check compliance with debt covenants (maximum, minimum, or range-based)',
    inputSchema: {
      type: 'object',
      properties: {
        tests: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              name: {
                type: 'string',
                description: 'Name of the covenant test',
              },
              covenant_type: {
                type: 'string',
                enum: ['maximum', 'minimum', 'range'],
                description: 'Type of covenant',
              },
              limit: {
                type: ['number', 'string'],
                description: 'Covenant limit value',
              },
              actual: {
                type: ['number', 'string'],
                description: 'Actual value to test',
              },
            },
            required: ['name', 'covenant_type', 'limit', 'actual'],
          },
          description: 'Array of covenant tests to perform',
        },
      },
      required: ['tests'],
    },
  },
];

// Combine all tools
const ALL_TOOLS = [...PHASE1_TOOLS, ...PHASE2_TOOLS, ...PHASE3_TOOLS];

// Create server instance
const server = new Server(
  {
    name: 'corp-finance-mcp',
    version: '0.3.0',  // Updated to Phase 3
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

// Register tool list handler
server.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: ALL_TOOLS,
}));

// Register tool call handler
server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;

  switch (name) {
    // Phase 1 tools
    case 'wacc_calculator':
      return await handleWaccCalculator(args);
    case 'credit_metrics':
      return await handleCreditMetrics(args);
    case 'dcf_model':
      return await handleDcfModel(args);
    case 'debt_capacity':
      return await handleDebtCapacity(args);
    case 'covenant_compliance':
      return await handleCovenantCompliance(args);

    // Phase 2 tools
    case 'three_statement_model':
      return await handleThreeStatementModel(args);
    case 'equity_enterprise_bridge':
      return await handleEquityEnterpriseBridge(args);
    case 'diluted_shares':
      return await handleDilutedShares(args);
    case 'accounting_flow':
      return await handleAccountingFlow(args);
    case 'football_field':
      return await handleFootballField(args);
    case 'paper_lbo':
      return await handlePaperLbo(args);

    // Phase 3 tools
    case 'calculate_npv':
      return await handleCalculateNpv(args);
    case 'calculate_irr':
      return await handleCalculateIrr(args);
    case 'calculate_moic':
      return await handleCalculateMoic(args);
    case 'sources_and_uses':
      return await handleSourcesAndUses(args);
    case 'value_bridge':
      return await handleValueBridge(args);

    default:
      throw new Error(`Unknown tool: ${name}`);
  }
});

// Start server
async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error('Corp Finance MCP Server running on stdio');
}

main().catch((error) => {
  console.error('Server error:', error);
  process.exit(1);
});
