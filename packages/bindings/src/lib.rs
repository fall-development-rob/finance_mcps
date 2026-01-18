use napi::bindgen_prelude::*;
use napi_derive::napi;
use corp_finance_core::{
    // Phase 1 functions
    calculate_wacc, calculate_credit_metrics, calculate_dcf,
    calculate_debt_capacity, check_covenant_compliance,
    WaccInput, CreditMetricsInput, DcfInput, DebtCapacityInput, CovenantInput,

    // Phase 2 functions
    build_three_statement_model, equity_enterprise_bridge,
    calculate_diluted_shares, analyze_accounting_flow,
    create_football_field, calculate_paper_lbo,

    // Phase 2 types
    fundamentals::{
        ThreeStatementInput, EquityEnterpriseInput,
        DilutedSharesInput, AccountingFlowInput,
    },
    valuation::{
        FootballFieldInput, PaperLboInput,
    },
};

#[napi]
pub fn wacc_calculator(input: String) -> Result<String> {
    let input: WaccInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let output = calculate_wacc(input)
        .map_err(|e| Error::from_reason(format!("Calculation error: {}", e)))?;

    serde_json::to_string(&output)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}

#[napi]
pub fn credit_metrics(input: String) -> Result<String> {
    let input: CreditMetricsInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let output = calculate_credit_metrics(input)
        .map_err(|e| Error::from_reason(format!("Calculation error: {}", e)))?;

    serde_json::to_string(&output)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}

#[napi]
pub fn dcf_model(input: String) -> Result<String> {
    let input: DcfInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let output = calculate_dcf(input)
        .map_err(|e| Error::from_reason(format!("Calculation error: {}", e)))?;

    serde_json::to_string(&output)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}

#[napi]
pub fn debt_capacity(input: String) -> Result<String> {
    let input: DebtCapacityInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let output = calculate_debt_capacity(input)
        .map_err(|e| Error::from_reason(format!("Calculation error: {}", e)))?;

    serde_json::to_string(&output)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}

#[napi]
pub fn covenant_compliance(input: String) -> Result<String> {
    let input: CovenantInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let output = check_covenant_compliance(input)
        .map_err(|e| Error::from_reason(format!("Calculation error: {}", e)))?;

    serde_json::to_string(&output)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}

// ========== Phase 2 Functions ==========

#[napi]
pub fn three_statement_model(input: String) -> Result<String> {
    let input: ThreeStatementInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let output = build_three_statement_model(input)
        .map_err(|e| Error::from_reason(format!("Calculation error: {}", e)))?;

    serde_json::to_string(&output)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}

#[napi]
pub fn equity_enterprise_bridge_calc(input: String) -> Result<String> {
    let input: EquityEnterpriseInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let output = equity_enterprise_bridge(input)
        .map_err(|e| Error::from_reason(format!("Calculation error: {}", e)))?;

    serde_json::to_string(&output)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}

#[napi]
pub fn diluted_shares_calc(input: String) -> Result<String> {
    let input: DilutedSharesInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let output = calculate_diluted_shares(input)
        .map_err(|e| Error::from_reason(format!("Calculation error: {}", e)))?;

    serde_json::to_string(&output)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}

#[napi]
pub fn accounting_flow_analysis(input: String) -> Result<String> {
    let input: AccountingFlowInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let output = analyze_accounting_flow(input)
        .map_err(|e| Error::from_reason(format!("Calculation error: {}", e)))?;

    serde_json::to_string(&output)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}

#[napi]
pub fn football_field_valuation(input: String) -> Result<String> {
    let input: FootballFieldInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let output = create_football_field(input)
        .map_err(|e| Error::from_reason(format!("Calculation error: {}", e)))?;

    serde_json::to_string(&output)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}

#[napi]
pub fn paper_lbo_calc(input: String) -> Result<String> {
    let input: PaperLboInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let output = calculate_paper_lbo(input)
        .map_err(|e| Error::from_reason(format!("Calculation error: {}", e)))?;

    serde_json::to_string(&output)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}
