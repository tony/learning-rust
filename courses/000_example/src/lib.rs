//! # Course 000: Example - Template and Best Practices
//!
//! This course demonstrates the patterns and practices used throughout
//! all Learning Rust courses. It serves as both a template for course
//! creators and a learning resource for students.
//!
//! ## Course Structure
//!
//! - Each lesson is a separate module
//! - Lessons progress from simple to complex
//! - Every concept includes multiple implementations
//! - Comprehensive testing at multiple levels
//!
//! ## Learning Path
//!
//! 1. Start with `intro` for course overview
//! 2. Study `linear_search` for algorithm patterns
//! 3. Explore `error_handling` for Result/Option usage
//! 4. Learn `generics_and_traits` for abstraction
//! 5. Master `performance_optimization` for benchmarking

#![warn(missing_docs)]
#![warn(clippy::all)]

// Re-export all lesson modules
pub mod error_handling;
pub mod generics_and_traits;
pub mod intro;
pub mod linear_search;
pub mod performance_optimization;

// Re-export commonly used items at crate root
pub use error_handling::{SearchError, SearchResult};
pub use generics_and_traits::Searchable;
pub use linear_search::{linear_search, linear_search_naive};
pub use performance_optimization::{binary_search, binary_search_optimized};
