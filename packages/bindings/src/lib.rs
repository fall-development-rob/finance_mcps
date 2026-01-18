use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde::Deserialize;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
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
        SourcesAndUsesInput,
    },
    valuation::{
        FootballFieldInput, PaperLboInput,
    },

    // Phase 3 functions
    calculate_npv, calculate_irr, calculate_moic,
    build_sources_and_uses, calculate_value_bridge,

    // Phase 3 types
    pe::ValueBridgeInput,
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

// ========== Phase 3 Functions ==========

#[napi]
pub fn calculate_npv_binding(input: String) -> Result<String> {
    #[derive(serde::Deserialize)]
    struct NpvInput {
        cash_flows: Vec<String>,
        discount_rate: String,
    }

    let input: NpvInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let cash_flows: std::result::Result<Vec<Decimal>, _> = input.cash_flows
        .iter()
        .map(|s| s.parse::<Decimal>())
        .collect();
    let cash_flows = cash_flows
        .map_err(|e| Error::from_reason(format!("Invalid cash flow: {}", e)))?;

    let discount_rate: Decimal = input.discount_rate.parse()
        .map_err(|e| Error::from_reason(format!("Invalid discount rate: {}", e)))?;

    let npv = calculate_npv(&cash_flows, discount_rate);

    serde_json::to_string(&npv)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}

#[napi]
pub fn calculate_irr_binding(input: String) -> Result<String> {
    #[derive(serde::Deserialize)]
    struct IrrInput {
        cash_flows: Vec<String>,
        initial_guess: Option<String>,
    }

    let input: IrrInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let cash_flows: std::result::Result<Vec<Decimal>, _> = input.cash_flows
        .iter()
        .map(|s| s.parse::<Decimal>())
        .collect();
    let cash_flows = cash_flows
        .map_err(|e| Error::from_reason(format!("Invalid cash flow: {}", e)))?;

    let initial_guess = if let Some(guess) = input.initial_guess {
        guess.parse::<Decimal>()
            .map_err(|e| Error::from_reason(format!("Invalid initial guess: {}", e)))?
    } else {
        dec!(10.0)
    };

    let irr = calculate_irr(&cash_flows, initial_guess)
        .map_err(|e| Error::from_reason(format!("Calculation error: {}", e)))?;

    serde_json::to_string(&irr)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}

#[napi]
pub fn calculate_moic_binding(input: String) -> Result<String> {
    #[derive(serde::Deserialize)]
    struct MoicInput {
        invested_capital: String,
        realized_value: String,
    }

    let input: MoicInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let invested_capital: Decimal = input.invested_capital.parse()
        .map_err(|e| Error::from_reason(format!("Invalid invested capital: {}", e)))?;
    let realized_value: Decimal = input.realized_value.parse()
        .map_err(|e| Error::from_reason(format!("Invalid realized value: {}", e)))?;

    let moic = calculate_moic(invested_capital, realized_value)
        .map_err(|e| Error::from_reason(format!("Calculation error: {}", e)))?;

    serde_json::to_string(&moic)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}

#[napi]
pub fn sources_and_uses_calc(input: String) -> Result<String> {
    let input: SourcesAndUsesInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let output = build_sources_and_uses(input)
        .map_err(|e| Error::from_reason(format!("Calculation error: {}", e)))?;

    serde_json::to_string(&output)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}

#[napi]
pub fn value_bridge_calc(input: String) -> Result<String> {
    let input: ValueBridgeInput = serde_json::from_str(&input)
        .map_err(|e| Error::from_reason(format!("Invalid input: {}", e)))?;

    let output = calculate_value_bridge(input)
        .map_err(|e| Error::from_reason(format!("Calculation error: {}", e)))?;

    serde_json::to_string(&output)
        .map_err(|e| Error::from_reason(format!("Serialization error: {}", e)))
}
