// Phase 1 modules
pub mod wacc;
pub mod credit_metrics;
pub mod dcf;
pub mod debt_capacity;
pub mod covenant;
pub mod error;
pub mod types;

// Phase 2 modules
pub mod fundamentals;
pub mod valuation;

pub use error::FinanceError;
pub use types::*;

// Re-export Phase 1 functions
pub use wacc::calculate_wacc;
pub use credit_metrics::calculate_credit_metrics;
pub use dcf::calculate_dcf;
pub use debt_capacity::calculate_debt_capacity;
pub use covenant::check_covenant_compliance;

// Re-export Phase 2 functions
pub use fundamentals::{
    build_three_statement_model,
    equity_enterprise_bridge,
    calculate_diluted_shares,
    analyze_accounting_flow,
};
pub use valuation::{
    create_football_field,
    calculate_paper_lbo,
};
