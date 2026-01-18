use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::error::{FinanceError, Result};
use crate::types::{CreditMetricsInput, CreditMetricsOutput};

/// Calculate key credit metrics for corporate debt analysis
pub fn calculate_credit_metrics(input: CreditMetricsInput) -> Result<CreditMetricsOutput> {
    // Debt to EBITDA
    let debt_to_ebitda = if input.ebitda == Decimal::ZERO {
        return Err(FinanceError::DivisionByZero("EBITDA".to_string()));
    } else {
        input.total_debt / input.ebitda
    };

    // Interest Coverage Ratio (EBIT / Interest Expense)
    let interest_coverage = if input.interest_expense == Decimal::ZERO {
        Decimal::MAX  // If no interest expense, coverage is effectively infinite
    } else {
        input.ebit / input.interest_expense
    };

    // Current Ratio (Current Assets / Current Liabilities)
    let current_ratio = if input.current_liabilities == Decimal::ZERO {
        return Err(FinanceError::DivisionByZero("current_liabilities".to_string()));
    } else {
        input.current_assets / input.current_liabilities
    };

    // Leverage Ratio (Total Debt / Total Assets)
    let leverage_ratio = if input.total_assets == Decimal::ZERO {
        return Err(FinanceError::DivisionByZero("total_assets".to_string()));
    } else {
        input.total_debt / input.total_assets
    };

    // Simple rating indication based on metrics
    let rating_indication = determine_rating(
        debt_to_ebitda,
        interest_coverage,
        current_ratio,
        leverage_ratio,
    );

    Ok(CreditMetricsOutput {
        debt_to_ebitda,
        interest_coverage,
        current_ratio,
        leverage_ratio,
        rating_indication,
    })
}

fn determine_rating(
    debt_to_ebitda: Decimal,
    interest_coverage: Decimal,
    current_ratio: Decimal,
    leverage_ratio: Decimal,
) -> String {
    // Simplified investment grade criteria
    let strong_metrics = debt_to_ebitda <= dec!(2.0)
        && interest_coverage >= dec!(5.0)
        && current_ratio >= dec!(1.5)
        && leverage_ratio <= dec!(0.4);

    let good_metrics = debt_to_ebitda <= dec!(3.5)
        && interest_coverage >= dec!(3.0)
        && current_ratio >= dec!(1.2)
        && leverage_ratio <= dec!(0.55);

    let acceptable_metrics = debt_to_ebitda <= dec!(5.0)
        && interest_coverage >= dec!(2.0)
        && current_ratio >= dec!(1.0)
        && leverage_ratio <= dec!(0.65);

    if strong_metrics {
        "Strong (A-/A)".to_string()
    } else if good_metrics {
        "Investment Grade (BBB)".to_string()
    } else if acceptable_metrics {
        "Below Investment Grade (BB)".to_string()
    } else {
        "Weak/High Risk (B or below)".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_credit_metrics() {
        let input = CreditMetricsInput {
            ebitda: dec!(100000),
            total_debt: dec!(250000),
            interest_expense: dec!(15000),
            ebit: dec!(80000),
            current_assets: dec!(150000),
            current_liabilities: dec!(100000),
            total_assets: dec!(500000),
        };

        let result = calculate_credit_metrics(input).unwrap();

        assert_eq!(result.debt_to_ebitda, dec!(2.5));
        assert_eq!(result.interest_coverage, dec!(5.3333333333333333333333333333));
        assert_eq!(result.current_ratio, dec!(1.5));
        assert_eq!(result.leverage_ratio, dec!(0.5));
    }
}
