//! Full demonstration of the example course functionality.
//!
//! Run with: `cargo run --example full_demo`

use courses_000_example::{
    binary_search, error_handling, generics_and_traits::Container, intro::Lesson, linear_search,
    performance_optimization::compare_search_performance,
};

fn main() {
    println!("🦀 Learning Rust - Example Course Demo\n");

    // Lesson 1: Introduction
    demo_intro();

    // Lesson 2: Linear Search
    demo_search();

    // Lesson 3: Error Handling
    demo_error_handling();

    // Lesson 4: Generics and Traits
    demo_generics();

    // Lesson 5: Performance
    demo_performance();

    println!("\n✅ All demonstrations completed successfully!");
}

fn demo_intro() {
    println!("📚 Lesson 1: Introduction");
    println!("─────────────────────────");

    let lesson = Lesson::new(1, "Introduction to Rust", 30);
    println!("{}", lesson.describe());

    let greeting = courses_000_example::intro::greet("Learner");
    println!("{}", greeting);

    println!();
}

fn demo_search() {
    println!("🔍 Lesson 2: Linear Search");
    println!("──────────────────────────");

    let data = vec![10, 20, 30, 40, 50];
    let target = 30;

    match linear_search(&data, &target) {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found", target),
    }

    // Compare implementations
    let results = courses_000_example::linear_search::compare_implementations(&data, &30);
    println!(
        "All implementations agree: {} (result: {:?})",
        results.all_match, results.naive_result
    );

    println!();
}

fn demo_error_handling() {
    println!("⚠️  Lesson 3: Error Handling");
    println!("────────────────────────────");

    let data = vec![100, 200, 300];

    // Successful search
    match error_handling::find_with_result(&data, 200) {
        Ok(index) => println!("Success: Found at index {}", index),
        Err(e) => println!("Error: {}", e),
    }

    // Failed search
    match error_handling::find_with_result(&data, 999) {
        Ok(index) => println!("Found at index {}", index),
        Err(e) => println!("Expected error: {}", e),
    }

    // Complex operation
    match error_handling::parse_and_find(&data, "200") {
        Ok(index) => println!("Parsed and found at index {}", index),
        Err(e) => println!("Error: {}", e),
    }

    println!();
}

fn demo_generics() {
    println!("🧬 Lesson 4: Generics and Traits");
    println!("─────────────────────────────────");

    let mut container = Container::from_vec(vec![5, 2, 8, 1, 9]);
    println!("Original container: {:?}", container);

    if let Some(index) = container.find(&8) {
        println!("Found 8 at index {}", index);
    }

    use courses_000_example::generics_and_traits::Sortable;
    container.sort();
    println!("Sorted container: {:?}", container);

    println!();
}

fn demo_performance() {
    println!("⚡ Lesson 5: Performance Optimization");
    println!("──────────────────────────────────────");

    // Create sorted data for binary search
    let data: Vec<i32> = (0..10000).collect();
    let target = 7500;

    // Compare performance
    let results = compare_search_performance(&data, target);

    println!("Performance comparison for array of {} elements:", data.len());
    println!("Target: {} (found: {})\n", target, results[0].found);

    for result in &results {
        println!("{:<25} {:>10} ns", format!("{}:", result.name), result.time_ns);
    }

    // Binary search specific
    match binary_search(&data, target) {
        Some(index) => println!("\nBinary search found {} at index {}", target, index),
        None => println!("\n{} not found", target),
    }
}