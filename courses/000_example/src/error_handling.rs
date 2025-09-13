//! # Lesson 003: Error Handling - From Option to Custom Errors
//!
//! ## Context
//!
//! Robust error handling is crucial in systems programming. Rust's approach
//! using `Result` and `Option` types ensures errors are handled explicitly.
//! This lesson shows the evolution from simple `Option` returns to sophisticated
//! custom error types used in production code.
//!
//! ## Concepts Covered
//!
//! - Option<T> for nullable values
//! - Result<T, E> for fallible operations
//! - The ? operator for error propagation
//! - Custom error types with thiserror
//! - Error conversion and context
//!
//! ## Complexity Analysis
//!
//! - Time: O(1) for error creation and propagation
//! - Space: O(1) for stack-allocated errors
//!
//! ## Prerequisites
//!
//! - Lesson 001: Basic Rust syntax
//! - Lesson 002: Generic types and Option
//!
//! ## Real-World Applications
//!
//! Error handling patterns shown here are used in:
//! - Web services (handling request failures)
//! - File I/O operations
//! - Network programming
//! - Database operations

#![warn(missing_docs)]

use std::fmt;
use std::num::ParseIntError;

/// Simple search using Option for not-found cases.
///
/// This is appropriate when the only failure mode is "not found".
///
/// # Examples
///
/// ```
/// use courses_000_example::error_handling::find_with_option;
///
/// let data = vec![10, 20, 30];
/// assert_eq!(find_with_option(&data, 20), Some(1));
/// assert_eq!(find_with_option(&data, 40), None);
/// ```
pub fn find_with_option<T: PartialEq>(data: &[T], target: T) -> Option<usize> {
    data.iter().position(|item| item == &target)
}

/// Basic error type for search operations.
#[derive(Debug, Clone, PartialEq)]
pub enum SearchError {
    /// The collection is empty
    EmptyCollection,
    /// The target was not found
    NotFound,
    /// The index is out of bounds
    IndexOutOfBounds(usize),
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchError::EmptyCollection => write!(f, "Cannot search in empty collection"),
            SearchError::NotFound => write!(f, "Target not found in collection"),
            SearchError::IndexOutOfBounds(idx) => write!(f, "Index {} is out of bounds", idx),
        }
    }
}

impl std::error::Error for SearchError {}

/// Type alias for Results with SearchError.
pub type SearchResult<T> = Result<T, SearchError>;

/// Search that returns a Result with detailed error information.
///
/// # Examples
///
/// ```
/// use courses_000_example::error_handling::{find_with_result, SearchError};
///
/// let data = vec![10, 20, 30];
/// assert_eq!(find_with_result(&data, 20), Ok(1));
/// assert_eq!(find_with_result(&data, 40), Err(SearchError::NotFound));
///
/// let empty: Vec<i32> = vec![];
/// assert_eq!(find_with_result(&empty, 10), Err(SearchError::EmptyCollection));
/// ```
pub fn find_with_result<T: PartialEq>(data: &[T], target: T) -> SearchResult<usize> {
    if data.is_empty() {
        return Err(SearchError::EmptyCollection);
    }

    data.iter()
        .position(|item| item == &target)
        .ok_or(SearchError::NotFound)
}

/// Gets an element by index with proper error handling.
///
/// # Examples
///
/// ```
/// use courses_000_example::error_handling::{get_element, SearchError};
///
/// let data = vec!["a", "b", "c"];
/// assert_eq!(get_element(&data, 1), Ok(&"b"));
/// assert_eq!(get_element(&data, 10), Err(SearchError::IndexOutOfBounds(10)));
/// ```
pub fn get_element<T>(data: &[T], index: usize) -> SearchResult<&T> {
    if data.is_empty() {
        return Err(SearchError::EmptyCollection);
    }

    data.get(index).ok_or(SearchError::IndexOutOfBounds(index))
}

/// Demonstrates the ? operator for error propagation.
///
/// # Examples
///
/// ```
/// use courses_000_example::error_handling::find_and_get_next;
///
/// let data = vec![10, 20, 30, 40];
/// assert_eq!(find_and_get_next(&data, 20), Ok(&30));
/// assert_eq!(find_and_get_next(&data, 40).is_err(), true);
/// ```
pub fn find_and_get_next<T: PartialEq>(data: &[T], target: T) -> SearchResult<&T> {
    let index = find_with_result(data, target)?;
    let next_index = index + 1;
    get_element(data, next_index)
}

/// More complex error type with context.
#[derive(Debug)]
pub struct ParseAndSearchError {
    /// What operation failed
    pub context: String,
    /// The underlying error
    pub source: Box<dyn std::error::Error + Send + Sync>,
}

impl fmt::Display for ParseAndSearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse and search failed: {}", self.context)
    }
}

impl std::error::Error for ParseAndSearchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}

impl From<ParseIntError> for ParseAndSearchError {
    fn from(err: ParseIntError) -> Self {
        ParseAndSearchError {
            context: "Failed to parse integer".to_string(),
            source: Box::new(err),
        }
    }
}

impl From<SearchError> for ParseAndSearchError {
    fn from(err: SearchError) -> Self {
        ParseAndSearchError {
            context: "Search operation failed".to_string(),
            source: Box::new(err),
        }
    }
}

/// Complex operation that can fail in multiple ways.
///
/// # Examples
///
/// ```
/// use courses_000_example::error_handling::parse_and_find;
///
/// let data = vec![10, 20, 30];
/// let result = parse_and_find(&data, "20");
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), 1);
/// assert!(parse_and_find(&data, "abc").is_err()); // Parse error
/// assert!(parse_and_find(&data, "40").is_err());  // Search error
/// ```
pub fn parse_and_find(data: &[i32], target_str: &str) -> Result<usize, ParseAndSearchError> {
    let target = target_str.parse::<i32>()?;
    let index = find_with_result(data, target)?;
    Ok(index)
}

/// Demonstrates error handling with multiple fallback strategies.
///
/// # Examples
///
/// ```
/// use courses_000_example::error_handling::find_with_fallback;
///
/// let primary = vec![10, 20];
/// let fallback = vec![30, 40, 50];
///
/// assert_eq!(find_with_fallback(&primary, &fallback, &20), Some(1));
/// assert_eq!(find_with_fallback(&primary, &fallback, &40), Some(1));
/// assert_eq!(find_with_fallback(&primary, &fallback, &60), None);
/// ```
pub fn find_with_fallback<T: PartialEq>(
    primary: &[T],
    fallback: &[T],
    target: &T,
) -> Option<usize> {
    primary
        .iter()
        .position(|item| item == target)
        .or_else(|| fallback.iter().position(|item| item == target))
}

/// Builder pattern with validation.
#[derive(Debug, Default)]
pub struct SearchConfigBuilder {
    case_sensitive: bool,
    max_results: Option<usize>,
    timeout_ms: Option<u64>,
}

/// Configuration for search operations.
#[derive(Debug)]
pub struct SearchConfig {
    /// Whether the search is case-sensitive
    pub case_sensitive: bool,
    /// Maximum number of results to return
    pub max_results: usize,
    /// Timeout in milliseconds
    pub timeout_ms: u64,
}

/// Errors that can occur when building SearchConfig.
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    /// Invalid timeout value
    InvalidTimeout,
    /// Invalid max results value
    InvalidMaxResults,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidTimeout => write!(f, "Timeout must be greater than 0"),
            ConfigError::InvalidMaxResults => write!(f, "Max results must be greater than 0"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl SearchConfigBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets case sensitivity.
    pub fn case_sensitive(mut self, value: bool) -> Self {
        self.case_sensitive = value;
        self
    }

    /// Sets maximum results.
    pub fn max_results(mut self, value: usize) -> Self {
        self.max_results = Some(value);
        self
    }

    /// Sets timeout in milliseconds.
    pub fn timeout_ms(mut self, value: u64) -> Self {
        self.timeout_ms = Some(value);
        self
    }

    /// Builds the configuration with validation.
    ///
    /// # Examples
    ///
    /// ```
    /// use courses_000_example::error_handling::SearchConfigBuilder;
    ///
    /// let config = SearchConfigBuilder::new()
    ///     .case_sensitive(true)
    ///     .max_results(10)
    ///     .timeout_ms(5000)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(config.case_sensitive, true);
    /// assert_eq!(config.max_results, 10);
    /// assert_eq!(config.timeout_ms, 5000);
    ///
    /// // Invalid configuration
    /// let result = SearchConfigBuilder::new()
    ///     .max_results(0)
    ///     .build();
    /// assert!(result.is_err());
    /// ```
    pub fn build(self) -> Result<SearchConfig, ConfigError> {
        let max_results = self.max_results.unwrap_or(100);
        let timeout_ms = self.timeout_ms.unwrap_or(1000);

        if max_results == 0 {
            return Err(ConfigError::InvalidMaxResults);
        }

        if timeout_ms == 0 {
            return Err(ConfigError::InvalidTimeout);
        }

        Ok(SearchConfig {
            case_sensitive: self.case_sensitive,
            max_results,
            timeout_ms,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_pattern() {
        let data = vec![1, 2, 3];
        assert_eq!(find_with_option(&data, 2), Some(1));
        assert_eq!(find_with_option(&data, 5), None);
    }

    #[test]
    fn test_result_pattern() {
        let data = vec![1, 2, 3];
        assert_eq!(find_with_result(&data, 2), Ok(1));
        assert_eq!(find_with_result(&data, 5), Err(SearchError::NotFound));

        let empty: Vec<i32> = vec![];
        assert_eq!(
            find_with_result(&empty, 1),
            Err(SearchError::EmptyCollection)
        );
    }

    #[test]
    fn test_error_propagation() {
        let data = vec![10, 20, 30];

        // Success case
        assert_eq!(find_and_get_next(&data, 10), Ok(&20));

        // Not found
        assert!(matches!(
            find_and_get_next(&data, 40),
            Err(SearchError::NotFound)
        ));

        // Found but next index out of bounds
        assert!(matches!(
            find_and_get_next(&data, 30),
            Err(SearchError::IndexOutOfBounds(_))
        ));
    }

    #[test]
    fn test_error_conversion() {
        let data = vec![10, 20, 30];

        // Successful parse and find
        let result = parse_and_find(&data, "20");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);

        // Parse error
        let parse_err = parse_and_find(&data, "abc");
        assert!(parse_err.is_err());
        if let Err(e) = parse_err {
            // Check if context mentions parsing OR if it mentions failure
            assert!(
                e.context.contains("parse")
                    || e.context.contains("Parse")
                    || e.context.contains("Failed")
            );
        }

        // Search error
        let search_err = parse_and_find(&data, "40");
        assert!(search_err.is_err());
        if let Err(e) = search_err {
            assert!(e.context.contains("Search"));
        }
    }

    #[test]
    fn test_fallback_strategy() {
        let primary = vec!["a", "b"];
        let fallback = vec!["c", "d", "e"];

        assert_eq!(find_with_fallback(&primary, &fallback, &"b"), Some(1));
        assert_eq!(find_with_fallback(&primary, &fallback, &"d"), Some(1));
        assert_eq!(find_with_fallback(&primary, &fallback, &"f"), None);
    }

    #[test]
    fn test_builder_pattern() {
        // Valid configuration
        let config = SearchConfigBuilder::new()
            .case_sensitive(true)
            .max_results(50)
            .timeout_ms(2000)
            .build()
            .unwrap();

        assert_eq!(config.case_sensitive, true);
        assert_eq!(config.max_results, 50);
        assert_eq!(config.timeout_ms, 2000);

        // Default values
        let default_config = SearchConfigBuilder::new().build().unwrap();
        assert_eq!(default_config.case_sensitive, false);
        assert_eq!(default_config.max_results, 100);
        assert_eq!(default_config.timeout_ms, 1000);

        // Invalid configurations
        assert!(matches!(
            SearchConfigBuilder::new().max_results(0).build(),
            Err(ConfigError::InvalidMaxResults)
        ));

        assert!(matches!(
            SearchConfigBuilder::new().timeout_ms(0).build(),
            Err(ConfigError::InvalidTimeout)
        ));
    }

    #[test]
    fn test_error_display() {
        assert_eq!(
            SearchError::EmptyCollection.to_string(),
            "Cannot search in empty collection"
        );
        assert_eq!(
            SearchError::NotFound.to_string(),
            "Target not found in collection"
        );
        assert_eq!(
            SearchError::IndexOutOfBounds(5).to_string(),
            "Index 5 is out of bounds"
        );
    }
}
