use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::error::Result;
use super::types::{DilutedSharesInput, DilutedSharesOutput, DilutionItem};

/// Calculate fully diluted shares using treasury stock method
///
/// Treasury Stock Method (for options):
/// - If stock price > strike price, option is in-the-money
/// - Dilution = Shares from exercise - Shares that can be repurchased
/// - Shares that can be repurchased = (Strike * Quantity) / Stock Price
///
/// RSUs: Add full count (already granted, no strike price)
///
/// Convertibles: Add shares from conversion if economical
pub fn calculate_diluted_shares(input: DilutedSharesInput) -> Result<DilutedSharesOutput> {
    let basic_shares = input.basic_shares;
    let mut breakdown = Vec::new();

    // Basic shares
    breakdown.push(DilutionItem {
        source: "Basic Shares".to_string(),
        shares: basic_shares,
        method: "Base count".to_string(),
    });

    // Calculate options dilution using treasury stock method
    let mut options_dilution = Decimal::ZERO;
    for option in &input.options {
        if input.stock_price > option.strike_price {
            // In-the-money options
            let proceeds = option.quantity * option.strike_price;
            let shares_repurchased = proceeds / input.stock_price;
            let net_dilution = option.quantity - shares_repurchased;

            options_dilution += net_dilution;

            breakdown.push(DilutionItem {
                source: format!(
                    "Options @ ${} ({} shares)",
                    option.strike_price, option.quantity
                ),
                shares: net_dilution,
                method: "Treasury Stock Method".to_string(),
            });
        }
    }

    // RSUs - fully dilutive (no strike price)
    let rsu_dilution = input.rsus;
    if rsu_dilution > Decimal::ZERO {
        breakdown.push(DilutionItem {
            source: "RSUs".to_string(),
            shares: rsu_dilution,
            method: "Full count (no strike)".to_string(),
        });
    }

    // Convertibles - if conversion is economical
    let mut convertibles_dilution = Decimal::ZERO;
    for convertible in &input.convertibles {
        let shares_on_conversion = convertible.principal / convertible.conversion_price;

        // Check if conversion is economical
        if input.stock_price > convertible.conversion_price {
            convertibles_dilution += shares_on_conversion;

            breakdown.push(DilutionItem {
                source: format!(
                    "Convertible ${} @ ${}",
                    convertible.principal, convertible.conversion_price
                ),
                shares: shares_on_conversion,
                method: "If-converted method".to_string(),
            });
        }
    }

    // Calculate totals
    let fully_diluted_shares =
        basic_shares + options_dilution + rsu_dilution + convertibles_dilution;

    let dilution_percentage = if basic_shares > Decimal::ZERO {
        ((fully_diluted_shares - basic_shares) / basic_shares) * dec!(100)
    } else {
        Decimal::ZERO
    };

    Ok(DilutedSharesOutput {
        basic_shares,
        options_dilution,
        rsu_dilution,
        convertibles_dilution,
        fully_diluted_shares,
        dilution_percentage,
        breakdown,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use super::super::types::{OptionGrant, Convertible};

    #[test]
    fn test_treasury_stock_method() {
        let input = DilutedSharesInput {
            basic_shares: dec!(100),
            stock_price: dec!(50),
            options: vec![
                OptionGrant {
                    quantity: dec!(10),
                    strike_price: dec!(30),
                },
            ],
            rsus: dec!(0),
            convertibles: vec![],
        };

        let result = calculate_diluted_shares(input).unwrap();

        // Options dilution:
        // Proceeds = 10 * 30 = 300
        // Shares repurchased = 300 / 50 = 6
        // Net dilution = 10 - 6 = 4
        assert_eq!(result.options_dilution, dec!(4));
        assert_eq!(result.fully_diluted_shares, dec!(104));
    }

    #[test]
    fn test_out_of_money_options() {
        let input = DilutedSharesInput {
            basic_shares: dec!(100),
            stock_price: dec!(20),
            options: vec![
                OptionGrant {
                    quantity: dec!(10),
                    strike_price: dec!(30),
                },
            ],
            rsus: dec!(0),
            convertibles: vec![],
        };

        let result = calculate_diluted_shares(input).unwrap();

        // Options are out of money, no dilution
        assert_eq!(result.options_dilution, dec!(0));
        assert_eq!(result.fully_diluted_shares, dec!(100));
    }

    #[test]
    fn test_rsus() {
        let input = DilutedSharesInput {
            basic_shares: dec!(100),
            stock_price: dec!(50),
            options: vec![],
            rsus: dec!(5),
            convertibles: vec![],
        };

        let result = calculate_diluted_shares(input).unwrap();

        assert_eq!(result.rsu_dilution, dec!(5));
        assert_eq!(result.fully_diluted_shares, dec!(105));
        assert_eq!(result.dilution_percentage, dec!(5)); // 5/100 = 5%
    }

    #[test]
    fn test_convertibles() {
        let input = DilutedSharesInput {
            basic_shares: dec!(100),
            stock_price: dec!(50),
            options: vec![],
            rsus: dec!(0),
            convertibles: vec![
                Convertible {
                    principal: dec!(1000),
                    conversion_price: dec!(40),
                },
            ],
        };

        let result = calculate_diluted_shares(input).unwrap();

        // Shares on conversion = 1000 / 40 = 25
        assert_eq!(result.convertibles_dilution, dec!(25));
        assert_eq!(result.fully_diluted_shares, dec!(125));
    }

    #[test]
    fn test_full_dilution_mix() {
        let input = DilutedSharesInput {
            basic_shares: dec!(1000),
            stock_price: dec!(100),
            options: vec![
                OptionGrant {
                    quantity: dec!(50),
                    strike_price: dec!(60),
                },
                OptionGrant {
                    quantity: dec!(30),
                    strike_price: dec!(80),
                },
            ],
            rsus: dec!(20),
            convertibles: vec![
                Convertible {
                    principal: dec!(5000),
                    conversion_price: dec!(90),
                },
            ],
        };

        let result = calculate_diluted_shares(input).unwrap();

        // Option 1: (50 * 60 / 100) = 30 repurchased, 20 dilution
        // Option 2: (30 * 80 / 100) = 24 repurchased, 6 dilution
        // Total options dilution = 26
        assert_eq!(result.options_dilution, dec!(26));

        // RSUs = 20
        assert_eq!(result.rsu_dilution, dec!(20));

        // Convertibles: 5000 / 90 = 55.555... shares
        // Should be around 55.55
        assert!(result.convertibles_dilution > dec!(55));
        assert!(result.convertibles_dilution < dec!(56));

        // Total should be 1000 + 26 + 20 + 55.55 ≈ 1101.55
        assert!(result.fully_diluted_shares > dec!(1101));
        assert!(result.fully_diluted_shares < dec!(1102));
    }
}
