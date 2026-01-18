use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueBridgeInput {
    // Entry metrics
    pub entry_ebitda: Decimal,
    pub entry_multiple: Decimal,
    pub entry_net_debt: Decimal,

    // Exit metrics
    pub exit_ebitda: Decimal,
    pub exit_multiple: Decimal,
    pub exit_net_debt: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueBridgeOutput {
    pub entry_equity: Decimal,
    pub exit_equity: Decimal,
    pub total_return: Decimal,
    pub moic: Decimal,

    // Attribution breakdown
    pub ebitda_growth_value: Decimal,
    pub ebitda_growth_pct: Decimal,

    pub multiple_expansion_value: Decimal,
    pub multiple_expansion_pct: Decimal,

    pub deleveraging_value: Decimal,
    pub deleveraging_pct: Decimal,

    pub other_value: Decimal,
    pub other_pct: Decimal,

    // Bridge steps for visualization
    pub bridge_steps: Vec<BridgeStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeStep {
    pub description: String,
    pub value: Decimal,
    pub cumulative: Decimal,
}

/// Calculate returns attribution bridge
/// Decomposes equity returns into: EBITDA growth, multiple expansion, and deleveraging
pub fn calculate_value_bridge(input: ValueBridgeInput) -> Result<ValueBridgeOutput> {
    // Calculate entry and exit equity values
    let entry_ev = input.entry_ebitda * input.entry_multiple;
    let entry_equity = entry_ev - input.entry_net_debt;

    let exit_ev = input.exit_ebitda * input.exit_multiple;
    let exit_equity = exit_ev - input.exit_net_debt;

    let total_return = exit_equity - entry_equity;
    let moic = if entry_equity > Decimal::ZERO {
        exit_equity / entry_equity
    } else {
        Decimal::ZERO
    };

    // === VALUE BRIDGE CALCULATION ===
    // We'll walk through the bridge step by step

    let mut bridge_steps = Vec::new();
    let mut cumulative = entry_equity;

    // Step 1: Starting point
    bridge_steps.push(BridgeStep {
        description: "Entry Equity Value".to_string(),
        value: entry_equity,
        cumulative,
    });

    // Step 2: EBITDA Growth (holding multiple and leverage constant)
    // Impact = (Exit EBITDA - Entry EBITDA) × Entry Multiple
    let ebitda_growth_value = (input.exit_ebitda - input.entry_ebitda) * input.entry_multiple;
    cumulative += ebitda_growth_value;

    bridge_steps.push(BridgeStep {
        description: "EBITDA Growth".to_string(),
        value: ebitda_growth_value,
        cumulative,
    });

    // Step 3: Multiple Expansion (using exit EBITDA)
    // Impact = Exit EBITDA × (Exit Multiple - Entry Multiple)
    let multiple_expansion_value = input.exit_ebitda * (input.exit_multiple - input.entry_multiple);
    cumulative += multiple_expansion_value;

    bridge_steps.push(BridgeStep {
        description: "Multiple Expansion".to_string(),
        value: multiple_expansion_value,
        cumulative,
    });

    // Step 4: Deleveraging
    // Impact = Entry Net Debt - Exit Net Debt
    let deleveraging_value = input.entry_net_debt - input.exit_net_debt;
    cumulative += deleveraging_value;

    bridge_steps.push(BridgeStep {
        description: "Deleveraging".to_string(),
        value: deleveraging_value,
        cumulative,
    });

    // Step 5: Other/Residual (should be close to zero if our math is right)
    let other_value = exit_equity - cumulative;
    cumulative += other_value;

    if other_value.abs() > dec!(0.01) {
        bridge_steps.push(BridgeStep {
            description: "Other/Residual".to_string(),
            value: other_value,
            cumulative,
        });
    }

    // Final step: Exit equity
    bridge_steps.push(BridgeStep {
        description: "Exit Equity Value".to_string(),
        value: exit_equity,
        cumulative: exit_equity,
    });

    // Calculate percentages of total return
    let ebitda_growth_pct = if total_return != Decimal::ZERO {
        (ebitda_growth_value / total_return) * dec!(100)
    } else {
        Decimal::ZERO
    };

    let multiple_expansion_pct = if total_return != Decimal::ZERO {
        (multiple_expansion_value / total_return) * dec!(100)
    } else {
        Decimal::ZERO
    };

    let deleveraging_pct = if total_return != Decimal::ZERO {
        (deleveraging_value / total_return) * dec!(100)
    } else {
        Decimal::ZERO
    };

    let other_pct = if total_return != Decimal::ZERO {
        (other_value / total_return) * dec!(100)
    } else {
        Decimal::ZERO
    };

    Ok(ValueBridgeOutput {
        entry_equity,
        exit_equity,
        total_return,
        moic,
        ebitda_growth_value,
        ebitda_growth_pct,
        multiple_expansion_value,
        multiple_expansion_pct,
        deleveraging_value,
        deleveraging_pct,
        other_value,
        other_pct,
        bridge_steps,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_value_bridge_ebitda_growth_only() {
        let input = ValueBridgeInput {
            entry_ebitda: dec!(100),
            entry_multiple: dec!(10),
            entry_net_debt: dec!(500),
            exit_ebitda: dec!(150),
            exit_multiple: dec!(10), // No multiple expansion
            exit_net_debt: dec!(500), // No deleveraging
        };

        let result = calculate_value_bridge(input).unwrap();

        // Entry EV = 100 * 10 = 1000
        // Entry Equity = 1000 - 500 = 500
        assert_eq!(result.entry_equity, dec!(500));

        // Exit EV = 150 * 10 = 1500
        // Exit Equity = 1500 - 500 = 1000
        assert_eq!(result.exit_equity, dec!(1000));

        // Total return = 1000 - 500 = 500
        assert_eq!(result.total_return, dec!(500));

        // MOIC = 1000 / 500 = 2.0x
        assert_eq!(result.moic, dec!(2.0));

        // EBITDA growth = (150 - 100) * 10 = 500
        assert_eq!(result.ebitda_growth_value, dec!(500));
        assert_eq!(result.ebitda_growth_pct, dec!(100)); // 100% of return

        // No multiple expansion
        assert_eq!(result.multiple_expansion_value, Decimal::ZERO);

        // No deleveraging
        assert_eq!(result.deleveraging_value, Decimal::ZERO);
    }

    #[test]
    fn test_value_bridge_multiple_expansion() {
        let input = ValueBridgeInput {
            entry_ebitda: dec!(100),
            entry_multiple: dec!(8),
            entry_net_debt: dec!(400),
            exit_ebitda: dec!(100), // No EBITDA growth
            exit_multiple: dec!(10), // Multiple expansion
            exit_net_debt: dec!(400), // No deleveraging
        };

        let result = calculate_value_bridge(input).unwrap();

        // Entry Equity = 800 - 400 = 400
        assert_eq!(result.entry_equity, dec!(400));

        // Exit Equity = 1000 - 400 = 600
        assert_eq!(result.exit_equity, dec!(600));

        // Multiple expansion = 100 * (10 - 8) = 200
        assert_eq!(result.multiple_expansion_value, dec!(200));
        assert_eq!(result.multiple_expansion_pct, dec!(100)); // 100% of return

        // No EBITDA growth
        assert_eq!(result.ebitda_growth_value, Decimal::ZERO);
    }

    #[test]
    fn test_value_bridge_deleveraging() {
        let input = ValueBridgeInput {
            entry_ebitda: dec!(100),
            entry_multiple: dec!(10),
            entry_net_debt: dec!(600),
            exit_ebitda: dec!(100), // No growth
            exit_multiple: dec!(10), // No multiple expansion
            exit_net_debt: dec!(300), // Paid down 300 of debt
        };

        let result = calculate_value_bridge(input).unwrap();

        // Entry Equity = 1000 - 600 = 400
        assert_eq!(result.entry_equity, dec!(400));

        // Exit Equity = 1000 - 300 = 700
        assert_eq!(result.exit_equity, dec!(700));

        // Deleveraging = 600 - 300 = 300
        assert_eq!(result.deleveraging_value, dec!(300));
        assert_eq!(result.deleveraging_pct, dec!(100)); // 100% of return
    }

    #[test]
    fn test_value_bridge_mixed() {
        // Realistic LBO with all three factors
        let input = ValueBridgeInput {
            entry_ebitda: dec!(100),
            entry_multiple: dec!(10),
            entry_net_debt: dec!(500),
            exit_ebitda: dec!(130), // 30% EBITDA growth
            exit_multiple: dec!(11), // 1 turn multiple expansion
            exit_net_debt: dec!(300), // 200 deleveraging
        };

        let result = calculate_value_bridge(input).unwrap();

        // Entry Equity = 1000 - 500 = 500
        assert_eq!(result.entry_equity, dec!(500));

        // Exit EV = 130 * 11 = 1430
        // Exit Equity = 1430 - 300 = 1130
        assert_eq!(result.exit_equity, dec!(1130));

        // Total return = 1130 - 500 = 630
        assert_eq!(result.total_return, dec!(630));

        // MOIC = 1130 / 500 = 2.26x
        assert!((result.moic - dec!(2.26)).abs() < dec!(0.01));

        // EBITDA growth = (130 - 100) * 10 = 300
        assert_eq!(result.ebitda_growth_value, dec!(300));

        // Multiple expansion = 130 * (11 - 10) = 130
        assert_eq!(result.multiple_expansion_value, dec!(130));

        // Deleveraging = 500 - 300 = 200
        assert_eq!(result.deleveraging_value, dec!(200));

        // Total should match: 300 + 130 + 200 = 630
        let total_attributed =
            result.ebitda_growth_value + result.multiple_expansion_value + result.deleveraging_value;
        assert_eq!(total_attributed, result.total_return);

        // Check percentages add up to 100%
        let total_pct = result.ebitda_growth_pct
            + result.multiple_expansion_pct
            + result.deleveraging_pct
            + result.other_pct;
        assert!((total_pct - dec!(100)).abs() < dec!(0.1));
    }
}
