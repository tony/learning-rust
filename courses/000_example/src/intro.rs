//! # Lesson 001: Introduction to Learning Rust Courses
//!
//! ## Context
//!
//! When learning a new programming language, having a structured path with
//! clear examples and progressive complexity helps build understanding
//! systematically. This introductory lesson establishes the patterns used
//! throughout all courses.
//!
//! ## Concepts Covered
//!
//! - Module documentation with `//!` comments
//! - Function documentation with `///` comments
//! - Doctests for inline examples
//! - Basic Rust syntax and conventions
//! - Testing patterns
//!
//! ## Complexity Analysis
//!
//! - Time: O(1) - All operations in this lesson are constant time
//! - Space: O(1) - No dynamic allocation
//!
//! ## Prerequisites
//!
//! - None (this is the first lesson)
//!
//! ## Real-World Applications
//!
//! Documentation and testing patterns shown here are used in:
//! - Production Rust codebases
//! - Open source libraries
//! - System programming projects

#![warn(missing_docs)]

/// A simple greeting function demonstrating basic Rust syntax.
///
/// This function shows:
/// - Function declaration syntax
/// - String references as parameters
/// - String formatting with `format!` macro
///
/// # Arguments
///
/// * `name` - The name to include in the greeting
///
/// # Returns
///
/// A formatted greeting string
///
/// # Examples
///
/// ```
/// use courses_000_example::intro::greet;
///
/// let message = greet("Rust Learner");
/// assert_eq!(message, "Hello, Rust Learner! Welcome to Learning Rust.");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Learning Rust.", name)
}

/// Demonstrates ownership and borrowing concepts.
///
/// This function shows the difference between moving and borrowing values,
/// a fundamental concept in Rust's memory management.
///
/// # Examples
///
/// ```
/// use courses_000_example::intro::demonstrate_ownership;
///
/// let original = String::from("Hello");
/// let result = demonstrate_ownership(original.clone());
/// assert_eq!(result, "Hello, World!");
/// // original can still be used if cloned
/// ```
pub fn demonstrate_ownership(mut owned_string: String) -> String {
    owned_string.push_str(", World!");
    owned_string
}

/// Shows the difference between mutable and immutable references.
///
/// # Arguments
///
/// * `immutable` - An immutable reference (can read but not modify)
/// * `mutable` - A mutable reference (can read and modify)
///
/// # Examples
///
/// ```
/// use courses_000_example::intro::demonstrate_references;
///
/// let value = 42;
/// let mut counter = 0;
/// let result = demonstrate_references(&value, &mut counter);
/// assert_eq!(result, 42);
/// assert_eq!(counter, 1);
/// ```
pub fn demonstrate_references(immutable: &i32, mutable: &mut i32) -> i32 {
    *mutable += 1;
    *immutable
}

/// A simple struct demonstrating Rust's type system.
///
/// This shows how to define custom types with derived traits.
#[derive(Debug, Clone, PartialEq)]
pub struct Lesson {
    /// The lesson number (e.g., 001, 002)
    pub number: u32,
    /// The lesson title
    pub title: String,
    /// Estimated time to complete in minutes
    pub duration_minutes: u32,
}

impl Lesson {
    /// Creates a new lesson with the given parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use courses_000_example::intro::Lesson;
    ///
    /// let lesson = Lesson::new(1, "Introduction", 30);
    /// assert_eq!(lesson.number, 1);
    /// assert_eq!(lesson.title, "Introduction");
    /// assert_eq!(lesson.duration_minutes, 30);
    /// ```
    pub fn new(number: u32, title: impl Into<String>, duration_minutes: u32) -> Self {
        Self {
            number,
            title: title.into(),
            duration_minutes,
        }
    }

    /// Returns a formatted description of the lesson.
    ///
    /// # Examples
    ///
    /// ```
    /// use courses_000_example::intro::Lesson;
    ///
    /// let lesson = Lesson::new(1, "Introduction", 30);
    /// let description = lesson.describe();
    /// assert_eq!(description, "Lesson 001: Introduction (30 minutes)");
    /// ```
    pub fn describe(&self) -> String {
        format!(
            "Lesson {:03}: {} ({} minutes)",
            self.number, self.title, self.duration_minutes
        )
    }
}

/// Demonstrates pattern matching, a powerful Rust feature.
///
/// # Arguments
///
/// * `value` - An optional integer to process
///
/// # Returns
///
/// A description of the value
///
/// # Examples
///
/// ```
/// use courses_000_example::intro::describe_option;
///
/// assert_eq!(describe_option(Some(42)), "The value is 42");
/// assert_eq!(describe_option(Some(0)), "The value is zero");
/// assert_eq!(describe_option(None), "No value provided");
/// ```
pub fn describe_option(value: Option<i32>) -> String {
    match value {
        Some(0) => "The value is zero".to_string(),
        Some(n) => format!("The value is {}", n),
        None => "No value provided".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_with_empty_name() {
        let result = greet("");
        assert_eq!(result, "Hello, ! Welcome to Learning Rust.");
    }

    #[test]
    fn test_greet_with_unicode() {
        let result = greet("世界");
        assert_eq!(result, "Hello, 世界! Welcome to Learning Rust.");
    }

    #[test]
    fn test_ownership_transfer() {
        let original = String::from("Test");
        let result = demonstrate_ownership(original);
        assert_eq!(result, "Test, World!");
        // original is moved and cannot be used here
    }

    #[test]
    fn test_references_dont_move() {
        let value = 100;
        let mut counter = 5;

        let result = demonstrate_references(&value, &mut counter);

        assert_eq!(result, 100);
        assert_eq!(counter, 6);
        assert_eq!(value, 100); // value can still be used
    }

    #[test]
    fn test_lesson_creation_with_string() {
        let lesson = Lesson::new(42, String::from("Advanced Topics"), 60);
        assert_eq!(lesson.number, 42);
        assert_eq!(lesson.title, "Advanced Topics");
        assert_eq!(lesson.duration_minutes, 60);
    }

    #[test]
    fn test_lesson_creation_with_str() {
        let lesson = Lesson::new(1, "Basic Concepts", 15);
        assert_eq!(lesson.describe(), "Lesson 001: Basic Concepts (15 minutes)");
    }

    #[test]
    fn test_lesson_equality() {
        let lesson1 = Lesson::new(1, "Test", 30);
        let lesson2 = Lesson::new(1, "Test", 30);
        let lesson3 = Lesson::new(2, "Test", 30);

        assert_eq!(lesson1, lesson2);
        assert_ne!(lesson1, lesson3);
    }

    #[test]
    fn test_pattern_matching_completeness() {
        assert_eq!(describe_option(Some(42)), "The value is 42");
        assert_eq!(describe_option(Some(0)), "The value is zero");
        assert_eq!(describe_option(Some(-5)), "The value is -5");
        assert_eq!(describe_option(None), "No value provided");
    }
}
