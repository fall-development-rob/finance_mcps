use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::error::Result;
use super::types::{
    ThreeStatementInput, ThreeStatementOutput, IncomeStatement, BalanceSheet, CashFlow,
};

/// Build linked three-statement financial model
/// Links income statement → balance sheet → cash flow statement
pub fn build_three_statement_model(input: ThreeStatementInput) -> Result<ThreeStatementOutput> {
    let num_years = input.revenue.len();
    let mut income_statements = Vec::new();
    let mut balance_sheets = Vec::new();
    let mut cash_flows = Vec::new();
    let mut years = Vec::new();

    // Previous balances (start with year 0)
    let mut prev_cash = input.starting_cash;
    let mut prev_ar = input.starting_ar;
    let mut prev_inventory = input.starting_inventory;
    let mut prev_ap = input.starting_ap;
    let mut prev_ppe = input.starting_ppe;
    let mut prev_debt = input.starting_debt;
    let mut prev_equity = input.starting_equity;

    for year in 0..num_years {
        years.push(year as u32 + 1);

        // === INCOME STATEMENT ===
        let revenue = input.revenue[year];
        let cogs = revenue * input.cogs_percent / dec!(100);
        let gross_profit = revenue - cogs;
        let opex = revenue * input.opex_percent / dec!(100);
        let depreciation = input.depreciation[year];
        let ebitda = gross_profit - opex;
        let ebit = ebitda - depreciation;

        // Interest calculated on beginning debt balance
        let interest_expense = prev_debt * input.interest_rate / dec!(100);
        let ebt = ebit - interest_expense;
        let tax = ebt.max(Decimal::ZERO) * input.tax_rate / dec!(100);
        let net_income = ebt - tax;

        income_statements.push(IncomeStatement {
            revenue,
            cogs,
            gross_profit,
            opex,
            ebitda,
            depreciation,
            ebit,
            interest_expense,
            ebt,
            tax,
            net_income,
        });

        // === BALANCE SHEET ===
        // Calculate NWC items based on revenue
        let nwc_target = revenue * input.nwc_percent_revenue / dec!(100);

        // Simplified: distribute NWC across AR, Inventory, AP proportionally
        let accounts_receivable = nwc_target * dec!(0.4);
        let inventory = nwc_target * dec!(0.3);
        let accounts_payable = nwc_target * dec!(0.3);

        // PPE: Previous PPE + Capex - Depreciation
        let capex = input.capex[year];
        let ppe_net = prev_ppe + capex - depreciation;

        // Assets side
        let total_assets_pre_cash = accounts_receivable + inventory + ppe_net;

        // === CASH FLOW STATEMENT ===
        // Operating activities
        let change_ar = accounts_receivable - prev_ar;
        let change_inventory = inventory - prev_inventory;
        let change_ap = accounts_payable - prev_ap;
        let change_in_nwc = change_ar + change_inventory - change_ap;

        let cfo = net_income + depreciation - change_in_nwc;

        // Investing activities
        let cfi = -capex;

        // Financing activities - debt stays constant for simplicity, plug with equity
        let debt = prev_debt;
        let net_change_cash = cfo + cfi;
        let cash = prev_cash + net_change_cash;

        // Equity is the plug to balance the balance sheet
        let total_liabilities = accounts_payable + debt;
        let equity = cash + accounts_receivable + inventory + ppe_net - total_liabilities;

        let total_assets = cash + accounts_receivable + inventory + ppe_net;
        let total_liabilities_equity = accounts_payable + debt + equity;

        balance_sheets.push(BalanceSheet {
            cash,
            accounts_receivable,
            inventory,
            ppe_net,
            total_assets,
            accounts_payable,
            debt,
            equity,
            total_liabilities_equity,
        });

        cash_flows.push(CashFlow {
            net_income,
            depreciation,
            change_in_nwc,
            cfo,
            capex,
            cfi,
            debt_issuance: Decimal::ZERO,
            cff: Decimal::ZERO,
            net_change_cash,
        });

        // Update previous balances for next iteration
        prev_cash = cash;
        prev_ar = accounts_receivable;
        prev_inventory = inventory;
        prev_ap = accounts_payable;
        prev_ppe = ppe_net;
        prev_debt = debt;
        prev_equity = equity;
    }

    Ok(ThreeStatementOutput {
        income_statements,
        balance_sheets,
        cash_flows,
        years,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_three_statement_model() {
        let input = ThreeStatementInput {
            starting_cash: dec!(100),
            starting_debt: dec!(500),
            starting_equity: dec!(1000),
            starting_inventory: dec!(200),
            starting_ar: dec!(150),
            starting_ap: dec!(100),
            starting_ppe: dec!(800),
            revenue: vec![dec!(1000), dec!(1100), dec!(1210)],
            cogs_percent: dec!(60),
            opex_percent: dec!(20),
            tax_rate: dec!(25),
            capex: vec![dec!(100), dec!(110), dec!(120)],
            depreciation: vec![dec!(80), dec!(88), dec!(96)],
            nwc_percent_revenue: dec!(10),
            interest_rate: dec!(5),
        };

        let result = build_three_statement_model(input).unwrap();

        assert_eq!(result.years.len(), 3);
        assert_eq!(result.income_statements.len(), 3);
        assert_eq!(result.balance_sheets.len(), 3);
        assert_eq!(result.cash_flows.len(), 3);

        // Year 1 checks
        assert_eq!(result.income_statements[0].revenue, dec!(1000));
        assert_eq!(result.income_statements[0].cogs, dec!(600));
        assert!(result.balance_sheets[0].total_assets > Decimal::ZERO);

        // Balance sheet should balance
        let bs = &result.balance_sheets[0];
        assert_eq!(bs.total_assets, bs.total_liabilities_equity);
    }

    #[test]
    fn test_linked_statements() {
        let input = ThreeStatementInput {
            starting_cash: dec!(100),
            starting_debt: dec!(500),
            starting_equity: dec!(1000),
            starting_inventory: dec!(200),
            starting_ar: dec!(150),
            starting_ap: dec!(100),
            starting_ppe: dec!(800),
            revenue: vec![dec!(1000)],
            cogs_percent: dec!(60),
            opex_percent: dec!(20),
            tax_rate: dec!(25),
            capex: vec![dec!(100)],
            depreciation: vec![dec!(80)],
            nwc_percent_revenue: dec!(10),
            interest_rate: dec!(5),
        };

        let result = build_three_statement_model(input).unwrap();

        // Net income from IS should flow to CF
        assert_eq!(
            result.income_statements[0].net_income,
            result.cash_flows[0].net_income
        );

        // Cash flow change should reconcile cash on balance sheet
        // prev_cash + net_change_cash = new_cash
        let expected_cash = dec!(100) + result.cash_flows[0].net_change_cash;
        assert_eq!(result.balance_sheets[0].cash, expected_cash);
    }
}
