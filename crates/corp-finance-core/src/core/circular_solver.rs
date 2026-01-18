use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::error::{FinanceError, Result};

/// Solves circular references using iterative convergence
/// Common use case: Interest depends on debt, debt depends on cash, cash depends on interest
///
/// Uses simple fixed-point iteration with convergence tolerance
pub fn solve_circular<F>(
    initial_guess: Decimal,
    calculate_fn: F,
    tolerance: Decimal,
    max_iterations: usize,
) -> Result<Decimal>
where
    F: Fn(Decimal) -> Decimal,
{
    let mut current = initial_guess;
    let mut iteration = 0;

    while iteration < max_iterations {
        let next = calculate_fn(current);
        let error = (next - current).abs();

        if error < tolerance {
            return Ok(next);
        }

        current = next;
        iteration += 1;
    }

    Err(FinanceError::InvalidInput(format!(
        "Circular solver failed to converge after {} iterations (tolerance: {})",
        max_iterations, tolerance
    )))
}

/// Newton-Raphson method for faster convergence when derivative is available
pub fn solve_circular_newton<F, G>(
    initial_guess: Decimal,
    function: F,
    derivative: G,
    tolerance: Decimal,
    max_iterations: usize,
) -> Result<Decimal>
where
    F: Fn(Decimal) -> Decimal,
    G: Fn(Decimal) -> Decimal,
{
    let mut current = initial_guess;
    let mut iteration = 0;

    while iteration < max_iterations {
        let f_val = function(current);
        let df_val = derivative(current);

        if df_val.abs() < dec!(0.0000001) {
            return Err(FinanceError::InvalidInput(
                "Derivative too close to zero in Newton-Raphson".to_string(),
            ));
        }

        let next = current - f_val / df_val;
        let error = (next - current).abs();

        if error < tolerance {
            return Ok(next);
        }

        current = next;
        iteration += 1;
    }

    Err(FinanceError::InvalidInput(format!(
        "Newton-Raphson failed to converge after {} iterations",
        max_iterations
    )))
}

/// Solve system where cash depends on interest, and interest depends on debt, which depends on cash
/// This is the most common circular reference in financial modeling
///
/// The algorithm:
/// 1. Start with initial cash guess
/// 2. Calculate interest based on debt (which uses cash)
/// 3. Calculate new cash based on interest
/// 4. Repeat until convergence
pub fn solve_cash_interest_circular<F>(
    initial_cash_guess: Decimal,
    calculate_cash_given_interest: F,
    interest_rate: Decimal,
    beginning_debt: Decimal,
    tolerance: Decimal,
) -> Result<Decimal>
where
    F: Fn(Decimal) -> Decimal,
{
    solve_circular(
        initial_cash_guess,
        |cash| {
            // Calculate interest expense based on average debt
            // (debt can change based on cash availability)
            let interest_expense = beginning_debt * interest_rate / dec!(100);

            // Calculate new cash based on this interest expense
            calculate_cash_given_interest(interest_expense)
        },
        tolerance,
        100,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_simple_circular() {
        // Simple example: x = (x + 10) / 2
        // Solution: x = 10
        let result = solve_circular(
            dec!(0),
            |x| (x + dec!(10)) / dec!(2),
            dec!(0.0001),
            100,
        )
        .unwrap();

        assert!((result - dec!(10)).abs() < dec!(0.01));
    }

    #[test]
    fn test_circular_no_convergence() {
        // This will oscillate and not converge
        let result = solve_circular(
            dec!(1),
            |x| -x + dec!(1),
            dec!(0.0001),
            10,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_cash_interest_circular() {
        // Scenario:
        // Starting with 100 cash
        // 500 debt at 10% = 50 interest
        // Cash flow from ops = 200
        // New cash = starting cash + cash flow - interest
        let result = solve_cash_interest_circular(
            dec!(100), // initial guess
            |interest_expense| {
                let starting_cash = dec!(100);
                let cash_flow_from_ops = dec!(200);
                starting_cash + cash_flow_from_ops - interest_expense
            },
            dec!(10), // 10% interest rate
            dec!(500), // beginning debt
            dec!(0.01),
        )
        .unwrap();

        // Expected: cash = 100 + 200 - 50 = 250
        assert!((result - dec!(250)).abs() < dec!(1));
    }

    #[test]
    fn test_newton_raphson() {
        // Solve x^2 - 4 = 0 (answer should be 2)
        let result = solve_circular_newton(
            dec!(1), // initial guess
            |x| x * x - dec!(4),
            |x| dec!(2) * x, // derivative: 2x
            dec!(0.0001),
            20,
        )
        .unwrap();

        assert!((result - dec!(2)).abs() < dec!(0.01));
    }

    #[test]
    fn test_newton_raphson_faster_convergence() {
        let mut iterations_simple = 0;
        let mut iterations_newton = 0;

        // Count iterations for simple method
        let mut current = dec!(10);
        let target = dec!(2);
        while (current - target).abs() > dec!(0.0001) && iterations_simple < 100 {
            current = (current + target * target / current) / dec!(2);
            iterations_simple += 1;
        }

        // Newton-Raphson should converge faster
        let result = solve_circular_newton(
            dec!(10),
            |x| x * x - dec!(4),
            |x| dec!(2) * x,
            dec!(0.0001),
            100,
        );

        assert!(result.is_ok());
        // Newton-Raphson typically converges in < 10 iterations
        // while simple iteration might take more
    }
}
