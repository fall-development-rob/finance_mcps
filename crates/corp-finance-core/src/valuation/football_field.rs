use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::error::Result;
use super::types::{FootballFieldInput, FootballFieldOutput, ValuationMethodology};

/// Create football field valuation summary
/// Summarizes valuation ranges from multiple methodologies
pub fn create_football_field(input: FootballFieldInput) -> Result<FootballFieldOutput> {
    let mut methodologies = Vec::new();

    // DCF methodology
    let dcf_midpoint = (input.dcf_low + input.dcf_high) / dec!(2);
    let dcf_range_width = input.dcf_high - input.dcf_low;
    methodologies.push(ValuationMethodology {
        method: "DCF Analysis".to_string(),
        low: input.dcf_low,
        high: input.dcf_high,
        midpoint: dcf_midpoint,
        range_width: dcf_range_width,
    });

    // Comparable Companies
    let comps_midpoint = (input.comps_low + input.comps_high) / dec!(2);
    let comps_range_width = input.comps_high - input.comps_low;
    methodologies.push(ValuationMethodology {
        method: "Comparable Companies".to_string(),
        low: input.comps_low,
        high: input.comps_high,
        midpoint: comps_midpoint,
        range_width: comps_range_width,
    });

    // Precedent Transactions
    let precedents_midpoint = (input.precedents_low + input.precedents_high) / dec!(2);
    let precedents_range_width = input.precedents_high - input.precedents_low;
    methodologies.push(ValuationMethodology {
        method: "Precedent Transactions".to_string(),
        low: input.precedents_low,
        high: input.precedents_high,
        midpoint: precedents_midpoint,
        range_width: precedents_range_width,
    });

    // Calculate overall range
    let overall_low = input.dcf_low.min(input.comps_low).min(input.precedents_low);
    let overall_high = input.dcf_high.max(input.comps_high).max(input.precedents_high);
    let overall_midpoint = (overall_low + overall_high) / dec!(2);

    // Calculate implied upside/downside if current price provided
    let implied_upside_downside = if let Some(current_price) = input.current_price {
        if current_price > Decimal::ZERO {
            Some(((overall_midpoint - current_price) / current_price) * dec!(100))
        } else {
            None
        }
    } else {
        None
    };

    // Generate summary
    let summary = format!(
        "Valuation range: ${:.2} - ${:.2} (midpoint: ${:.2}). DCF: ${:.2}-${:.2}, Comps: ${:.2}-${:.2}, Precedents: ${:.2}-${:.2}",
        overall_low, overall_high, overall_midpoint,
        input.dcf_low, input.dcf_high,
        input.comps_low, input.comps_high,
        input.precedents_low, input.precedents_high
    );

    Ok(FootballFieldOutput {
        methodologies,
        overall_low,
        overall_high,
        overall_midpoint,
        current_price: input.current_price,
        implied_upside_downside,
        summary,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_football_field() {
        let input = FootballFieldInput {
            dcf_low: dec!(90),
            dcf_high: dec!(110),
            comps_low: dec!(85),
            comps_high: dec!(105),
            precedents_low: dec!(95),
            precedents_high: dec!(115),
            current_price: Some(dec!(80)),
        };

        let result = create_football_field(input).unwrap();

        assert_eq!(result.methodologies.len(), 3);
        assert_eq!(result.overall_low, dec!(85));
        assert_eq!(result.overall_high, dec!(115));
        assert_eq!(result.overall_midpoint, dec!(100));

        // DCF midpoint
        assert_eq!(result.methodologies[0].midpoint, dec!(100));

        // Implied upside from $80 to $100 midpoint = 25%
        assert_eq!(result.implied_upside_downside.unwrap(), dec!(25));
    }

    #[test]
    fn test_football_field_no_current_price() {
        let input = FootballFieldInput {
            dcf_low: dec!(100),
            dcf_high: dec!(120),
            comps_low: dec!(95),
            comps_high: dec!(115),
            precedents_low: dec!(105),
            precedents_high: dec!(125),
            current_price: None,
        };

        let result = create_football_field(input).unwrap();

        assert!(result.implied_upside_downside.is_none());
        assert_eq!(result.current_price, None);
    }

    #[test]
    fn test_football_field_range_widths() {
        let input = FootballFieldInput {
            dcf_low: dec!(80),
            dcf_high: dec!(120),
            comps_low: dec!(90),
            comps_high: dec!(100),
            precedents_low: dec!(95),
            precedents_high: dec!(105),
            current_price: None,
        };

        let result = create_football_field(input).unwrap();

        // DCF has widest range (40)
        assert_eq!(result.methodologies[0].range_width, dec!(40));

        // Comps has narrower range (10)
        assert_eq!(result.methodologies[1].range_width, dec!(10));

        // Precedents has narrowest range (10)
        assert_eq!(result.methodologies[2].range_width, dec!(10));
    }
}
