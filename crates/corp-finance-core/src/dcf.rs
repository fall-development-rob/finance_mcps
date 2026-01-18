use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::error::{FinanceError, Result};
use crate::types::{DcfInput, DcfOutput};

/// Helper function to calculate decimal power for positive integer exponents
fn power_decimal(base: Decimal, exp: usize) -> Result<Decimal> {
    if exp == 0 {
        return Ok(Decimal::ONE);
    }

    let mut result = base;
    for _ in 1..exp {
        result = result.checked_mul(base)
            .ok_or_else(|| FinanceError::CalculationError("power calculation overflow".to_string()))?;
    }

    Ok(result)
}

/// Calculate Discounted Cash Flow (DCF) valuation
/// Includes terminal value calculation using perpetual growth method
pub fn calculate_dcf(input: DcfInput) -> Result<DcfOutput> {
    if input.free_cash_flows.is_empty() {
        return Err(FinanceError::InvalidInput("free_cash_flows cannot be empty".to_string()));
    }

    if input.discount_rate <= Decimal::ZERO {
        return Err(FinanceError::InvalidInput("discount_rate must be positive".to_string()));
    }

    if input.terminal_growth_rate >= input.discount_rate {
        return Err(FinanceError::InvalidInput(
            "terminal_growth_rate must be less than discount_rate".to_string()
        ));
    }

    let discount_rate_decimal = input.discount_rate / dec!(100);
    let terminal_growth_decimal = input.terminal_growth_rate / dec!(100);

    let mut present_values = Vec::new();
    let mut total_pv = Decimal::ZERO;

    // Calculate present value of each cash flow
    for (period, fcf) in input.free_cash_flows.iter().enumerate() {
        let period_num = period + 1;
        let discount_factor = power_decimal(Decimal::ONE + discount_rate_decimal, period_num)?;

        let pv = fcf / discount_factor;
        present_values.push(pv);
        total_pv += pv;
    }

    // Calculate terminal value
    // TV = FCF_final * (1 + g) / (r - g)
    let final_fcf = input.free_cash_flows.last().unwrap();
    let terminal_fcf = final_fcf * (Decimal::ONE + terminal_growth_decimal);
    let terminal_value_undisc = terminal_fcf / (discount_rate_decimal - terminal_growth_decimal);

    // Discount terminal value to present
    let n_periods = input.free_cash_flows.len();
    let terminal_discount_factor = power_decimal(Decimal::ONE + discount_rate_decimal, n_periods)?;

    let terminal_value = terminal_value_undisc / terminal_discount_factor;

    // Enterprise value = sum of PV of FCFs + terminal value
    let enterprise_value = total_pv + terminal_value;
    let npv = enterprise_value;

    Ok(DcfOutput {
        present_values,
        terminal_value,
        enterprise_value,
        npv,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_dcf_calculation() {
        let input = DcfInput {
            free_cash_flows: vec![dec!(10000), dec!(11000), dec!(12100), dec!(13310), dec!(14641)],
            discount_rate: dec!(10.0),
            terminal_growth_rate: dec!(2.5),
        };

        let result = calculate_dcf(input).unwrap();

        assert_eq!(result.present_values.len(), 5);
        assert!(result.terminal_value > Decimal::ZERO);
        assert!(result.enterprise_value > Decimal::ZERO);
    }
}
