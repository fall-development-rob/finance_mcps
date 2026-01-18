use rust_decimal::Decimal;
use crate::error::Result;
use super::types::{AccountingFlowInput, AccountingFlowOutput, AccountingImpact};

/// Analyze impact of a transaction on all three financial statements
/// "Walk me through" questions - classic interview format
pub fn analyze_accounting_flow(input: AccountingFlowInput) -> Result<AccountingFlowOutput> {
    let mut income_statement_impact = Vec::new();
    let mut balance_sheet_impact = Vec::new();
    let mut cash_flow_impact = Vec::new();
    let explanation;

    match input.transaction_type.as_str() {
        "depreciation" => {
            explanation = format!(
                "Depreciation of ${} is a non-cash expense that reduces net income but doesn't affect cash.",
                input.amount
            );

            income_statement_impact.push(AccountingImpact {
                line_item: "Depreciation Expense".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });
            income_statement_impact.push(AccountingImpact {
                line_item: "Net Income".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });

            balance_sheet_impact.push(AccountingImpact {
                line_item: "PP&E (Accumulated Depreciation)".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });
            balance_sheet_impact.push(AccountingImpact {
                line_item: "Retained Earnings".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });

            cash_flow_impact.push(AccountingImpact {
                line_item: "Net Income".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });
            cash_flow_impact.push(AccountingImpact {
                line_item: "Add: Depreciation".to_string(),
                impact: input.amount,
                sign: "positive".to_string(),
            });
            cash_flow_impact.push(AccountingImpact {
                line_item: "Cash from Operations".to_string(),
                impact: Decimal::ZERO,
                sign: "neutral".to_string(),
            });
        }

        "amortization" => {
            explanation = format!(
                "Amortization of ${} reduces net income but is added back in cash flow as a non-cash expense.",
                input.amount
            );

            income_statement_impact.push(AccountingImpact {
                line_item: "Amortization Expense".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });
            income_statement_impact.push(AccountingImpact {
                line_item: "Net Income".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });

            balance_sheet_impact.push(AccountingImpact {
                line_item: "Intangible Assets".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });
            balance_sheet_impact.push(AccountingImpact {
                line_item: "Retained Earnings".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });

            cash_flow_impact.push(AccountingImpact {
                line_item: "Net Income".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });
            cash_flow_impact.push(AccountingImpact {
                line_item: "Add: Amortization".to_string(),
                impact: input.amount,
                sign: "positive".to_string(),
            });
        }

        "capex" => {
            explanation = format!(
                "CapEx of ${} increases PP&E and reduces cash. It's an investing activity with no immediate P&L impact.",
                input.amount
            );

            income_statement_impact.push(AccountingImpact {
                line_item: "No immediate impact".to_string(),
                impact: Decimal::ZERO,
                sign: "neutral".to_string(),
            });

            balance_sheet_impact.push(AccountingImpact {
                line_item: "PP&E".to_string(),
                impact: input.amount,
                sign: "positive".to_string(),
            });
            balance_sheet_impact.push(AccountingImpact {
                line_item: "Cash".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });

            cash_flow_impact.push(AccountingImpact {
                line_item: "CapEx (Investing Activities)".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });
        }

        "debt_issuance" => {
            explanation = format!(
                "Issuing ${} of debt increases cash and debt on balance sheet. No P&L impact, but future interest will affect earnings.",
                input.amount
            );

            income_statement_impact.push(AccountingImpact {
                line_item: "No immediate impact".to_string(),
                impact: Decimal::ZERO,
                sign: "neutral".to_string(),
            });

            balance_sheet_impact.push(AccountingImpact {
                line_item: "Cash".to_string(),
                impact: input.amount,
                sign: "positive".to_string(),
            });
            balance_sheet_impact.push(AccountingImpact {
                line_item: "Debt".to_string(),
                impact: input.amount,
                sign: "positive".to_string(),
            });

            cash_flow_impact.push(AccountingImpact {
                line_item: "Debt Issuance (Financing Activities)".to_string(),
                impact: input.amount,
                sign: "positive".to_string(),
            });
        }

        "debt_repayment" => {
            explanation = format!(
                "Repaying ${} of debt reduces both cash and debt. No P&L impact, but reduces future interest expense.",
                input.amount
            );

            income_statement_impact.push(AccountingImpact {
                line_item: "No immediate impact".to_string(),
                impact: Decimal::ZERO,
                sign: "neutral".to_string(),
            });

            balance_sheet_impact.push(AccountingImpact {
                line_item: "Cash".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });
            balance_sheet_impact.push(AccountingImpact {
                line_item: "Debt".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });

            cash_flow_impact.push(AccountingImpact {
                line_item: "Debt Repayment (Financing Activities)".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });
        }

        "inventory_purchase" => {
            explanation = format!(
                "Purchasing ${} of inventory increases inventory and reduces cash. No P&L impact until sold.",
                input.amount
            );

            income_statement_impact.push(AccountingImpact {
                line_item: "No immediate impact".to_string(),
                impact: Decimal::ZERO,
                sign: "neutral".to_string(),
            });

            balance_sheet_impact.push(AccountingImpact {
                line_item: "Inventory".to_string(),
                impact: input.amount,
                sign: "positive".to_string(),
            });
            balance_sheet_impact.push(AccountingImpact {
                line_item: "Cash".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });

            cash_flow_impact.push(AccountingImpact {
                line_item: "Change in NWC - Inventory".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });
        }

        "revenue_recognition" => {
            explanation = format!(
                "Recognizing ${} of revenue increases A/R and revenue. Cash collected later.",
                input.amount
            );

            income_statement_impact.push(AccountingImpact {
                line_item: "Revenue".to_string(),
                impact: input.amount,
                sign: "positive".to_string(),
            });
            income_statement_impact.push(AccountingImpact {
                line_item: "Net Income".to_string(),
                impact: input.amount,
                sign: "positive".to_string(),
            });

            balance_sheet_impact.push(AccountingImpact {
                line_item: "Accounts Receivable".to_string(),
                impact: input.amount,
                sign: "positive".to_string(),
            });
            balance_sheet_impact.push(AccountingImpact {
                line_item: "Retained Earnings".to_string(),
                impact: input.amount,
                sign: "positive".to_string(),
            });

            cash_flow_impact.push(AccountingImpact {
                line_item: "Net Income".to_string(),
                impact: input.amount,
                sign: "positive".to_string(),
            });
            cash_flow_impact.push(AccountingImpact {
                line_item: "Change in NWC - A/R Increase".to_string(),
                impact: input.amount,
                sign: "negative".to_string(),
            });
        }

        _ => {
            explanation = format!(
                "Transaction type '{}' not recognized. Supported types: depreciation, amortization, capex, debt_issuance, debt_repayment, inventory_purchase, revenue_recognition",
                input.transaction_type
            );
        }
    }

    Ok(AccountingFlowOutput {
        transaction: input.transaction,
        amount: input.amount,
        income_statement_impact,
        balance_sheet_impact,
        cash_flow_impact,
        explanation,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_depreciation_flow() {
        let input = AccountingFlowInput {
            transaction: "Annual depreciation".to_string(),
            amount: dec!(100),
            transaction_type: "depreciation".to_string(),
        };

        let result = analyze_accounting_flow(input).unwrap();

        assert_eq!(result.income_statement_impact.len(), 2);
        assert_eq!(result.balance_sheet_impact.len(), 2);
        assert_eq!(result.cash_flow_impact.len(), 3);

        // Verify no net cash impact
        let cfo_impact = result.cash_flow_impact.iter()
            .find(|i| i.line_item == "Cash from Operations")
            .unwrap();
        assert_eq!(cfo_impact.impact, Decimal::ZERO);
    }

    #[test]
    fn test_capex_flow() {
        let input = AccountingFlowInput {
            transaction: "Purchase equipment".to_string(),
            amount: dec!(500),
            transaction_type: "capex".to_string(),
        };

        let result = analyze_accounting_flow(input).unwrap();

        // CapEx has no immediate P&L impact
        assert_eq!(result.income_statement_impact[0].sign, "neutral");

        // Should increase PP&E and decrease cash
        assert_eq!(result.balance_sheet_impact.len(), 2);
    }

    #[test]
    fn test_debt_issuance() {
        let input = AccountingFlowInput {
            transaction: "Issue bonds".to_string(),
            amount: dec!(1000),
            transaction_type: "debt_issuance".to_string(),
        };

        let result = analyze_accounting_flow(input).unwrap();

        // No P&L impact
        assert_eq!(result.income_statement_impact[0].sign, "neutral");

        // Increases cash and debt
        assert_eq!(result.balance_sheet_impact.len(), 2);
        assert_eq!(result.cash_flow_impact.len(), 1);
    }
}
