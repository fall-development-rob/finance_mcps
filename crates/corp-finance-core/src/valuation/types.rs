use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Football Field Valuation Summary Input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootballFieldInput {
    pub dcf_low: Decimal,
    pub dcf_high: Decimal,
    pub comps_low: Decimal,
    pub comps_high: Decimal,
    pub precedents_low: Decimal,
    pub precedents_high: Decimal,
    pub current_price: Option<Decimal>,
}

/// Football Field Valuation Summary Output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootballFieldOutput {
    pub methodologies: Vec<ValuationMethodology>,
    pub overall_low: Decimal,
    pub overall_high: Decimal,
    pub overall_midpoint: Decimal,
    pub current_price: Option<Decimal>,
    pub implied_upside_downside: Option<Decimal>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationMethodology {
    pub method: String,
    pub low: Decimal,
    pub high: Decimal,
    pub midpoint: Decimal,
    pub range_width: Decimal,
}

/// Paper LBO (Mental Math LBO) Input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperLboInput {
    pub purchase_price: Decimal,
    pub ebitda: Decimal,
    pub entry_multiple: Decimal,
    pub debt_multiple: Decimal,  // Debt as multiple of EBITDA (e.g., 5x)
    pub ebitda_growth_rate: Decimal,  // Annual growth % (can be 0 for flat)
    pub hold_period_years: u32,
    pub exit_multiple: Decimal,
    pub interest_rate: Decimal,  // % per annum
}

/// Paper LBO Output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperLboOutput {
    pub entry_valuation: Decimal,
    pub entry_ebitda: Decimal,
    pub entry_multiple: Decimal,
    pub equity_invested: Decimal,
    pub debt_amount: Decimal,
    pub leverage_ratio: Decimal,

    pub exit_ebitda: Decimal,
    pub exit_valuation: Decimal,
    pub exit_multiple: Decimal,

    pub debt_paydown: Decimal,
    pub remaining_debt: Decimal,
    pub exit_equity_value: Decimal,

    pub money_multiple: Decimal,  // MoM (exit equity / entry equity)
    pub irr_percent: Decimal,

    pub key_assumptions: Vec<String>,
    pub mental_math_steps: Vec<String>,
}
