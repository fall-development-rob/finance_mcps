use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaccInput {
    pub equity_value: Decimal,
    pub debt_value: Decimal,
    pub cost_of_equity: Decimal,  // as percentage, e.g., 12.5
    pub cost_of_debt: Decimal,    // as percentage
    pub tax_rate: Decimal,         // as percentage
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaccOutput {
    pub wacc: Decimal,  // as percentage
    pub equity_weight: Decimal,
    pub debt_weight: Decimal,
    pub after_tax_cost_of_debt: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditMetricsInput {
    pub ebitda: Decimal,
    pub total_debt: Decimal,
    pub interest_expense: Decimal,
    pub ebit: Decimal,
    pub current_assets: Decimal,
    pub current_liabilities: Decimal,
    pub total_assets: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditMetricsOutput {
    pub debt_to_ebitda: Decimal,
    pub interest_coverage: Decimal,
    pub current_ratio: Decimal,
    pub leverage_ratio: Decimal,
    pub rating_indication: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcfInput {
    pub free_cash_flows: Vec<Decimal>,  // Array of FCF projections
    pub discount_rate: Decimal,          // as percentage
    pub terminal_growth_rate: Decimal,   // as percentage
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DcfOutput {
    pub present_values: Vec<Decimal>,
    pub terminal_value: Decimal,
    pub enterprise_value: Decimal,
    pub npv: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebtCapacityInput {
    pub ebitda: Decimal,
    pub target_leverage_multiple: Decimal,
    pub existing_debt: Decimal,
    pub cash_balance: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebtCapacityOutput {
    pub maximum_debt: Decimal,
    pub incremental_capacity: Decimal,
    pub net_debt_capacity: Decimal,
    pub headroom_percentage: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovenantTest {
    pub name: String,
    pub covenant_type: String,  // "maximum", "minimum", "range"
    pub limit: Decimal,
    pub actual: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovenantInput {
    pub tests: Vec<CovenantTest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovenantResult {
    pub test_name: String,
    pub compliant: bool,
    pub limit: Decimal,
    pub actual: Decimal,
    pub headroom: Decimal,
    pub headroom_percentage: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovenantOutput {
    pub overall_compliant: bool,
    pub results: Vec<CovenantResult>,
    pub violations: Vec<String>,
}
