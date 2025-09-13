# CLAUDE.md - AI Assistant Guidance for Rust Learning Courses

This file provides guidance to Claude Code (claude.ai/code) when working with the Rust learning courses in this repository.

## Repository Purpose

This repository contains 14 progressive Rust courses designed to teach Rust through step-by-step lessons, each building on previous concepts. The structure mirrors successful Python learning repositories but leverages Rust's unique features.

## Course Development Standards

### Lesson File Structure

Every lesson file MUST include:

1. **Module-level documentation** with:
   - Brief title
   - Context section explaining real-world use case
   - Concepts covered in the lesson
   - Complexity analysis (time/space)
   - Prerequisites from previous lessons

2. **Code organization**:
   - Start with simplest/naive implementation
   - Progress to idiomatic Rust version
   - Include optimized version if applicable
   - Show common pitfalls and how to avoid them

3. **Documentation requirements**:
   ```rust
   #![warn(missing_docs)]  // Enforce documentation
   ```

4. **Testing approach**:
   - Doctests for every public function
   - Unit tests at bottom of file
   - Integration tests in parallel test files
   - Property-based tests where applicable

### Documentation Template

```rust
//! # Lesson XXX: [Title]
//!
//! ## Context
//! [2-3 sentences explaining real-world scenario where this applies]
//!
//! ## Concepts Covered
//! - [Concept 1]: [Brief explanation]
//! - [Concept 2]: [Brief explanation]
//!
//! ## Complexity Analysis
//! - Time: O(n) - [explanation]
//! - Space: O(1) - [explanation]
//! - Best case: [scenario]
//! - Worst case: [scenario]
//!
//! ## Prerequisites
//! - Lesson XXX: [What concept from that lesson]
//! - Lesson XXX: [What concept from that lesson]
//!
//! ## Real-World Applications
//! This pattern is used in:
//! - [Application 1]
//! - [Application 2]

#![warn(missing_docs)]

/// [Function documentation]
///
/// # Arguments
/// * `param` - [description]
///
/// # Returns
/// [description]
///
/// # Examples
/// ```
/// use course_name::function_name;
///
/// let result = function_name(input);
/// assert_eq!(result, expected);
/// ```
///
/// # Panics
/// [When this might panic, if applicable]
///
/// # Errors
/// [What errors might be returned, if Result type]
pub fn function_name() -> Type {
    // Implementation
}
```

### Code Patterns to Follow

1. **Prefer iterator chains over manual loops**:
   ```rust
   // Good
   data.iter().filter(|x| x > 0).collect()

   // Avoid unless teaching loops specifically
   let mut result = vec![];
   for x in data {
       if x > 0 { result.push(x); }
   }
   ```

2. **Show progression from simple to idiomatic**:
   ```rust
   // Version 1: Naive
   pub fn find_naive(data: &[i32], target: i32) -> bool {
       for i in 0..data.len() {
           if data[i] == target { return true; }
       }
       false
   }

   // Version 2: Idiomatic
   pub fn find(data: &[i32], target: i32) -> bool {
       data.contains(&target)
   }
   ```

3. **Include performance considerations**:
   - Add benchmarks for algorithm lessons
   - Explain trade-offs between approaches
   - Show Big-O complexity in practice

4. **Error handling progression**:
   - Early lessons: Use `Option<T>`
   - Mid lessons: Introduce `Result<T, E>`
   - Advanced: Custom error types

### Testing Requirements

1. **Doctests**: Every public function needs runnable examples
2. **Unit tests**: Test edge cases at bottom of file
3. **Integration tests**: In `tests/` directory
4. **Property tests**: For algorithms and data structures
5. **Benchmarks**: In `benches/` for performance-critical code

### Course Progression Guidelines

1. **000_example**: Reference implementation showing all patterns
2. **dsa_data_structures**: Start with arrays, build to trees
3. **dsa_algorithms**: From O(n²) to O(log n) algorithms
4. **async**: Future trait before async/await syntax
5. **cli**: Parse args manually before using clap
6. **tui**: Raw terminal control before ratatui
7. Each course should have 15-25 lessons

### Common Commands

```bash
# Build all courses
cargo build --workspace

# Test everything
cargo test --workspace --all-targets
cargo test --workspace --doc

# Run specific course tests
cargo test -p courses_000_example

# Check formatting and lints
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings

# Run benchmarks
cargo bench --workspace

# Generate documentation
cargo doc --workspace --no-deps --open
```

### File Naming Convention

- `001_intro.rs` - Introduction and motivation
- `002_basic_[concept].rs` - Simplest form
- `003-019_[topic].rs` - Progressive complexity
- `020_advanced_[topic].rs` - Advanced patterns
- Use subdirectories for related advanced topics

### When Adding New Courses

1. Create directory: `courses/[name]/`
2. Add to workspace in root `Cargo.toml`
3. Follow 000_example structure exactly
4. Start with README.md explaining course goals
5. Implement at least 5 lessons before committing
6. Ensure all tests pass before pushing

### Important Notes

- NEVER skip documentation
- ALWAYS include doctests
- Every lesson must be self-contained
- Prefer clarity over cleverness
- Show multiple approaches when educational
- Include performance measurements for algorithms
- Test with both `cargo test` and `cargo test --doc`
- Maintain consistent style across all courses

## Architecture Decisions

- Single workspace for all courses (easier dependency management)
- Each course is a library crate (allows benching and examples)
- Shared dependencies via workspace.dependencies
- Progressive numbering ensures clear learning path
- Parallel test files allow comprehensive testing without cluttering lessons

## Common Issues and Solutions

1. **Doctest failures**: Ensure all imports are shown or use `# use course_name::*;`
2. **Clippy warnings**: Address them, they're usually correct
3. **Benchmark variance**: Use criterion for statistical significance
4. **Cross-course dependencies**: Avoid them, each course should stand alone