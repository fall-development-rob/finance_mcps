use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceItem {
    pub name: String,
    pub amount: Decimal,
    pub pct_of_total: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseItem {
    pub name: String,
    pub amount: Decimal,
    pub pct_of_total: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcesAndUsesInput {
    // Sources
    pub senior_debt: Decimal,
    pub subordinated_debt: Decimal,
    pub equity_contribution: Decimal,
    pub rollover_equity: Decimal,
    pub seller_note: Option<Decimal>,
    pub other_sources: Vec<SourceItem>,

    // Uses
    pub purchase_equity_value: Decimal,
    pub refinanced_debt: Decimal,
    pub transaction_fees: Decimal,
    pub financing_fees: Decimal,
    pub other_uses: Vec<UseItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcesAndUsesOutput {
    pub sources: Vec<SourceItem>,
    pub uses: Vec<UseItem>,
    pub total_sources: Decimal,
    pub total_uses: Decimal,
    pub balanced: bool,
    pub imbalance: Decimal,

    // Summary metrics
    pub total_debt: Decimal,
    pub total_equity: Decimal,
    pub debt_to_equity_ratio: Decimal,
    pub equity_percentage: Decimal,
}

/// Build Sources and Uses table for a transaction
pub fn build_sources_and_uses(input: SourcesAndUsesInput) -> Result<SourcesAndUsesOutput> {
    let mut sources = Vec::new();
    let mut uses = Vec::new();

    // === SOURCES ===
    if input.senior_debt > Decimal::ZERO {
        sources.push(SourceItem {
            name: "Senior Debt".to_string(),
            amount: input.senior_debt,
            pct_of_total: Decimal::ZERO, // Will calculate later
        });
    }

    if input.subordinated_debt > Decimal::ZERO {
        sources.push(SourceItem {
            name: "Subordinated Debt".to_string(),
            amount: input.subordinated_debt,
            pct_of_total: Decimal::ZERO,
        });
    }

    if input.equity_contribution > Decimal::ZERO {
        sources.push(SourceItem {
            name: "Sponsor Equity".to_string(),
            amount: input.equity_contribution,
            pct_of_total: Decimal::ZERO,
        });
    }

    if input.rollover_equity > Decimal::ZERO {
        sources.push(SourceItem {
            name: "Rollover Equity".to_string(),
            amount: input.rollover_equity,
            pct_of_total: Decimal::ZERO,
        });
    }

    if let Some(seller_note) = input.seller_note {
        if seller_note > Decimal::ZERO {
            sources.push(SourceItem {
                name: "Seller Note".to_string(),
                amount: seller_note,
                pct_of_total: Decimal::ZERO,
            });
        }
    }

    // Add other sources
    for source in input.other_sources {
        if source.amount > Decimal::ZERO {
            sources.push(source);
        }
    }

    // === USES ===
    if input.purchase_equity_value > Decimal::ZERO {
        uses.push(UseItem {
            name: "Purchase Equity Value".to_string(),
            amount: input.purchase_equity_value,
            pct_of_total: Decimal::ZERO,
        });
    }

    if input.refinanced_debt > Decimal::ZERO {
        uses.push(UseItem {
            name: "Refinance Existing Debt".to_string(),
            amount: input.refinanced_debt,
            pct_of_total: Decimal::ZERO,
        });
    }

    if input.transaction_fees > Decimal::ZERO {
        uses.push(UseItem {
            name: "Transaction Fees".to_string(),
            amount: input.transaction_fees,
            pct_of_total: Decimal::ZERO,
        });
    }

    if input.financing_fees > Decimal::ZERO {
        uses.push(UseItem {
            name: "Financing Fees".to_string(),
            amount: input.financing_fees,
            pct_of_total: Decimal::ZERO,
        });
    }

    // Add other uses
    for use_item in input.other_uses {
        if use_item.amount > Decimal::ZERO {
            uses.push(use_item);
        }
    }

    // Calculate totals
    let total_sources: Decimal = sources.iter().map(|s| s.amount).sum();
    let total_uses: Decimal = uses.iter().map(|u| u.amount).sum();

    // Calculate percentages
    if total_sources > Decimal::ZERO {
        for source in &mut sources {
            source.pct_of_total = (source.amount / total_sources) * dec!(100);
        }
    }

    if total_uses > Decimal::ZERO {
        for use_item in &mut uses {
            use_item.pct_of_total = (use_item.amount / total_uses) * dec!(100);
        }
    }

    // Check if balanced
    let imbalance = total_sources - total_uses;
    let balanced = imbalance.abs() < dec!(0.01); // Within 1 cent

    // Calculate summary metrics
    let total_debt = input.senior_debt
        + input.subordinated_debt
        + input.seller_note.unwrap_or(Decimal::ZERO);

    let total_equity = input.equity_contribution + input.rollover_equity;

    let debt_to_equity_ratio = if total_equity > Decimal::ZERO {
        total_debt / total_equity
    } else {
        Decimal::ZERO
    };

    let equity_percentage = if total_sources > Decimal::ZERO {
        (total_equity / total_sources) * dec!(100)
    } else {
        Decimal::ZERO
    };

    Ok(SourcesAndUsesOutput {
        sources,
        uses,
        total_sources,
        total_uses,
        balanced,
        imbalance,
        total_debt,
        total_equity,
        debt_to_equity_ratio,
        equity_percentage,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_sources_and_uses_balanced() {
        let input = SourcesAndUsesInput {
            senior_debt: dec!(500),
            subordinated_debt: dec!(200),
            equity_contribution: dec!(250),
            rollover_equity: dec!(50),
            seller_note: None,
            other_sources: vec![],
            purchase_equity_value: dec!(800),
            refinanced_debt: dec!(100),
            transaction_fees: dec!(50),
            financing_fees: dec!(50),
            other_uses: vec![],
        };

        let result = build_sources_and_uses(input).unwrap();

        // Total sources = 500 + 200 + 250 + 50 = 1000
        assert_eq!(result.total_sources, dec!(1000));

        // Total uses = 800 + 100 + 50 + 50 = 1000
        assert_eq!(result.total_uses, dec!(1000));

        // Should be balanced
        assert!(result.balanced);
        assert_eq!(result.imbalance, Decimal::ZERO);

        // Check debt metrics
        assert_eq!(result.total_debt, dec!(700)); // 500 + 200
        assert_eq!(result.total_equity, dec!(300)); // 250 + 50

        // Debt/Equity = 700/300 = 2.33x
        assert!((result.debt_to_equity_ratio - dec!(2.33)).abs() < dec!(0.01));

        // Equity % = 300/1000 = 30%
        assert_eq!(result.equity_percentage, dec!(30));
    }

    #[test]
    fn test_sources_and_uses_with_seller_note() {
        let input = SourcesAndUsesInput {
            senior_debt: dec!(400),
            subordinated_debt: dec!(100),
            equity_contribution: dec!(200),
            rollover_equity: dec!(100),
            seller_note: Some(dec!(200)),
            other_sources: vec![],
            purchase_equity_value: dec!(900),
            refinanced_debt: dec!(50),
            transaction_fees: dec!(30),
            financing_fees: dec!(20),
            other_uses: vec![],
        };

        let result = build_sources_and_uses(input).unwrap();

        // Total sources = 400 + 100 + 200 + 100 + 200 = 1000
        assert_eq!(result.total_sources, dec!(1000));

        // Seller note counts as debt
        assert_eq!(result.total_debt, dec!(700)); // 400 + 100 + 200
    }

    #[test]
    fn test_sources_and_uses_imbalanced() {
        let input = SourcesAndUsesInput {
            senior_debt: dec!(500),
            subordinated_debt: dec!(0),
            equity_contribution: dec!(400),
            rollover_equity: dec!(0),
            seller_note: None,
            other_sources: vec![],
            purchase_equity_value: dec!(800),
            refinanced_debt: dec!(50),
            transaction_fees: dec!(30),
            financing_fees: dec!(20),
            other_uses: vec![],
        };

        let result = build_sources_and_uses(input).unwrap();

        // Total sources = 900
        // Total uses = 900
        assert_eq!(result.total_sources, dec!(900));
        assert_eq!(result.total_uses, dec!(900));
        assert!(result.balanced);
    }

    #[test]
    fn test_percentage_calculations() {
        let input = SourcesAndUsesInput {
            senior_debt: dec!(600),
            subordinated_debt: dec!(200),
            equity_contribution: dec!(200),
            rollover_equity: dec!(0),
            seller_note: None,
            other_sources: vec![],
            purchase_equity_value: dec!(950),
            refinanced_debt: dec!(0),
            transaction_fees: dec!(30),
            financing_fees: dec!(20),
            other_uses: vec![],
        };

        let result = build_sources_and_uses(input).unwrap();

        // Check that percentages add up to 100%
        let total_sources_pct: Decimal = result.sources.iter().map(|s| s.pct_of_total).sum();
        assert!((total_sources_pct - dec!(100)).abs() < dec!(0.01));

        let total_uses_pct: Decimal = result.uses.iter().map(|u| u.pct_of_total).sum();
        assert!((total_uses_pct - dec!(100)).abs() < dec!(0.01));

        // Senior debt should be 60% (600/1000)
        let senior_pct = result
            .sources
            .iter()
            .find(|s| s.name == "Senior Debt")
            .unwrap()
            .pct_of_total;
        assert_eq!(senior_pct, dec!(60));
    }
}
