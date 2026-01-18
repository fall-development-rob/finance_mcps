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

// Type exports for TypeScript
export type WaccInput = z.infer<typeof WaccInputSchema>;
export type CreditMetricsInput = z.infer<typeof CreditMetricsInputSchema>;
export type DcfInput = z.infer<typeof DcfInputSchema>;
export type DebtCapacityInput = z.infer<typeof DebtCapacityInputSchema>;
export type CovenantInput = z.infer<typeof CovenantInputSchema>;
