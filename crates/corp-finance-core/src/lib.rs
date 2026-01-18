pub mod wacc;
pub mod credit_metrics;
pub mod dcf;
pub mod debt_capacity;
pub mod covenant;
pub mod error;
pub mod types;

pub use error::FinanceError;
pub use types::*;

// Re-export main functions for easy access
pub use wacc::calculate_wacc;
pub use credit_metrics::calculate_credit_metrics;
pub use dcf::calculate_dcf;
pub use debt_capacity::calculate_debt_capacity;
pub use covenant::check_covenant_compliance;
