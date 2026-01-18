use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::NaiveDate;
use crate::error::{FinanceError, Result};
use super::circular_solver::solve_circular_newton;

/// Calculate Net Present Value
pub fn calculate_npv(cash_flows: &[Decimal], discount_rate: Decimal) -> Decimal {
    let rate_decimal = discount_rate / dec!(100);

    cash_flows
        .iter()
        .enumerate()
        .map(|(period, &cf)| {
            let mut discount_factor = Decimal::ONE;
            for _ in 0..period {
                discount_factor /= Decimal::ONE + rate_decimal;
            }
            cf * discount_factor
        })
        .sum()
}

/// Calculate Internal Rate of Return using Newton-Raphson
/// Returns IRR as a percentage (e.g., 15.5 for 15.5%)
pub fn calculate_irr(cash_flows: &[Decimal], initial_guess: Decimal) -> Result<Decimal> {
    if cash_flows.is_empty() {
        return Err(FinanceError::InvalidInput(
            "Cash flows cannot be empty".to_string(),
        ));
    }

    // NPV function
    let npv_fn = |rate: Decimal| -> Decimal {
        let rate_decimal = rate / dec!(100);
        cash_flows
            .iter()
            .enumerate()
            .map(|(period, &cf)| {
                let mut discount_factor = Decimal::ONE;
                for _ in 0..period {
                    discount_factor /= Decimal::ONE + rate_decimal;
                }
                cf * discount_factor
            })
            .sum()
    };

    // Derivative of NPV with respect to rate
    let npv_derivative = |rate: Decimal| -> Decimal {
        let rate_decimal = rate / dec!(100);
        cash_flows
            .iter()
            .enumerate()
            .skip(1) // Skip period 0
            .map(|(period, &cf)| {
                let period_dec = Decimal::from(period);
                let mut discount_factor = Decimal::ONE;
                for _ in 0..period {
                    discount_factor /= Decimal::ONE + rate_decimal;
                }
                -period_dec * cf * discount_factor / (Decimal::ONE + rate_decimal) / dec!(100)
            })
            .sum()
    };

    let irr = solve_circular_newton(
        initial_guess,
        npv_fn,
        npv_derivative,
        dec!(0.0001),
        100,
    )?;

    Ok(irr)
}

/// Calculate XIRR (IRR with irregular time periods)
/// dates[0] should be the initial investment date
/// cash_flows[0] should be the initial investment (typically negative)
pub fn calculate_xirr(
    cash_flows: &[Decimal],
    dates: &[NaiveDate],
    initial_guess: Decimal,
) -> Result<Decimal> {
    if cash_flows.len() != dates.len() {
        return Err(FinanceError::InvalidInput(
            "Cash flows and dates must have same length".to_string(),
        ));
    }

    if dates.is_empty() {
        return Err(FinanceError::InvalidInput(
            "Dates cannot be empty".to_string(),
        ));
    }

    let base_date = dates[0];

    // XNPV function
    let xnpv_fn = |rate: Decimal| -> Decimal {
        let rate_decimal = rate / dec!(100);
        cash_flows
            .iter()
            .zip(dates.iter())
            .map(|(&cf, &date)| {
                let days = (date - base_date).num_days();
                let years = Decimal::from(days) / dec!(365.25);

                // Discount factor: 1 / (1 + r)^years
                let mut discount_factor = Decimal::ONE;
                let one_plus_r = Decimal::ONE + rate_decimal;

                // Approximate (1+r)^years using iteration
                // For small years, this is accurate enough
                if years > Decimal::ZERO {
                    let years_int = years.floor().to_string().parse::<i64>().unwrap_or(0);
                    for _ in 0..years_int {
                        discount_factor /= one_plus_r;
                    }
                    // Handle fractional year (simplified)
                    let frac = years - Decimal::from(years_int);
                    if frac > Decimal::ZERO {
                        discount_factor /= Decimal::ONE + rate_decimal * frac;
                    }
                }

                cf * discount_factor
            })
            .sum()
    };

    // Derivative (numerical approximation)
    let xnpv_derivative = |rate: Decimal| -> Decimal {
        let delta = dec!(0.01);
        (xnpv_fn(rate + delta) - xnpv_fn(rate - delta)) / (dec!(2) * delta)
    };

    let xirr = solve_circular_newton(
        initial_guess,
        xnpv_fn,
        xnpv_derivative,
        dec!(0.0001),
        100,
    )?;

    Ok(xirr)
}

/// Calculate Multiple on Invested Capital (MOIC)
/// MOIC = Total Value / Total Invested
pub fn calculate_moic(invested: Decimal, returned: Decimal) -> Result<Decimal> {
    if invested == Decimal::ZERO {
        return Err(FinanceError::InvalidInput(
            "Invested capital cannot be zero".to_string(),
        ));
    }

    if invested < Decimal::ZERO {
        return Ok(returned / invested.abs());
    }

    Ok(returned / invested)
}

/// Convert MOIC and holding period to approximate IRR
/// Uses rule of 72 approximation for quick estimates
pub fn moic_to_irr_approx(moic: Decimal, years: Decimal) -> Result<Decimal> {
    if years <= Decimal::ZERO {
        return Err(FinanceError::InvalidInput(
            "Years must be positive".to_string(),
        ));
    }

    if moic <= Decimal::ZERO {
        return Err(FinanceError::InvalidInput(
            "MOIC must be positive".to_string(),
        ));
    }

    // Approximation: IRR ≈ (MOIC - 1) / years * 100
    // More accurate: IRR = (MOIC^(1/years) - 1) * 100
    // We'll use the simple approximation
    let irr_approx = ((moic - Decimal::ONE) / years) * dec!(100);

    Ok(irr_approx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_npv() {
        let cash_flows = vec![dec!(-100), dec!(50), dec!(50), dec!(50)];
        let npv = calculate_npv(&cash_flows, dec!(10));

        // NPV at 10% should be approximately 24.34
        assert!((npv - dec!(24.34)).abs() < dec!(1));
    }

    #[test]
    fn test_irr_simple() {
        // Invest 100, get 110 after 1 year = 10% IRR
        let cash_flows = vec![dec!(-100), dec!(110)];
        let irr = calculate_irr(&cash_flows, dec!(10)).unwrap();

        assert!((irr - dec!(10)).abs() < dec!(0.1));
    }

    #[test]
    fn test_irr_complex() {
        // Multi-period cash flows
        let cash_flows = vec![dec!(-1000), dec!(300), dec!(300), dec!(300), dec!(500)];
        let irr = calculate_irr(&cash_flows, dec!(15)).unwrap();

        // Should be around 15-20%
        assert!(irr > dec!(10) && irr < dec!(25));

        // Verify NPV at this IRR is close to zero
        let npv_at_irr = calculate_npv(&cash_flows, irr);
        assert!(npv_at_irr.abs() < dec!(1));
    }

    #[test]
    fn test_moic() {
        let moic = calculate_moic(dec!(100), dec!(250)).unwrap();
        assert_eq!(moic, dec!(2.5));
    }

    #[test]
    fn test_moic_to_irr_approx() {
        // 2.5x over 5 years
        let irr_approx = moic_to_irr_approx(dec!(2.5), dec!(5)).unwrap();

        // Approximation: (2.5 - 1) / 5 * 100 = 30%
        assert!((irr_approx - dec!(30)).abs() < dec!(1));
    }

    #[test]
    fn test_xirr() {
        use chrono::NaiveDate;

        let dates = vec![
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 7, 1).unwrap(),
            NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        ];

        let cash_flows = vec![dec!(-1000), dec!(200), dec!(300), dec!(800)];

        let xirr = calculate_xirr(&cash_flows, &dates, dec!(15));

        assert!(xirr.is_ok());
        let xirr_val = xirr.unwrap();

        // Should be positive return
        assert!(xirr_val > dec!(0));
        assert!(xirr_val < dec!(50));
    }
}
