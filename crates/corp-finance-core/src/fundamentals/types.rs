use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Three Statement Model Input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeStatementInput {
    // Starting balances
    pub starting_cash: Decimal,
    pub starting_debt: Decimal,
    pub starting_equity: Decimal,
    pub starting_inventory: Decimal,
    pub starting_ar: Decimal,
    pub starting_ap: Decimal,
    pub starting_ppe: Decimal,

    // Projections (annual)
    pub revenue: Vec<Decimal>,
    pub cogs_percent: Decimal,          // as % of revenue
    pub opex_percent: Decimal,          // as % of revenue
    pub tax_rate: Decimal,              // as %
    pub capex: Vec<Decimal>,
    pub depreciation: Vec<Decimal>,
    pub nwc_percent_revenue: Decimal,   // NWC as % of revenue
    pub interest_rate: Decimal,         // as %
}

/// Three Statement Model Output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeStatementOutput {
    pub income_statements: Vec<IncomeStatement>,
    pub balance_sheets: Vec<BalanceSheet>,
    pub cash_flows: Vec<CashFlow>,
    pub years: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeStatement {
    pub revenue: Decimal,
    pub cogs: Decimal,
    pub gross_profit: Decimal,
    pub opex: Decimal,
    pub ebitda: Decimal,
    pub depreciation: Decimal,
    pub ebit: Decimal,
    pub interest_expense: Decimal,
    pub ebt: Decimal,
    pub tax: Decimal,
    pub net_income: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSheet {
    pub cash: Decimal,
    pub accounts_receivable: Decimal,
    pub inventory: Decimal,
    pub ppe_net: Decimal,
    pub total_assets: Decimal,
    pub accounts_payable: Decimal,
    pub debt: Decimal,
    pub equity: Decimal,
    pub total_liabilities_equity: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashFlow {
    pub net_income: Decimal,
    pub depreciation: Decimal,
    pub change_in_nwc: Decimal,
    pub cfo: Decimal,
    pub capex: Decimal,
    pub cfi: Decimal,
    pub debt_issuance: Decimal,
    pub cff: Decimal,
    pub net_change_cash: Decimal,
}

/// Equity Enterprise Bridge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityEnterpriseInput {
    pub direction: String,  // "equity_to_ev" or "ev_to_equity"
    pub value: Decimal,
    pub cash: Decimal,
    pub debt: Decimal,
    pub minority_interest: Decimal,
    pub associates: Decimal,
    pub preferred_stock: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityEnterpriseOutput {
    pub equity_value: Decimal,
    pub enterprise_value: Decimal,
    pub net_debt: Decimal,
    pub bridge_items: Vec<BridgeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeItem {
    pub item: String,
    pub amount: Decimal,
    pub direction: String,  // "add" or "subtract"
}

/// Diluted Shares Calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DilutedSharesInput {
    pub basic_shares: Decimal,
    pub stock_price: Decimal,
    pub options: Vec<OptionGrant>,
    pub rsus: Decimal,
    pub convertibles: Vec<Convertible>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionGrant {
    pub quantity: Decimal,
    pub strike_price: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Convertible {
    pub principal: Decimal,
    pub conversion_price: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DilutedSharesOutput {
    pub basic_shares: Decimal,
    pub options_dilution: Decimal,
    pub rsu_dilution: Decimal,
    pub convertibles_dilution: Decimal,
    pub fully_diluted_shares: Decimal,
    pub dilution_percentage: Decimal,
    pub breakdown: Vec<DilutionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DilutionItem {
    pub source: String,
    pub shares: Decimal,
    pub method: String,
}

/// Accounting Flows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountingFlowInput {
    pub transaction: String,
    pub amount: Decimal,
    pub transaction_type: String,  // "depreciation", "amortization", "capex", "debt_issuance", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountingFlowOutput {
    pub transaction: String,
    pub amount: Decimal,
    pub income_statement_impact: Vec<AccountingImpact>,
    pub balance_sheet_impact: Vec<AccountingImpact>,
    pub cash_flow_impact: Vec<AccountingImpact>,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountingImpact {
    pub line_item: String,
    pub impact: Decimal,
    pub sign: String,  // "positive", "negative", "neutral"
}
