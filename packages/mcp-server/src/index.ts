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
} from './tools.js';

// Tool definitions
const TOOLS = [
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

// Create server instance
const server = new Server(
  {
    name: 'corp-finance-mcp',
    version: '0.1.0',
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

// Register tool list handler
server.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: TOOLS,
}));

// Register tool call handler
server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;

  switch (name) {
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
