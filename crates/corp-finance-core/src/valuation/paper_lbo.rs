use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::error::Result;
use super::types::{PaperLboInput, PaperLboOutput};

/// Calculate quick Paper LBO (mental math approach)
/// Used in interviews and quick analysis without Excel
///
/// Steps:
/// 1. Entry: Purchase Price = EBITDA × Entry Multiple
/// 2. Financing: Equity = Purchase Price - Debt, where Debt = EBITDA × Debt Multiple
/// 3. Exit EBITDA: Apply growth rate over hold period
/// 4. Exit Value: Exit EBITDA × Exit Multiple
/// 5. Debt Paydown: Use simplified approach (% of EBITDA)
/// 6. Exit Equity: Exit Value - Remaining Debt
/// 7. IRR: Using approximation formulas
pub fn calculate_paper_lbo(input: PaperLboInput) -> Result<PaperLboOutput> {
    let mut mental_math_steps = Vec::new();
    let mut key_assumptions = Vec::new();

    // Entry valuation
    let entry_ebitda = input.ebitda;
    let entry_multiple = input.entry_multiple;
    let entry_valuation = entry_ebitda * entry_multiple;

    mental_math_steps.push(format!(
        "Entry: ${} EBITDA × {}x = ${} entry valuation",
        entry_ebitda, entry_multiple, entry_valuation
    ));

    // Financing structure
    let debt_amount = entry_ebitda * input.debt_multiple;
    let equity_invested = entry_valuation - debt_amount;
    let leverage_ratio = debt_amount / entry_ebitda;

    mental_math_steps.push(format!(
        "Debt: ${} EBITDA × {}x = ${}",
        entry_ebitda, input.debt_multiple, debt_amount
    ));
    mental_math_steps.push(format!(
        "Equity: ${} - ${} = ${} invested",
        entry_valuation, debt_amount, equity_invested
    ));

    key_assumptions.push(format!(
        "Entry leverage: {:.1}x EBITDA",
        leverage_ratio
    ));

    // Project exit EBITDA with growth
    let growth_rate_decimal = input.ebitda_growth_rate / dec!(100);

    // Calculate growth multiplier: (1 + rate)^years
    // Since rust_decimal doesn't have powi, we'll do it manually
    let mut growth_multiplier = Decimal::ONE;
    for _ in 0..input.hold_period_years {
        growth_multiplier *= Decimal::ONE + growth_rate_decimal;
    }
    let exit_ebitda = entry_ebitda * growth_multiplier;

    if input.ebitda_growth_rate != Decimal::ZERO {
        mental_math_steps.push(format!(
            "EBITDA growth: ${} × (1 + {}%)^{} years = ${}",
            entry_ebitda,
            input.ebitda_growth_rate,
            input.hold_period_years,
            exit_ebitda
        ));
        key_assumptions.push(format!(
            "{}% annual EBITDA growth",
            input.ebitda_growth_rate
        ));
    } else {
        mental_math_steps.push(format!(
            "EBITDA flat at ${} (no growth assumed)",
            exit_ebitda
        ));
    }

    // Exit valuation
    let exit_multiple = input.exit_multiple;
    let exit_valuation = exit_ebitda * exit_multiple;

    mental_math_steps.push(format!(
        "Exit: ${} EBITDA × {}x = ${} exit valuation",
        exit_ebitda, exit_multiple, exit_valuation
    ));

    key_assumptions.push(format!(
        "Exit at {}x EBITDA",
        exit_multiple
    ));

    // Simplified debt paydown
    // Assume debt is paid down using free cash flow
    // Simplified: 50% of cumulative EBITDA goes to debt paydown
    let cumulative_ebitda = if input.ebitda_growth_rate == Decimal::ZERO {
        exit_ebitda * Decimal::from(input.hold_period_years)
    } else {
        // Simplified: average EBITDA × years
        let avg_ebitda = (entry_ebitda + exit_ebitda) / dec!(2);
        avg_ebitda * Decimal::from(input.hold_period_years)
    };

    let debt_paydown = (cumulative_ebitda * dec!(0.5)).min(debt_amount);
    let remaining_debt = debt_amount - debt_paydown;

    mental_math_steps.push(format!(
        "Debt paydown: ~50% of cumulative EBITDA = ${} paid down",
        debt_paydown
    ));

    key_assumptions.push(format!(
        "{}% interest rate on debt",
        input.interest_rate
    ));

    // Exit equity value
    let exit_equity_value = exit_valuation - remaining_debt;

    mental_math_steps.push(format!(
        "Exit equity: ${} valuation - ${} remaining debt = ${}",
        exit_valuation, remaining_debt, exit_equity_value
    ));

    // Money multiple
    let money_multiple = if equity_invested > Decimal::ZERO {
        exit_equity_value / equity_invested
    } else {
        Decimal::ZERO
    };

    mental_math_steps.push(format!(
        "Money multiple: ${} / ${} = {:.2}x",
        exit_equity_value, equity_invested, money_multiple
    ));

    // IRR calculation using approximation
    // IRR ≈ (MoM^(1/years) - 1) × 100
    // For mental math, use rule of 72 approximation
    let irr_percent = if input.hold_period_years > 0 && money_multiple > Decimal::ZERO {
        // More accurate: (MoM^(1/n) - 1) × 100
        // Approximation for mental math:
        let years = Decimal::from(input.hold_period_years);

        // Simple approximation: (MoM - 1) / years × 100 for rough estimate
        // Better approximation: Use (MoM^(1/n) - 1) which we can approximate
        if money_multiple == dec!(2) {
            // Rule of 72: 72/years ≈ IRR for 2x
            dec!(72) / years
        } else if money_multiple == dec!(3) {
            // Rule of 114 for 3x
            dec!(114) / years
        } else {
            // General approximation: (MoM - 1) / years × 100
            // This is simplified and less accurate but easier for mental math
            ((money_multiple - Decimal::ONE) / years) * dec!(100)
        }
    } else {
        Decimal::ZERO
    };

    mental_math_steps.push(format!(
        "IRR approximation: {:.1}% per year over {} years",
        irr_percent, input.hold_period_years
    ));

    Ok(PaperLboOutput {
        entry_valuation,
        entry_ebitda,
        entry_multiple,
        equity_invested,
        debt_amount,
        leverage_ratio,
        exit_ebitda,
        exit_valuation,
        exit_multiple,
        debt_paydown,
        remaining_debt,
        exit_equity_value,
        money_multiple,
        irr_percent,
        key_assumptions,
        mental_math_steps,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_paper_lbo_basic() {
        let input = PaperLboInput {
            purchase_price: dec!(1000),
            ebitda: dec!(100),
            entry_multiple: dec!(10),
            debt_multiple: dec!(5),
            ebitda_growth_rate: dec!(0),  // Flat EBITDA
            hold_period_years: 5,
            exit_multiple: dec!(10),
            interest_rate: dec!(5),
        };

        let result = calculate_paper_lbo(input).unwrap();

        // Entry
        assert_eq!(result.entry_valuation, dec!(1000));
        assert_eq!(result.entry_ebitda, dec!(100));

        // Financing
        assert_eq!(result.debt_amount, dec!(500));  // 100 × 5
        assert_eq!(result.equity_invested, dec!(500));  // 1000 - 500

        // Exit (flat EBITDA)
        assert_eq!(result.exit_ebitda, dec!(100));
        assert_eq!(result.exit_valuation, dec!(1000));  // 100 × 10

        // Returns
        assert!(result.money_multiple > Decimal::ZERO);
        assert!(result.irr_percent > Decimal::ZERO);
    }

    #[test]
    fn test_paper_lbo_with_growth() {
        let input = PaperLboInput {
            purchase_price: dec!(1000),
            ebitda: dec!(100),
            entry_multiple: dec!(10),
            debt_multiple: dec!(5),
            ebitda_growth_rate: dec!(10),  // 10% annual growth
            hold_period_years: 5,
            exit_multiple: dec!(10),
            interest_rate: dec!(5),
        };

        let result = calculate_paper_lbo(input).unwrap();

        // Exit EBITDA should be higher due to growth
        // 100 × 1.1^5 ≈ 161
        assert!(result.exit_ebitda > dec!(160));
        assert!(result.exit_ebitda < dec!(162));

        // Exit valuation should be proportionally higher
        assert!(result.exit_valuation > dec!(1600));

        // Better returns with growth
        assert!(result.money_multiple > dec!(2));
    }

    #[test]
    fn test_paper_lbo_multiple_expansion() {
        let input = PaperLboInput {
            purchase_price: dec!(1000),
            ebitda: dec!(100),
            entry_multiple: dec!(10),
            debt_multiple: dec!(5),
            ebitda_growth_rate: dec!(0),
            hold_period_years: 5,
            exit_multiple: dec!(12),  // Multiple expansion
            interest_rate: dec!(5),
        };

        let result = calculate_paper_lbo(input).unwrap();

        // Exit valuation benefits from multiple expansion
        assert_eq!(result.exit_valuation, dec!(1200));  // 100 × 12

        // Higher returns due to multiple expansion
        assert!(result.money_multiple > dec!(1));
    }

    #[test]
    fn test_paper_lbo_high_leverage() {
        let input = PaperLboInput {
            purchase_price: dec!(1000),
            ebitda: dec!(100),
            entry_multiple: dec!(10),
            debt_multiple: dec!(6),  // 6x leverage
            ebitda_growth_rate: dec!(5),
            hold_period_years: 5,
            exit_multiple: dec!(10),
            interest_rate: dec!(6),
        };

        let result = calculate_paper_lbo(input).unwrap();

        // Higher leverage
        assert_eq!(result.debt_amount, dec!(600));
        assert_eq!(result.equity_invested, dec!(400));
        assert_eq!(result.leverage_ratio, dec!(6));

        // Higher leverage should lead to higher returns (if exit works out)
        assert!(result.money_multiple > dec!(1.5));
    }
}
