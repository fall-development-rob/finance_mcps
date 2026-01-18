use napi::bindgen_prelude::*;
use napi_derive::napi;
use corp_finance_core::{
    calculate_wacc, calculate_credit_metrics, calculate_dcf,
    calculate_debt_capacity, check_covenant_compliance,
    WaccInput, WaccOutput,
    CreditMetricsInput, CreditMetricsOutput,
    DcfInput, DcfOutput,
    DebtCapacityInput, DebtCapacityOutput,
    CovenantInput, CovenantOutput,
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
