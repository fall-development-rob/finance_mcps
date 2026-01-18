use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::error::{FinanceError, Result};
use crate::types::{DebtCapacityInput, DebtCapacityOutput};

/// Calculate debt capacity based on EBITDA multiples
/// Common in leveraged finance and M&A analysis
pub fn calculate_debt_capacity(input: DebtCapacityInput) -> Result<DebtCapacityOutput> {
    if input.ebitda <= Decimal::ZERO {
        return Err(FinanceError::InvalidInput("EBITDA must be positive".to_string()));
    }

    if input.target_leverage_multiple < Decimal::ZERO {
        return Err(FinanceError::NegativeValue("target_leverage_multiple".to_string()));
    }

    // Maximum debt = EBITDA * target leverage multiple
    let maximum_debt = input.ebitda * input.target_leverage_multiple;

    // Incremental capacity = maximum debt - existing debt
    let incremental_capacity = maximum_debt - input.existing_debt;

    // Net debt capacity = incremental capacity + cash balance
    // (cash can be used to pay down debt or increase borrowing capacity)
    let net_debt_capacity = incremental_capacity + input.cash_balance;

    // Calculate headroom as percentage of maximum debt
    let headroom_percentage = if maximum_debt == Decimal::ZERO {
        Decimal::ZERO
    } else {
        (incremental_capacity / maximum_debt) * dec!(100)
    };

    Ok(DebtCapacityOutput {
        maximum_debt,
        incremental_capacity,
        net_debt_capacity,
        headroom_percentage,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_debt_capacity() {
        let input = DebtCapacityInput {
            ebitda: dec!(50000),
            target_leverage_multiple: dec!(4.5),
            existing_debt: dec!(180000),
            cash_balance: dec!(25000),
        };

        let result = calculate_debt_capacity(input).unwrap();

        // Maximum debt = 50,000 * 4.5 = 225,000
        assert_eq!(result.maximum_debt, dec!(225000));
        // Incremental = 225,000 - 180,000 = 45,000
        assert_eq!(result.incremental_capacity, dec!(45000));
        // Net = 45,000 + 25,000 = 70,000
        assert_eq!(result.net_debt_capacity, dec!(70000));
    }
}
