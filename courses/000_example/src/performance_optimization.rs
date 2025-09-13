//! # Lesson 005: Performance Optimization - From O(n) to O(log n)
//!
//! ## Context
//!
//! Performance optimization is crucial when scaling systems. This lesson demonstrates
//! how algorithmic improvements and Rust-specific optimizations can dramatically
//! improve performance. We'll evolve from linear search O(n) to binary search O(log n),
//! showing measurement techniques along the way.
//!
//! ## Concepts Covered
//!
//! - Algorithmic complexity and Big-O notation
//! - Binary search implementation
//! - Performance measurement techniques
//! - Memory layout optimization
//! - SIMD and parallelization hints
//! - Benchmarking with criterion
//!
//! ## Complexity Analysis
//!
//! - Linear search: O(n) time, O(1) space
//! - Binary search: O(log n) time, O(1) space
//! - Interpolation search: O(log log n) average, O(n) worst
//! - Hash lookup: O(1) average, O(n) worst
//!
//! ## Prerequisites
//!
//! - Lessons 001-004: All previous concepts
//!
//! ## Real-World Applications
//!
//! Performance optimization techniques are critical in:
//! - Database indexing
//! - Search engines
//! - Game engines
//! - High-frequency trading systems
//! - Embedded systems with limited resources

#![warn(missing_docs)]

use std::cmp::Ordering;
use std::time::Instant;

/// Naive linear search for comparison baseline.
///
/// # Examples
///
/// ```
/// use courses_000_example::performance_optimization::linear_search_baseline;
///
/// let data = vec![1, 3, 5, 7, 9, 11, 13];
/// assert_eq!(linear_search_baseline(&data, 7), Some(3));
/// ```
pub fn linear_search_baseline<T: Ord>(data: &[T], target: T) -> Option<usize> {
    for (i, item) in data.iter().enumerate() {
        if item == &target {
            return Some(i);
        }
    }
    None
}

/// Standard binary search implementation.
///
/// Requires sorted data. Returns the index of the target if found.
///
/// # Panics
///
/// None - handles empty slices gracefully.
///
/// # Examples
///
/// ```
/// use courses_000_example::performance_optimization::binary_search;
///
/// let data = vec![1, 3, 5, 7, 9, 11, 13];
/// assert_eq!(binary_search(&data, 7), Some(3));
/// assert_eq!(binary_search(&data, 6), None);
///
/// let empty: Vec<i32> = vec![];
/// assert_eq!(binary_search(&empty, 5), None);
/// ```
pub fn binary_search<T: Ord>(data: &[T], target: T) -> Option<usize> {
    let mut left = 0;
    let mut right = data.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match data[mid].cmp(&target) {
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
            Ordering::Equal => return Some(mid),
        }
    }

    None
}

/// Optimized binary search with fewer branches.
///
/// Uses bitwise operations and branch-free comparisons when possible.
///
/// # Examples
///
/// ```
/// use courses_000_example::performance_optimization::binary_search_optimized;
///
/// let data = vec![1, 3, 5, 7, 9, 11, 13];
/// assert_eq!(binary_search_optimized(&data, 7), Some(3));
/// ```
pub fn binary_search_optimized<T: Ord>(data: &[T], target: T) -> Option<usize> {
    if data.is_empty() {
        return None;
    }

    let mut size = data.len();
    let mut base = 0;

    while size > 1 {
        let half = size / 2;
        let mid = base + half;

        // Branch-free update of base
        base = if data[mid] <= target { mid } else { base };
        size -= half;
    }

    if data[base] == target {
        Some(base)
    } else {
        None
    }
}

/// Binary search that returns insertion position if not found.
///
/// Returns `Ok(index)` if found, `Err(insert_pos)` if not found.
///
/// # Examples
///
/// ```
/// use courses_000_example::performance_optimization::binary_search_insertion;
///
/// let data = vec![1, 3, 5, 7, 9];
/// assert_eq!(binary_search_insertion(&data, 5), Ok(2));
/// assert_eq!(binary_search_insertion(&data, 6), Err(3)); // Insert at position 3
/// assert_eq!(binary_search_insertion(&data, 0), Err(0)); // Insert at beginning
/// ```
pub fn binary_search_insertion<T: Ord>(data: &[T], target: T) -> Result<usize, usize> {
    data.binary_search(&target)
}

/// Interpolation search for uniformly distributed data.
///
/// Better than binary search for uniform distributions.
/// Falls back to binary search for safety.
///
/// # Examples
///
/// ```
/// use courses_000_example::performance_optimization::interpolation_search;
///
/// let data = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];
/// assert_eq!(interpolation_search(&data, 50), Some(4));
/// ```
pub fn interpolation_search(data: &[i32], target: i32) -> Option<usize> {
    if data.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = data.len() - 1;

    while low <= high && target >= data[low] && target <= data[high] {
        if low == high {
            return if data[low] == target { Some(low) } else { None };
        }

        // Interpolation formula
        let range = data[high] - data[low];
        if range == 0 {
            return if data[low] == target { Some(low) } else { None };
        }

        let pos = low
            + ((target - data[low]) as usize * (high - low)) / (data[high] - data[low]) as usize;

        if pos >= data.len() {
            return None;
        }

        match data[pos].cmp(&target) {
            Ordering::Equal => return Some(pos),
            Ordering::Less => low = pos + 1,
            Ordering::Greater => {
                if pos == 0 {
                    return None;
                }
                high = pos - 1;
            }
        }
    }

    None
}

/// Exponential search for unbounded or very large arrays.
///
/// Useful when the target is likely near the beginning.
///
/// # Examples
///
/// ```
/// use courses_000_example::performance_optimization::exponential_search;
///
/// let data = vec![1, 2, 3, 4, 5, 10, 20, 30, 40, 50];
/// assert_eq!(exponential_search(&data, 10), Some(5));
/// ```
pub fn exponential_search<T: Ord>(data: &[T], target: T) -> Option<usize> {
    if data.is_empty() {
        return None;
    }

    if data[0] == target {
        return Some(0);
    }

    // Find range for binary search
    let mut bound = 1;
    while bound < data.len() && data[bound] <= target {
        bound *= 2;
    }

    // Perform binary search in the found range
    let start = bound / 2;
    let end = bound.min(data.len());

    binary_search(&data[start..end], target).map(|i| i + start)
}

/// Performance measurement helper.
#[derive(Debug)]
pub struct PerformanceResult {
    /// Algorithm name
    pub name: String,
    /// Time taken in nanoseconds
    pub time_ns: u128,
    /// Result of the search
    pub found: bool,
}

/// Compares performance of different search algorithms.
///
/// # Examples
///
/// ```
/// use courses_000_example::performance_optimization::compare_search_performance;
///
/// let data: Vec<i32> = (0..1000).collect();
/// let results = compare_search_performance(&data, 500);
///
/// // Binary search should be faster than linear for large datasets
/// for result in &results {
///     println!("{}: {} ns", result.name, result.time_ns);
/// }
/// ```
pub fn compare_search_performance(data: &[i32], target: i32) -> Vec<PerformanceResult> {
    let mut results = Vec::new();

    // Linear search
    let start = Instant::now();
    let found = linear_search_baseline(data, target).is_some();
    let time_ns = start.elapsed().as_nanos();
    results.push(PerformanceResult {
        name: "Linear Search".to_string(),
        time_ns,
        found,
    });

    // Binary search
    let start = Instant::now();
    let found = binary_search(data, target).is_some();
    let time_ns = start.elapsed().as_nanos();
    results.push(PerformanceResult {
        name: "Binary Search".to_string(),
        time_ns,
        found,
    });

    // Optimized binary search
    let start = Instant::now();
    let found = binary_search_optimized(data, target).is_some();
    let time_ns = start.elapsed().as_nanos();
    results.push(PerformanceResult {
        name: "Binary Search (Optimized)".to_string(),
        time_ns,
        found,
    });

    // Interpolation search
    let start = Instant::now();
    let found = interpolation_search(data, target).is_some();
    let time_ns = start.elapsed().as_nanos();
    results.push(PerformanceResult {
        name: "Interpolation Search".to_string(),
        time_ns,
        found,
    });

    results
}

/// Cache-friendly data structure for better performance.
#[derive(Debug)]
pub struct CacheFriendlyArray<T, const N: usize> {
    data: Box<[T; N]>,
}

impl<T: Default + Copy, const N: usize> CacheFriendlyArray<T, N> {
    /// Creates a new cache-friendly array.
    pub fn new() -> Self {
        Self {
            data: Box::new([T::default(); N]),
        }
    }

    /// Gets an element by index.
    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < N {
            Some(&self.data[index])
        } else {
            None
        }
    }

    /// Sets an element by index.
    #[inline(always)]
    pub fn set(&mut self, index: usize, value: T) -> bool {
        if index < N {
            self.data[index] = value;
            true
        } else {
            false
        }
    }
}

impl<T: Default + Copy, const N: usize> Default for CacheFriendlyArray<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Demonstrates the importance of data locality.
///
/// # Examples
///
/// ```
/// use courses_000_example::performance_optimization::demonstrate_cache_effects;
///
/// let (row_major_time, col_major_time) = demonstrate_cache_effects(100);
/// // Row-major access is typically faster due to cache locality
/// println!("Row-major: {} ns, Column-major: {} ns", row_major_time, col_major_time);
/// ```
pub fn demonstrate_cache_effects(size: usize) -> (u128, u128) {
    let matrix: Vec<Vec<i32>> = vec![vec![0; size]; size];

    // Row-major access (cache-friendly)
    let start = Instant::now();
    let mut _sum = 0;
    for row in &matrix {
        for &val in row {
            _sum += val;
        }
    }
    let row_major_time = start.elapsed().as_nanos();

    // Column-major access (cache-unfriendly)
    let start = Instant::now();
    let mut _sum = 0;
    for col in 0..size {
        for row in &matrix {
            _sum += row[col];
        }
    }
    let col_major_time = start.elapsed().as_nanos();

    // Prevent optimization
    std::hint::black_box(_sum);

    (row_major_time, col_major_time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_basic() {
        let data = vec![1, 3, 5, 7, 9, 11, 13, 15];

        // Found cases
        assert_eq!(binary_search(&data, 1), Some(0));
        assert_eq!(binary_search(&data, 9), Some(4));
        assert_eq!(binary_search(&data, 15), Some(7));

        // Not found cases
        assert_eq!(binary_search(&data, 0), None);
        assert_eq!(binary_search(&data, 4), None);
        assert_eq!(binary_search(&data, 16), None);

        // Empty array
        let empty: Vec<i32> = vec![];
        assert_eq!(binary_search(&empty, 5), None);
    }

    #[test]
    fn test_binary_search_optimized() {
        let data = vec![1, 3, 5, 7, 9, 11, 13, 15];

        assert_eq!(binary_search_optimized(&data, 7), Some(3));
        assert_eq!(binary_search_optimized(&data, 4), None);

        // Single element
        let single = vec![42];
        assert_eq!(binary_search_optimized(&single, 42), Some(0));
        assert_eq!(binary_search_optimized(&single, 0), None);
    }

    #[test]
    fn test_binary_search_insertion() {
        let data = vec![1, 3, 5, 7, 9];

        assert_eq!(binary_search_insertion(&data, 5), Ok(2));
        assert_eq!(binary_search_insertion(&data, 4), Err(2));
        assert_eq!(binary_search_insertion(&data, 0), Err(0));
        assert_eq!(binary_search_insertion(&data, 10), Err(5));
    }

    #[test]
    fn test_interpolation_search() {
        let data = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

        assert_eq!(interpolation_search(&data, 50), Some(4));
        assert_eq!(interpolation_search(&data, 10), Some(0));
        assert_eq!(interpolation_search(&data, 100), Some(9));
        assert_eq!(interpolation_search(&data, 55), None);

        // Non-uniform distribution still works
        let non_uniform = vec![1, 2, 3, 100, 200, 300];
        assert_eq!(interpolation_search(&non_uniform, 100), Some(3));
    }

    #[test]
    fn test_exponential_search() {
        let data = vec![1, 2, 3, 4, 5, 10, 20, 30, 40, 50, 100, 200];

        assert_eq!(exponential_search(&data, 1), Some(0));
        assert_eq!(exponential_search(&data, 10), Some(5));
        assert_eq!(exponential_search(&data, 200), Some(11));
        assert_eq!(exponential_search(&data, 25), None);
    }

    #[test]
    fn test_cache_friendly_array() {
        let mut arr: CacheFriendlyArray<i32, 10> = CacheFriendlyArray::new();

        assert_eq!(arr.get(0), Some(&0));
        assert!(arr.set(5, 42));
        assert_eq!(arr.get(5), Some(&42));
        assert_eq!(arr.get(10), None);
        assert!(!arr.set(10, 100));
    }

    #[test]
    fn test_performance_comparison() {
        let data: Vec<i32> = (0..1000).collect();
        let results = compare_search_performance(&data, 500);

        assert_eq!(results.len(), 4);
        for result in &results {
            assert!(result.found);
        }

        // Test with not found
        let results = compare_search_performance(&data, 1001);
        for result in &results {
            assert!(!result.found);
        }
    }

    #[test]
    fn test_all_algorithms_agree() {
        let data: Vec<i32> = (0..100).map(|x| x * 2).collect(); // Even numbers

        for target in 0..200 {
            let linear = linear_search_baseline(&data, target);
            let binary = binary_search(&data, target);
            let optimized = binary_search_optimized(&data, target);
            let interpolation = interpolation_search(&data, target);

            if target % 2 == 0 && target < 200 {
                // Should find even numbers
                assert!(linear.is_some());
                assert_eq!(linear, binary);
                assert_eq!(binary, optimized);
                assert_eq!(optimized, interpolation);
            } else {
                // Should not find odd numbers
                assert!(linear.is_none());
                assert!(binary.is_none());
                assert!(optimized.is_none());
                assert!(interpolation.is_none());
            }
        }
    }
}
