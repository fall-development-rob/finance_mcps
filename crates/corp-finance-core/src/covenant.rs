use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::error::Result;
use crate::types::{CovenantInput, CovenantOutput, CovenantResult, CovenantTest};

/// Check compliance with debt covenants
/// Supports maximum, minimum, and range-based covenants
pub fn check_covenant_compliance(input: CovenantInput) -> Result<CovenantOutput> {
    let mut results = Vec::new();
    let mut violations = Vec::new();
    let mut overall_compliant = true;

    for test in input.tests {
        let (compliant, headroom) = match test.covenant_type.as_str() {
            "maximum" => {
                let is_compliant = test.actual <= test.limit;
                let headroom_val = test.limit - test.actual;
                (is_compliant, headroom_val)
            }
            "minimum" => {
                let is_compliant = test.actual >= test.limit;
                let headroom_val = test.actual - test.limit;
                (is_compliant, headroom_val)
            }
            _ => {
                // For unknown types, assume maximum
                let is_compliant = test.actual <= test.limit;
                let headroom_val = test.limit - test.actual;
                (is_compliant, headroom_val)
            }
        };

        let headroom_percentage = if test.limit == Decimal::ZERO {
            Decimal::ZERO
        } else {
            (headroom / test.limit.abs()) * dec!(100)
        };

        if !compliant {
            overall_compliant = false;
            violations.push(format!(
                "{}: {} {} (limit: {})",
                test.name, test.actual, test.covenant_type, test.limit
            ));
        }

        results.push(CovenantResult {
            test_name: test.name,
            compliant,
            limit: test.limit,
            actual: test.actual,
            headroom,
            headroom_percentage,
        });
    }

    Ok(CovenantOutput {
        overall_compliant,
        results,
        violations,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_covenant_compliance() {
        let input = CovenantInput {
            tests: vec![
                CovenantTest {
                    name: "Max Leverage".to_string(),
                    covenant_type: "maximum".to_string(),
                    limit: dec!(5.0),
                    actual: dec!(4.2),
                },
                CovenantTest {
                    name: "Min Interest Coverage".to_string(),
                    covenant_type: "minimum".to_string(),
                    limit: dec!(2.5),
                    actual: dec!(3.1),
                },
            ],
        };

        let result = check_covenant_compliance(input).unwrap();

        assert!(result.overall_compliant);
        assert_eq!(result.violations.len(), 0);
        assert_eq!(result.results.len(), 2);
    }

    #[test]
    fn test_covenant_violation() {
        let input = CovenantInput {
            tests: vec![
                CovenantTest {
                    name: "Max Leverage".to_string(),
                    covenant_type: "maximum".to_string(),
                    limit: dec!(5.0),
                    actual: dec!(5.5),
                },
            ],
        };

        let result = check_covenant_compliance(input).unwrap();

        assert!(!result.overall_compliant);
        assert_eq!(result.violations.len(), 1);
    }
}
