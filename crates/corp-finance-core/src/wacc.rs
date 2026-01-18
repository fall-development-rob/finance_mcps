use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::error::{FinanceError, Result};
use crate::types::{WaccInput, WaccOutput};

/// Calculate Weighted Average Cost of Capital (WACC)
/// Formula: WACC = (E/V) * Re + (D/V) * Rd * (1 - Tc)
/// where:
/// - E = market value of equity
/// - D = market value of debt
/// - V = E + D (total value)
/// - Re = cost of equity
/// - Rd = cost of debt
/// - Tc = corporate tax rate
pub fn calculate_wacc(input: WaccInput) -> Result<WaccOutput> {
    // Validation
    if input.equity_value < Decimal::ZERO {
        return Err(FinanceError::NegativeValue("equity_value".to_string()));
    }
    if input.debt_value < Decimal::ZERO {
        return Err(FinanceError::NegativeValue("debt_value".to_string()));
    }

    let total_value = input.equity_value + input.debt_value;

    if total_value == Decimal::ZERO {
        return Err(FinanceError::DivisionByZero("total_value (equity + debt)".to_string()));
    }

    // Calculate weights
    let equity_weight = input.equity_value / total_value;
    let debt_weight = input.debt_value / total_value;

    // Convert percentages to decimals for calculation
    let cost_of_equity_decimal = input.cost_of_equity / dec!(100);
    let cost_of_debt_decimal = input.cost_of_debt / dec!(100);
    let tax_rate_decimal = input.tax_rate / dec!(100);

    // Calculate after-tax cost of debt
    let after_tax_cost_of_debt = cost_of_debt_decimal * (Decimal::ONE - tax_rate_decimal);

    // Calculate WACC
    let wacc_decimal = (equity_weight * cost_of_equity_decimal)
                     + (debt_weight * after_tax_cost_of_debt);

    // Convert back to percentage
    let wacc = wacc_decimal * dec!(100);
    let after_tax_cost_of_debt_pct = after_tax_cost_of_debt * dec!(100);

    Ok(WaccOutput {
        wacc,
        equity_weight,
        debt_weight,
        after_tax_cost_of_debt: after_tax_cost_of_debt_pct,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_wacc_calculation() {
        let input = WaccInput {
            equity_value: dec!(700000),
            debt_value: dec!(300000),
            cost_of_equity: dec!(12.5),
            cost_of_debt: dec!(6.0),
            tax_rate: dec!(25.0),
        };

        let result = calculate_wacc(input).unwrap();

        assert_eq!(result.equity_weight, dec!(0.7));
        assert_eq!(result.debt_weight, dec!(0.3));
        // WACC = 0.7 * 12.5% + 0.3 * 6% * (1 - 0.25) = 8.75% + 1.35% = 10.1%
        assert_eq!(result.wacc, dec!(10.1));
    }
}
