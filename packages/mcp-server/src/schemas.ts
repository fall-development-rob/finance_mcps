import { z } from 'zod';

// Zod schemas for input validation

export const WaccInputSchema = z.object({
  equity_value: z.string().or(z.number()),
  debt_value: z.string().or(z.number()),
  cost_of_equity: z.string().or(z.number()),
  cost_of_debt: z.string().or(z.number()),
  tax_rate: z.string().or(z.number()),
});

export const CreditMetricsInputSchema = z.object({
  ebitda: z.string().or(z.number()),
  total_debt: z.string().or(z.number()),
  interest_expense: z.string().or(z.number()),
  ebit: z.string().or(z.number()),
  current_assets: z.string().or(z.number()),
  current_liabilities: z.string().or(z.number()),
  total_assets: z.string().or(z.number()),
});

export const DcfInputSchema = z.object({
  free_cash_flows: z.array(z.string().or(z.number())),
  discount_rate: z.string().or(z.number()),
  terminal_growth_rate: z.string().or(z.number()),
});

export const DebtCapacityInputSchema = z.object({
  ebitda: z.string().or(z.number()),
  target_leverage_multiple: z.string().or(z.number()),
  existing_debt: z.string().or(z.number()),
  cash_balance: z.string().or(z.number()),
});

export const CovenantTestSchema = z.object({
  name: z.string(),
  covenant_type: z.enum(['maximum', 'minimum', 'range']),
  limit: z.string().or(z.number()),
  actual: z.string().or(z.number()),
});

export const CovenantInputSchema = z.object({
  tests: z.array(CovenantTestSchema),
});

// ========== Phase 2 Schemas ==========

export const ThreeStatementInputSchema = z.object({
  starting_cash: z.string().or(z.number()),
  starting_debt: z.string().or(z.number()),
  starting_equity: z.string().or(z.number()),
  starting_inventory: z.string().or(z.number()),
  starting_ar: z.string().or(z.number()),
  starting_ap: z.string().or(z.number()),
  starting_ppe: z.string().or(z.number()),
  revenue: z.array(z.string().or(z.number())),
  cogs_percent: z.string().or(z.number()),
  opex_percent: z.string().or(z.number()),
  tax_rate: z.string().or(z.number()),
  capex: z.array(z.string().or(z.number())),
  depreciation: z.array(z.string().or(z.number())),
  nwc_percent_revenue: z.string().or(z.number()),
  interest_rate: z.string().or(z.number()),
});

export const EquityEnterpriseBridgeInputSchema = z.object({
  direction: z.enum(['equity_to_ev', 'ev_to_equity']),
  value: z.string().or(z.number()),
  cash: z.string().or(z.number()),
  debt: z.string().or(z.number()),
  minority_interest: z.string().or(z.number()),
  associates: z.string().or(z.number()),
  preferred_stock: z.string().or(z.number()),
});

export const OptionGrantSchema = z.object({
  quantity: z.string().or(z.number()),
  strike_price: z.string().or(z.number()),
});

export const ConvertibleSchema = z.object({
  principal: z.string().or(z.number()),
  conversion_price: z.string().or(z.number()),
});

export const DilutedSharesInputSchema = z.object({
  basic_shares: z.string().or(z.number()),
  stock_price: z.string().or(z.number()),
  options: z.array(OptionGrantSchema),
  rsus: z.string().or(z.number()),
  convertibles: z.array(ConvertibleSchema),
});

export const AccountingFlowInputSchema = z.object({
  transaction: z.string(),
  amount: z.string().or(z.number()),
  transaction_type: z.enum([
    'depreciation',
    'amortization',
    'capex',
    'debt_issuance',
    'debt_repayment',
    'inventory_purchase',
    'revenue_recognition',
  ]),
});

export const FootballFieldInputSchema = z.object({
  dcf_low: z.string().or(z.number()),
  dcf_high: z.string().or(z.number()),
  comps_low: z.string().or(z.number()),
  comps_high: z.string().or(z.number()),
  precedents_low: z.string().or(z.number()),
  precedents_high: z.string().or(z.number()),
  current_price: z.string().or(z.number()).optional(),
});

export const PaperLboInputSchema = z.object({
  purchase_price: z.string().or(z.number()),
  ebitda: z.string().or(z.number()),
  entry_multiple: z.string().or(z.number()),
  debt_multiple: z.string().or(z.number()),
  ebitda_growth_rate: z.string().or(z.number()),
  hold_period_years: z.number(),
  exit_multiple: z.string().or(z.number()),
  interest_rate: z.string().or(z.number()),
});

// ========== Phase 3 Schemas ==========

export const NpvInputSchema = z.object({
  cash_flows: z.array(z.string().or(z.number())),
  discount_rate: z.string().or(z.number()),
});

export const IrrInputSchema = z.object({
  cash_flows: z.array(z.string().or(z.number())),
  initial_guess: z.string().or(z.number()).optional(),
});

export const MoicInputSchema = z.object({
  invested_capital: z.string().or(z.number()),
  realized_value: z.string().or(z.number()),
});

export const SourceItemSchema = z.object({
  name: z.string(),
  amount: z.string().or(z.number()),
  pct_of_total: z.string().or(z.number()).optional(),
});

export const UseItemSchema = z.object({
  name: z.string(),
  amount: z.string().or(z.number()),
  pct_of_total: z.string().or(z.number()).optional(),
});

export const SourcesAndUsesInputSchema = z.object({
  senior_debt: z.string().or(z.number()),
  subordinated_debt: z.string().or(z.number()),
  equity_contribution: z.string().or(z.number()),
  rollover_equity: z.string().or(z.number()),
  seller_note: z.string().or(z.number()).optional(),
  other_sources: z.array(SourceItemSchema).default([]),
  purchase_equity_value: z.string().or(z.number()),
  refinanced_debt: z.string().or(z.number()),
  transaction_fees: z.string().or(z.number()),
  financing_fees: z.string().or(z.number()),
  other_uses: z.array(UseItemSchema).default([]),
});

export const ValueBridgeInputSchema = z.object({
  entry_ebitda: z.string().or(z.number()),
  entry_multiple: z.string().or(z.number()),
  entry_net_debt: z.string().or(z.number()),
  exit_ebitda: z.string().or(z.number()),
  exit_multiple: z.string().or(z.number()),
  exit_net_debt: z.string().or(z.number()),
});

// Type exports for TypeScript
export type WaccInput = z.infer<typeof WaccInputSchema>;
export type CreditMetricsInput = z.infer<typeof CreditMetricsInputSchema>;
export type DcfInput = z.infer<typeof DcfInputSchema>;
export type DebtCapacityInput = z.infer<typeof DebtCapacityInputSchema>;
export type CovenantInput = z.infer<typeof CovenantInputSchema>;

export type ThreeStatementInput = z.infer<typeof ThreeStatementInputSchema>;
export type EquityEnterpriseBridgeInput = z.infer<typeof EquityEnterpriseBridgeInputSchema>;
export type DilutedSharesInput = z.infer<typeof DilutedSharesInputSchema>;
export type AccountingFlowInput = z.infer<typeof AccountingFlowInputSchema>;
export type FootballFieldInput = z.infer<typeof FootballFieldInputSchema>;
export type PaperLboInput = z.infer<typeof PaperLboInputSchema>;

export type NpvInput = z.infer<typeof NpvInputSchema>;
export type IrrInput = z.infer<typeof IrrInputSchema>;
export type MoicInput = z.infer<typeof MoicInputSchema>;
export type SourcesAndUsesInput = z.infer<typeof SourcesAndUsesInputSchema>;
export type ValueBridgeInput = z.infer<typeof ValueBridgeInputSchema>;
