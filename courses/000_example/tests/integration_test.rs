//! Integration tests for the example course.
//!
//! These tests verify that all lessons work together correctly
//! and demonstrate testing patterns for course modules.

use courses_000_example::{SearchError, binary_search, linear_search, linear_search_naive};

#[test]
fn test_all_search_implementations_agree() {
    let data = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];

    for target in 0..21 {
        let linear_result = linear_search(&data, &target);
        let naive_result = linear_search_naive(&data, &target);
        let binary_result = binary_search(&data, target);

        // All implementations should agree
        assert_eq!(linear_result, naive_result);
        assert_eq!(linear_result, binary_result);
    }
}

#[test]
fn test_error_handling_integration() {
    use courses_000_example::error_handling::find_and_get_next;

    let data = vec![10, 20, 30, 40, 50];

    // Successful case
    let result = find_and_get_next(&data, 30);
    assert_eq!(result, Ok(&40));

    // Error propagation
    let result = find_and_get_next(&data, 60);
    assert!(matches!(result, Err(SearchError::NotFound)));

    // Edge case - last element
    let result = find_and_get_next(&data, 50);
    assert!(matches!(result, Err(SearchError::IndexOutOfBounds(_))));
}

#[test]
fn test_generic_container_integration() {
    use courses_000_example::generics_and_traits::{Container, Sortable};

    let mut container = Container::from_vec(vec![5, 2, 8, 1, 9]);
    container.sort();

    let sorted = container.to_vec();
    assert_eq!(sorted, vec![1, 2, 5, 8, 9]);

    // Verify binary search works on sorted container
    let index = container.find(&5);
    assert_eq!(index, Some(2));
}

#[test]
fn test_performance_comparison() {
    use courses_000_example::performance_optimization::compare_search_performance;

    let data: Vec<i32> = (0..10000).collect();

    let results = compare_search_performance(&data, 5000);

    // All algorithms should find the target
    for result in &results {
        assert!(
            result.found,
            "Algorithm {} failed to find target",
            result.name
        );
    }

    // Binary search should be faster than linear for large datasets
    let linear_time = results
        .iter()
        .find(|r| r.name == "Linear Search")
        .unwrap()
        .time_ns;

    let binary_time = results
        .iter()
        .find(|r| r.name == "Binary Search")
        .unwrap()
        .time_ns;

    // Note: This might not always be true for small datasets or in debug mode
    // but should be true for release mode with optimization
    #[cfg(not(debug_assertions))]
    {
        assert!(
            binary_time < linear_time,
            "Binary search should be faster than linear search for large datasets"
        );
    }
}

#[test]
fn test_bounded_container() {
    use courses_000_example::generics_and_traits::BoundedContainer;

    let mut container: BoundedContainer<String, 3> = BoundedContainer::new();

    assert!(container.push("first".to_string()).is_ok());
    assert!(container.push("second".to_string()).is_ok());
    assert!(container.push("third".to_string()).is_ok());

    // Container is now full
    assert!(container.is_full());
    assert_eq!(
        container.push("fourth".to_string()),
        Err("fourth".to_string())
    );

    let items = container.to_vec();
    assert_eq!(items.len(), 3);
    assert_eq!(items[0], "first");
}

#[test]
fn test_product_type() {
    use courses_000_example::generics_and_traits::{Describable, Product};

    let products = vec![
        Product {
            id: 1,
            name: "Laptop".to_string(),
            price_cents: 99900,
        },
        Product {
            id: 2,
            name: "Mouse".to_string(),
            price_cents: 2500,
        },
        Product {
            id: 3,
            name: "Keyboard".to_string(),
            price_cents: 7500,
        },
    ];

    // Test sorting by price
    let mut sorted = products.clone();
    sorted.sort();
    assert_eq!(sorted[0].name, "Mouse");
    assert_eq!(sorted[2].name, "Laptop");

    // Test describable trait
    for product in &products {
        let description = product.describe();
        assert!(description.contains(&product.name));
        assert!(description.contains(&product.id.to_string()));
    }
}

#[test]
fn test_lesson_structure() {
    use courses_000_example::intro::Lesson;

    let lessons = vec![
        Lesson::new(1, "Introduction", 30),
        Lesson::new(2, "Linear Search", 45),
        Lesson::new(3, "Error Handling", 60),
        Lesson::new(4, "Generics and Traits", 75),
        Lesson::new(5, "Performance Optimization", 90),
    ];

    let total_duration: u32 = lessons.iter().map(|l| l.duration_minutes).sum();
    assert_eq!(total_duration, 300); // 5 hours total

    for (i, lesson) in lessons.iter().enumerate() {
        assert_eq!(lesson.number as usize, i + 1);
        let description = lesson.describe();
        assert!(description.contains(&format!("{:03}", lesson.number)));
    }
}
