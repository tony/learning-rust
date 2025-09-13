//! # Lesson 002: Linear Search - From Naive to Idiomatic
//!
//! ## Context
//!
//! In a product catalog system, we often need to find items by various criteria.
//! Linear search, while having O(n) complexity, is still useful for small datasets
//! or unsorted collections. Understanding its implementation helps us appreciate
//! more efficient algorithms and Rust's iterator patterns.
//!
//! ## Concepts Covered
//!
//! - Generic functions with trait bounds
//! - Iterator methods vs manual loops
//! - Performance implications of different approaches
//! - Slice patterns and indexing
//! - Option type for nullable returns
//!
//! ## Complexity Analysis
//!
//! - Time: O(n) - Must potentially check every element
//! - Space: O(1) - Only uses constant extra space
//! - Best case: O(1) - Element found at first position
//! - Worst case: O(n) - Element at last position or not present
//! - Average case: O(n/2) ≈ O(n) - Element in middle
//!
//! ## Prerequisites
//!
//! - Lesson 001: Basic Rust syntax and ownership
//!
//! ## Real-World Applications
//!
//! Linear search is used in:
//! - Small, unsorted datasets (< 100 elements)
//! - Searching with complex predicates
//! - One-time searches where sorting isn't worth it
//! - Embedded systems with memory constraints

#![warn(missing_docs)]

use std::fmt::Debug;

/// Performs a naive linear search using manual indexing.
///
/// This implementation shows the most basic approach, similar to
/// what you might write in C or early in your Rust journey.
///
/// # Arguments
///
/// * `data` - The slice to search through
/// * `target` - The value to search for
///
/// # Returns
///
/// `Some(index)` if the target is found, `None` otherwise
///
/// # Examples
///
/// ```
/// use courses_000_example::linear_search::linear_search_naive;
///
/// let numbers = vec![10, 20, 30, 40, 50];
/// assert_eq!(linear_search_naive(&numbers, &30), Some(2));
/// assert_eq!(linear_search_naive(&numbers, &35), None);
///
/// let empty: Vec<i32> = vec![];
/// assert_eq!(linear_search_naive(&empty, &10), None);
/// ```
pub fn linear_search_naive<T: PartialEq>(data: &[T], target: &T) -> Option<usize> {
    for (i, item) in data.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}

/// Performs linear search using iterator enumeration.
///
/// This version is more idiomatic but still explicit about the iteration.
///
/// # Examples
///
/// ```
/// use courses_000_example::linear_search::linear_search_enumerate;
///
/// let words = vec!["apple", "banana", "cherry"];
/// assert_eq!(linear_search_enumerate(&words, &"banana"), Some(1));
/// ```
pub fn linear_search_enumerate<T: PartialEq>(data: &[T], target: &T) -> Option<usize> {
    data.iter()
        .enumerate()
        .find(|(_, item)| *item == target)
        .map(|(index, _)| index)
}

/// Performs linear search using the idiomatic `position` method.
///
/// This is the recommended approach in Rust - concise and clear.
///
/// # Examples
///
/// ```
/// use courses_000_example::linear_search::linear_search;
///
/// let chars = vec!['a', 'b', 'c', 'd'];
/// assert_eq!(linear_search(&chars, &'c'), Some(2));
///
/// // Works with any type that implements PartialEq
/// #[derive(PartialEq)]
/// struct Point { x: i32, y: i32 }
///
/// let points = vec![
///     Point { x: 0, y: 0 },
///     Point { x: 1, y: 1 },
/// ];
/// let target = Point { x: 1, y: 1 };
/// assert_eq!(linear_search(&points, &target), Some(1));
/// ```
pub fn linear_search<T: PartialEq>(data: &[T], target: &T) -> Option<usize> {
    data.iter().position(|item| item == target)
}

/// Searches for the first element matching a predicate.
///
/// This demonstrates the flexibility of iterator-based searching.
///
/// # Arguments
///
/// * `data` - The slice to search through
/// * `predicate` - A function that returns true for the target element
///
/// # Examples
///
/// ```
/// use courses_000_example::linear_search::linear_search_by;
///
/// let numbers = vec![1, 5, 10, 15, 20];
///
/// // Find first number greater than 12
/// let result = linear_search_by(&numbers, |&x| x > 12);
/// assert_eq!(result, Some(3));
///
/// // Find first even number
/// let result = linear_search_by(&numbers, |&x| x % 2 == 0);
/// assert_eq!(result, Some(2));
/// ```
pub fn linear_search_by<T, F>(data: &[T], predicate: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    data.iter().position(predicate)
}

/// Finds all indices where the target appears.
///
/// Demonstrates collecting multiple results from a search.
///
/// # Examples
///
/// ```
/// use courses_000_example::linear_search::find_all_indices;
///
/// let numbers = vec![1, 2, 3, 2, 4, 2];
/// assert_eq!(find_all_indices(&numbers, &2), vec![1, 3, 5]);
///
/// let no_matches = find_all_indices(&numbers, &10);
/// assert!(no_matches.is_empty());
/// ```
pub fn find_all_indices<T: PartialEq>(data: &[T], target: &T) -> Vec<usize> {
    data.iter()
        .enumerate()
        .filter_map(|(index, item)| if item == target { Some(index) } else { None })
        .collect()
}

/// Demonstrates performance comparison between implementations.
///
/// This function is meant to be used in benchmarks to show that
/// all three implementations have similar performance characteristics.
///
/// # Examples
///
/// ```
/// use courses_000_example::linear_search::compare_implementations;
///
/// let data = vec![1, 2, 3, 4, 5];
/// let results = compare_implementations(&data, &3);
/// assert!(results.all_match);
/// assert_eq!(results.naive_result, Some(2));
/// ```
#[derive(Debug, PartialEq)]
pub struct ComparisonResults {
    /// Result from naive implementation
    pub naive_result: Option<usize>,
    /// Result from enumerate implementation
    pub enumerate_result: Option<usize>,
    /// Result from idiomatic implementation
    pub idiomatic_result: Option<usize>,
    /// Whether all implementations returned the same result
    pub all_match: bool,
}

/// Compares all search implementations to verify they return the same results.
pub fn compare_implementations<T: PartialEq + Debug>(data: &[T], target: &T) -> ComparisonResults {
    let naive = linear_search_naive(data, target);
    let enumerate = linear_search_enumerate(data, target);
    let idiomatic = linear_search(data, target);

    ComparisonResults {
        naive_result: naive,
        enumerate_result: enumerate,
        idiomatic_result: idiomatic,
        all_match: naive == enumerate && enumerate == idiomatic,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_empty_slice() {
        let empty: Vec<i32> = vec![];
        assert_eq!(linear_search(&empty, &42), None);
        assert_eq!(linear_search_naive(&empty, &42), None);
        assert_eq!(linear_search_enumerate(&empty, &42), None);
    }

    #[test]
    fn test_search_single_element() {
        let single = vec![42];
        assert_eq!(linear_search(&single, &42), Some(0));
        assert_eq!(linear_search(&single, &0), None);
    }

    #[test]
    fn test_search_multiple_occurrences() {
        let data = vec![1, 2, 3, 2, 4];
        // Should find first occurrence
        assert_eq!(linear_search(&data, &2), Some(1));
    }

    #[test]
    fn test_search_with_strings() {
        let words = vec!["rust", "is", "awesome"];
        assert_eq!(linear_search(&words, &"is"), Some(1));
        assert_eq!(linear_search(&words, &"python"), None);
    }

    #[test]
    fn test_implementations_consistency() {
        let test_cases = vec![
            (vec![1, 2, 3, 4, 5], 3, Some(2)),
            (vec![1, 2, 3, 4, 5], 6, None),
            (vec![5, 4, 3, 2, 1], 5, Some(0)),
            (vec![1], 1, Some(0)),
            (vec![], 1, None),
        ];

        for (data, target, expected) in test_cases {
            let results = compare_implementations(&data, &target);
            assert!(
                results.all_match,
                "Implementations don't match for {:?}",
                data
            );
            assert_eq!(results.naive_result, expected);
        }
    }

    #[test]
    fn test_search_by_predicate() {
        let numbers = vec![1, 3, 5, 7, 9, 11];

        // Find first number > 5
        assert_eq!(linear_search_by(&numbers, |&x| x > 5), Some(3));

        // Find first even number (none exist)
        assert_eq!(linear_search_by(&numbers, |&x| x % 2 == 0), None);

        // Complex predicate
        assert_eq!(
            linear_search_by(&numbers, |&x| x * x > 50),
            Some(4) // 9 * 9 = 81 > 50
        );
    }

    #[test]
    fn test_find_all_indices() {
        let data = vec![1, 2, 1, 3, 1, 4];
        assert_eq!(find_all_indices(&data, &1), vec![0, 2, 4]);

        let no_duplicates = vec![1, 2, 3, 4, 5];
        assert_eq!(find_all_indices(&no_duplicates, &3), vec![2]);

        let empty: Vec<i32> = vec![];
        assert_eq!(find_all_indices(&empty, &1), vec![]);
    }

    #[test]
    fn test_with_custom_types() {
        #[derive(Debug, PartialEq)]
        struct Person {
            name: String,
            age: u32,
        }

        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 30,
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
            },
            Person {
                name: "Charlie".to_string(),
                age: 35,
            },
        ];

        let target = Person {
            name: "Bob".to_string(),
            age: 25,
        };
        assert_eq!(linear_search(&people, &target), Some(1));
    }
}
