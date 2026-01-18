// Phase 2 Tool Definitions for MCP

export const PHASE2_TOOLS = [
  {
    name: 'three_statement_model',
    description: 'Build linked three-statement financial model (Income Statement, Balance Sheet, Cash Flow)',
    inputSchema: {
      type: 'object',
      properties: {
        starting_cash: { type: ['number', 'string'], description: 'Starting cash balance' },
        starting_debt: { type: ['number', 'string'], description: 'Starting debt balance' },
        starting_equity: { type: ['number', 'string'], description: 'Starting equity balance' },
        starting_inventory: { type: ['number', 'string'], description: 'Starting inventory' },
        starting_ar: { type: ['number', 'string'], description: 'Starting accounts receivable' },
        starting_ap: { type: ['number', 'string'], description: 'Starting accounts payable' },
        starting_ppe: { type: ['number', 'string'], description: 'Starting PP&E (net)' },
        revenue: { type: 'array', items: { type: ['number', 'string'] }, description: 'Annual revenue projections' },
        cogs_percent: { type: ['number', 'string'], description: 'COGS as % of revenue' },
        opex_percent: { type: ['number', 'string'], description: 'OpEx as % of revenue' },
        tax_rate: { type: ['number', 'string'], description: 'Tax rate %' },
        capex: { type: 'array', items: { type: ['number', 'string'] }, description: 'Annual CapEx' },
        depreciation: { type: 'array', items: { type: ['number', 'string'] }, description: 'Annual depreciation' },
        nwc_percent_revenue: { type: ['number', 'string'], description: 'NWC as % of revenue' },
        interest_rate: { type: ['number', 'string'], description: 'Interest rate % on debt' },
      },
      required: ['starting_cash', 'starting_debt', 'starting_equity', 'starting_inventory', 'starting_ar', 'starting_ap', 'starting_ppe', 'revenue', 'cogs_percent', 'opex_percent', 'tax_rate', 'capex', 'depreciation', 'nwc_percent_revenue', 'interest_rate'],
    },
  },
  {
    name: 'equity_enterprise_bridge',
    description: 'Convert between Equity Value and Enterprise Value using bridge items',
    inputSchema: {
      type: 'object',
      properties: {
        direction: { type: 'string', enum: ['equity_to_ev', 'ev_to_equity'], description: 'Conversion direction' },
        value: { type: ['number', 'string'], description: 'Starting value (equity or EV depending on direction)' },
        cash: { type: ['number', 'string'], description: 'Cash balance' },
        debt: { type: ['number', 'string'], description: 'Total debt' },
        minority_interest: { type: ['number', 'string'], description: 'Minority interest' },
        associates: { type: ['number', 'string'], description: 'Associates/investments' },
        preferred_stock: { type: ['number', 'string'], description: 'Preferred stock' },
      },
      required: ['direction', 'value', 'cash', 'debt', 'minority_interest', 'associates', 'preferred_stock'],
    },
  },
  {
    name: 'diluted_shares',
    description: 'Calculate fully diluted shares using treasury stock method for options, RSUs, and convertibles',
    inputSchema: {
      type: 'object',
      properties: {
        basic_shares: { type: ['number', 'string'], description: 'Basic shares outstanding' },
        stock_price: { type: ['number', 'string'], description: 'Current stock price' },
        options: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              quantity: { type: ['number', 'string'] },
              strike_price: { type: ['number', 'string'] },
            },
            required: ['quantity', 'strike_price'],
          },
          description: 'Stock options',
        },
        rsus: { type: ['number', 'string'], description: 'RSUs outstanding' },
        convertibles: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              principal: { type: ['number', 'string'] },
              conversion_price: { type: ['number', 'string'] },
            },
            required: ['principal', 'conversion_price'],
          },
          description: 'Convertible securities',
        },
      },
      required: ['basic_shares', 'stock_price', 'options', 'rsus', 'convertibles'],
    },
  },
  {
    name: 'accounting_flow',
    description: 'Analyze impact of a transaction on all three financial statements ("walk me through" questions)',
    inputSchema: {
      type: 'object',
      properties: {
        transaction: { type: 'string', description: 'Description of the transaction' },
        amount: { type: ['number', 'string'], description: 'Transaction amount' },
        transaction_type: {
          type: 'string',
          enum: ['depreciation', 'amortization', 'capex', 'debt_issuance', 'debt_repayment', 'inventory_purchase', 'revenue_recognition'],
          description: 'Type of transaction',
        },
      },
      required: ['transaction', 'amount', 'transaction_type'],
    },
  },
  {
    name: 'football_field',
    description: 'Create football field valuation summary across multiple methodologies (DCF, Comps, Precedents)',
    inputSchema: {
      type: 'object',
      properties: {
        dcf_low: { type: ['number', 'string'], description: 'DCF valuation low end' },
        dcf_high: { type: ['number', 'string'], description: 'DCF valuation high end' },
        comps_low: { type: ['number', 'string'], description: 'Comps valuation low end' },
        comps_high: { type: ['number', 'string'], description: 'Comps valuation high end' },
        precedents_low: { type: ['number', 'string'], description: 'Precedents valuation low end' },
        precedents_high: { type: ['number', 'string'], description: 'Precedents valuation high end' },
        current_price: { type: ['number', 'string'], description: 'Current price (optional)', required: false },
      },
      required: ['dcf_low', 'dcf_high', 'comps_low', 'comps_high', 'precedents_low', 'precedents_high'],
    },
  },
  {
    name: 'paper_lbo',
    description: 'Quick mental math LBO analysis (no Excel) - calculate IRR and returns',
    inputSchema: {
      type: 'object',
      properties: {
        purchase_price: { type: ['number', 'string'], description: 'Purchase price' },
        ebitda: { type: ['number', 'string'], description: 'Entry EBITDA' },
        entry_multiple: { type: ['number', 'string'], description: 'Entry EBITDA multiple' },
        debt_multiple: { type: ['number', 'string'], description: 'Debt as multiple of EBITDA (e.g., 5x)' },
        ebitda_growth_rate: { type: ['number', 'string'], description: 'Annual EBITDA growth %' },
        hold_period_years: { type: 'number', description: 'Hold period in years' },
        exit_multiple: { type: ['number', 'string'], description: 'Exit EBITDA multiple' },
        interest_rate: { type: ['number', 'string'], description: 'Interest rate % on debt' },
      },
      required: ['purchase_price', 'ebitda', 'entry_multiple', 'debt_multiple', 'ebitda_growth_rate', 'hold_period_years', 'exit_multiple', 'interest_rate'],
    },
  },
];
