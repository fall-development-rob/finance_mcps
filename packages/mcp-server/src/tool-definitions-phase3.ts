export const PHASE3_TOOLS = [
  {
    name: 'calculate_npv',
    description: 'Calculate Net Present Value (NPV) of a series of cash flows at a given discount rate',
    inputSchema: {
      type: 'object',
      properties: {
        cash_flows: {
          type: 'array',
          items: {
            type: ['number', 'string'],
          },
          description: 'Array of cash flows (negative for outflows, positive for inflows)',
        },
        discount_rate: {
          type: ['number', 'string'],
          description: 'Discount rate as percentage (e.g., 10 for 10%)',
        },
      },
      required: ['cash_flows', 'discount_rate'],
    },
  },
  {
    name: 'calculate_irr',
    description: 'Calculate Internal Rate of Return (IRR) using Newton-Raphson method for a series of cash flows',
    inputSchema: {
      type: 'object',
      properties: {
        cash_flows: {
          type: 'array',
          items: {
            type: ['number', 'string'],
          },
          description: 'Array of cash flows (must start with negative investment)',
        },
        initial_guess: {
          type: ['number', 'string'],
          description: 'Initial guess for IRR as percentage (defaults to 10)',
        },
      },
      required: ['cash_flows'],
    },
  },
  {
    name: 'calculate_moic',
    description: 'Calculate Multiple on Invested Capital (MOIC) - a key PE/LBO metric',
    inputSchema: {
      type: 'object',
      properties: {
        invested_capital: {
          type: ['number', 'string'],
          description: 'Initial capital invested',
        },
        realized_value: {
          type: ['number', 'string'],
          description: 'Total realized/exit value',
        },
      },
      required: ['invested_capital', 'realized_value'],
    },
  },
  {
    name: 'sources_and_uses',
    description: 'Build Sources and Uses table for M&A/LBO transactions with automatic balancing and percentage calculations',
    inputSchema: {
      type: 'object',
      properties: {
        senior_debt: {
          type: ['number', 'string'],
          description: 'Senior debt amount',
        },
        subordinated_debt: {
          type: ['number', 'string'],
          description: 'Subordinated debt amount',
        },
        equity_contribution: {
          type: ['number', 'string'],
          description: 'Sponsor equity contribution',
        },
        rollover_equity: {
          type: ['number', 'string'],
          description: 'Management rollover equity',
        },
        seller_note: {
          type: ['number', 'string'],
          description: 'Seller note amount (optional)',
        },
        other_sources: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              name: {
                type: 'string',
                description: 'Source name',
              },
              amount: {
                type: ['number', 'string'],
                description: 'Source amount',
              },
            },
            required: ['name', 'amount'],
          },
          description: 'Additional sources',
        },
        purchase_equity_value: {
          type: ['number', 'string'],
          description: 'Purchase equity value',
        },
        refinanced_debt: {
          type: ['number', 'string'],
          description: 'Debt being refinanced',
        },
        transaction_fees: {
          type: ['number', 'string'],
          description: 'Transaction/advisory fees',
        },
        financing_fees: {
          type: ['number', 'string'],
          description: 'Financing/arrangement fees',
        },
        other_uses: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              name: {
                type: 'string',
                description: 'Use name',
              },
              amount: {
                type: ['number', 'string'],
                description: 'Use amount',
              },
            },
            required: ['name', 'amount'],
          },
          description: 'Additional uses',
        },
      },
      required: [
        'senior_debt',
        'subordinated_debt',
        'equity_contribution',
        'rollover_equity',
        'purchase_equity_value',
        'refinanced_debt',
        'transaction_fees',
        'financing_fees',
      ],
    },
  },
  {
    name: 'value_bridge',
    description: 'Calculate PE/LBO returns attribution bridge - decompose equity returns into EBITDA growth, multiple expansion, and deleveraging',
    inputSchema: {
      type: 'object',
      properties: {
        entry_ebitda: {
          type: ['number', 'string'],
          description: 'Entry EBITDA',
        },
        entry_multiple: {
          type: ['number', 'string'],
          description: 'Entry EV/EBITDA multiple',
        },
        entry_net_debt: {
          type: ['number', 'string'],
          description: 'Entry net debt',
        },
        exit_ebitda: {
          type: ['number', 'string'],
          description: 'Exit EBITDA',
        },
        exit_multiple: {
          type: ['number', 'string'],
          description: 'Exit EV/EBITDA multiple',
        },
        exit_net_debt: {
          type: ['number', 'string'],
          description: 'Exit net debt',
        },
      },
      required: [
        'entry_ebitda',
        'entry_multiple',
        'entry_net_debt',
        'exit_ebitda',
        'exit_multiple',
        'exit_net_debt',
      ],
    },
  },
];
