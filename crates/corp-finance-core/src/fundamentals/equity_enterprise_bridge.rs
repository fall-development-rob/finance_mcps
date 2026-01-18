use rust_decimal::Decimal;
use crate::error::Result;
use super::types::{EquityEnterpriseInput, EquityEnterpriseOutput, BridgeItem};

/// Calculate Enterprise Value from Equity Value or vice versa
///
/// Equity Value → Enterprise Value:
///   EV = Equity Value + Debt - Cash + Minority Interest + Preferred Stock - Associates
///
/// Enterprise Value → Equity Value:
///   Equity Value = EV - Debt + Cash - Minority Interest - Preferred Stock + Associates
pub fn equity_enterprise_bridge(input: EquityEnterpriseInput) -> Result<EquityEnterpriseOutput> {
    let net_debt = input.debt - input.cash;
    let mut bridge_items = Vec::new();

    let (equity_value, enterprise_value) = match input.direction.as_str() {
        "equity_to_ev" => {
            let equity_value = input.value;

            // Build bridge from equity to EV
            bridge_items.push(BridgeItem {
                item: "Equity Value".to_string(),
                amount: equity_value,
                direction: "start".to_string(),
            });

            bridge_items.push(BridgeItem {
                item: "Add: Debt".to_string(),
                amount: input.debt,
                direction: "add".to_string(),
            });

            bridge_items.push(BridgeItem {
                item: "Less: Cash".to_string(),
                amount: input.cash,
                direction: "subtract".to_string(),
            });

            bridge_items.push(BridgeItem {
                item: "Add: Minority Interest".to_string(),
                amount: input.minority_interest,
                direction: "add".to_string(),
            });

            bridge_items.push(BridgeItem {
                item: "Add: Preferred Stock".to_string(),
                amount: input.preferred_stock,
                direction: "add".to_string(),
            });

            bridge_items.push(BridgeItem {
                item: "Less: Associates/Investments".to_string(),
                amount: input.associates,
                direction: "subtract".to_string(),
            });

            let enterprise_value = equity_value
                + input.debt
                - input.cash
                + input.minority_interest
                + input.preferred_stock
                - input.associates;

            bridge_items.push(BridgeItem {
                item: "Enterprise Value".to_string(),
                amount: enterprise_value,
                direction: "end".to_string(),
            });

            (equity_value, enterprise_value)
        }
        "ev_to_equity" => {
            let enterprise_value = input.value;

            bridge_items.push(BridgeItem {
                item: "Enterprise Value".to_string(),
                amount: enterprise_value,
                direction: "start".to_string(),
            });

            bridge_items.push(BridgeItem {
                item: "Less: Debt".to_string(),
                amount: input.debt,
                direction: "subtract".to_string(),
            });

            bridge_items.push(BridgeItem {
                item: "Add: Cash".to_string(),
                amount: input.cash,
                direction: "add".to_string(),
            });

            bridge_items.push(BridgeItem {
                item: "Less: Minority Interest".to_string(),
                amount: input.minority_interest,
                direction: "subtract".to_string(),
            });

            bridge_items.push(BridgeItem {
                item: "Less: Preferred Stock".to_string(),
                amount: input.preferred_stock,
                direction: "subtract".to_string(),
            });

            bridge_items.push(BridgeItem {
                item: "Add: Associates/Investments".to_string(),
                amount: input.associates,
                direction: "add".to_string(),
            });

            let equity_value = enterprise_value
                - input.debt
                + input.cash
                - input.minority_interest
                - input.preferred_stock
                + input.associates;

            bridge_items.push(BridgeItem {
                item: "Equity Value".to_string(),
                amount: equity_value,
                direction: "end".to_string(),
            });

            (equity_value, enterprise_value)
        }
        _ => {
            return Err(crate::error::FinanceError::InvalidInput(
                "direction must be 'equity_to_ev' or 'ev_to_equity'".to_string()
            ));
        }
    };

    Ok(EquityEnterpriseOutput {
        equity_value,
        enterprise_value,
        net_debt,
        bridge_items,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_equity_to_ev() {
        let input = EquityEnterpriseInput {
            direction: "equity_to_ev".to_string(),
            value: dec!(1000),
            cash: dec!(100),
            debt: dec!(300),
            minority_interest: dec!(50),
            associates: dec!(25),
            preferred_stock: dec!(75),
        };

        let result = equity_enterprise_bridge(input).unwrap();

        // EV = 1000 + 300 - 100 + 50 + 75 - 25 = 1300
        assert_eq!(result.enterprise_value, dec!(1300));
        assert_eq!(result.equity_value, dec!(1000));
        assert_eq!(result.net_debt, dec!(200)); // 300 - 100
    }

    #[test]
    fn test_ev_to_equity() {
        let input = EquityEnterpriseInput {
            direction: "ev_to_equity".to_string(),
            value: dec!(1300),
            cash: dec!(100),
            debt: dec!(300),
            minority_interest: dec!(50),
            associates: dec!(25),
            preferred_stock: dec!(75),
        };

        let result = equity_enterprise_bridge(input).unwrap();

        // Equity = 1300 - 300 + 100 - 50 - 75 + 25 = 1000
        assert_eq!(result.equity_value, dec!(1000));
        assert_eq!(result.enterprise_value, dec!(1300));
        assert_eq!(result.net_debt, dec!(200));
    }

    #[test]
    fn test_roundtrip() {
        // Test that equity -> EV -> equity gives same result
        let equity_to_ev = EquityEnterpriseInput {
            direction: "equity_to_ev".to_string(),
            value: dec!(5000),
            cash: dec!(500),
            debt: dec!(2000),
            minority_interest: dec!(100),
            associates: dec!(200),
            preferred_stock: dec!(300),
        };

        let ev_result = equity_enterprise_bridge(equity_to_ev).unwrap();

        let ev_to_equity = EquityEnterpriseInput {
            direction: "ev_to_equity".to_string(),
            value: ev_result.enterprise_value,
            cash: dec!(500),
            debt: dec!(2000),
            minority_interest: dec!(100),
            associates: dec!(200),
            preferred_stock: dec!(300),
        };

        let equity_result = equity_enterprise_bridge(ev_to_equity).unwrap();

        assert_eq!(equity_result.equity_value, dec!(5000));
    }
}
