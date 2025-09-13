//! # Lesson 004: Generics and Traits - Abstraction in Rust
//!
//! ## Context
//!
//! Generic programming allows us to write flexible, reusable code that works
//! with multiple types. Traits define shared behavior that types can implement.
//! Together, they form the foundation of Rust's zero-cost abstractions, enabling
//! code that is both generic and performant.
//!
//! ## Concepts Covered
//!
//! - Generic types and functions
//! - Trait definitions and implementations
//! - Trait bounds and where clauses
//! - Associated types and constants
//! - Default implementations
//! - Trait objects and dynamic dispatch
//!
//! ## Complexity Analysis
//!
//! - Compile-time: Monomorphization generates specialized code for each type
//! - Runtime: Zero-cost abstractions mean no performance penalty
//! - Binary size: Can increase due to monomorphization
//!
//! ## Prerequisites
//!
//! - Lesson 001-003: Basic Rust, ownership, error handling
//!
//! ## Real-World Applications
//!
//! Generics and traits are used in:
//! - Collection types (Vec<T>, HashMap<K, V>)
//! - Iterator chains and adaptors
//! - Async runtimes (Future trait)
//! - Serialization frameworks (Serde)

#![warn(missing_docs)]

use std::cmp::Ordering;
use std::fmt::Debug;

/// A trait for types that can be searched within a collection.
pub trait Searchable: PartialEq {
    /// Returns a value that can be used for comparison during search.
    fn search_key(&self) -> &Self;
}

/// Default implementation for most types - they are their own search key.
impl<T: PartialEq> Searchable for T {
    fn search_key(&self) -> &Self {
        self
    }
}

/// Generic container that can hold any type.
#[derive(Debug, Clone)]
pub struct Container<T> {
    items: Vec<T>,
}

impl<T> Container<T> {
    /// Creates a new empty container.
    ///
    /// # Examples
    ///
    /// ```
    /// use courses_000_example::generics_and_traits::Container;
    ///
    /// let container: Container<i32> = Container::new();
    /// assert_eq!(container.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Creates a container from a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use courses_000_example::generics_and_traits::Container;
    ///
    /// let container = Container::from_vec(vec![1, 2, 3]);
    /// assert_eq!(container.len(), 3);
    /// ```
    pub fn from_vec(items: Vec<T>) -> Self {
        Self { items }
    }

    /// Adds an item to the container.
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Returns the number of items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Checks if the container is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T: PartialEq> Container<T> {
    /// Finds an item in the container.
    ///
    /// This method is only available when T implements PartialEq.
    ///
    /// # Examples
    ///
    /// ```
    /// use courses_000_example::generics_and_traits::Container;
    ///
    /// let container = Container::from_vec(vec!["a", "b", "c"]);
    /// assert_eq!(container.find(&"b"), Some(1));
    /// ```
    pub fn find(&self, target: &T) -> Option<usize> {
        self.items.iter().position(|item| item == target)
    }

    /// Checks if the container contains an item.
    pub fn contains(&self, target: &T) -> bool {
        self.find(target).is_some()
    }
}

impl<T: Clone> Container<T> {
    /// Returns a vector of cloned items.
    ///
    /// This method is only available when T implements Clone.
    pub fn to_vec(&self) -> Vec<T> {
        self.items.clone()
    }
}

impl<T: Default> Default for Container<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A trait for sortable containers.
pub trait Sortable {
    /// The type of items in the container.
    type Item;

    /// Sorts the container in place.
    fn sort(&mut self)
    where
        Self::Item: Ord;

    /// Returns a sorted copy without modifying the original.
    fn sorted(&self) -> Self
    where
        Self: Clone,
        Self::Item: Ord;
}

impl<T> Sortable for Container<T> {
    type Item = T;

    fn sort(&mut self)
    where
        T: Ord,
    {
        self.items.sort();
    }

    fn sorted(&self) -> Self
    where
        Self: Clone,
        T: Ord,
    {
        let mut cloned = self.clone();
        cloned.sort();
        cloned
    }
}

/// A trait with a default implementation.
pub trait Describable {
    /// Returns a description of the type.
    fn describe(&self) -> String {
        String::from("A describable item")
    }

    /// Returns a detailed description.
    fn describe_detailed(&self) -> String {
        format!("{} (detailed)", self.describe())
    }
}

/// Custom type demonstrating trait implementation.
#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    /// Product ID
    pub id: u32,
    /// Product name
    pub name: String,
    /// Price in cents
    pub price_cents: u32,
}

impl Describable for Product {
    fn describe(&self) -> String {
        format!("Product: {} (ID: {})", self.name, self.id)
    }
}

impl PartialOrd for Product {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Product {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price_cents.cmp(&other.price_cents)
    }
}

impl Eq for Product {}

/// Generic function with multiple trait bounds.
///
/// # Examples
///
/// ```
/// use courses_000_example::generics_and_traits::find_min_max;
///
/// let numbers = vec![3, 1, 4, 1, 5, 9];
/// let (min, max) = find_min_max(&numbers).unwrap();
/// assert_eq!(*min, 1);
/// assert_eq!(*max, 9);
/// ```
pub fn find_min_max<T>(items: &[T]) -> Option<(&T, &T)>
where
    T: Ord,
{
    if items.is_empty() {
        return None;
    }

    let min = items.iter().min()?;
    let max = items.iter().max()?;
    Some((min, max))
}

/// Function with where clause for complex bounds.
///
/// # Examples
///
/// ```
/// use courses_000_example::generics_and_traits::process_and_describe;
///
/// let numbers = vec![3, 1, 2];
/// let result = process_and_describe(numbers);
/// assert!(result.contains("Sorted:"));
/// ```
pub fn process_and_describe<T>(mut items: Vec<T>) -> String
where
    T: Ord + Debug + Clone,
{
    items.sort();
    format!("Sorted: {:?}", items)
}

/// Trait object demonstration - dynamic dispatch.
pub fn describe_items(items: &[&dyn Describable]) -> Vec<String> {
    items.iter().map(|item| item.describe()).collect()
}

/// Generic struct with associated const.
pub struct BoundedContainer<T, const N: usize> {
    items: [Option<T>; N],
    count: usize,
}

impl<T, const N: usize> BoundedContainer<T, N> {
    /// Creates a new bounded container.
    ///
    /// # Examples
    ///
    /// ```
    /// use courses_000_example::generics_and_traits::BoundedContainer;
    ///
    /// let container: BoundedContainer<i32, 5> = BoundedContainer::new();
    /// assert_eq!(container.capacity(), 5);
    /// ```
    pub const fn new() -> Self {
        Self {
            items: [const { None }; N],
            count: 0,
        }
    }

    /// Returns the capacity.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Adds an item if there's space.
    ///
    /// # Examples
    ///
    /// ```
    /// use courses_000_example::generics_and_traits::BoundedContainer;
    ///
    /// let mut container: BoundedContainer<i32, 3> = BoundedContainer::new();
    /// assert!(container.push(1).is_ok());
    /// assert!(container.push(2).is_ok());
    /// assert!(container.push(3).is_ok());
    /// assert!(container.push(4).is_err()); // Full
    /// ```
    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.count >= N {
            return Err(item);
        }

        self.items[self.count] = Some(item);
        self.count += 1;
        Ok(())
    }

    /// Returns the current count.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Checks if the container is full.
    pub fn is_full(&self) -> bool {
        self.count >= N
    }

    /// Checks if the container is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl<T, const N: usize> Default for BoundedContainer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone, const N: usize> BoundedContainer<T, N> {
    /// Converts to a vector.
    pub fn to_vec(&self) -> Vec<T> {
        self.items
            .iter()
            .take(self.count)
            .filter_map(|item| item.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_container() {
        let mut container = Container::new();
        container.push(1);
        container.push(2);
        container.push(3);

        assert_eq!(container.len(), 3);
        assert_eq!(container.find(&2), Some(1));
        assert!(!container.contains(&4));
    }

    #[test]
    fn test_container_with_strings() {
        let container = Container::from_vec(vec![
            String::from("rust"),
            String::from("is"),
            String::from("awesome"),
        ]);

        assert_eq!(container.find(&String::from("is")), Some(1));
        assert!(container.contains(&String::from("rust")));
    }

    #[test]
    fn test_sortable_trait() {
        let mut container = Container::from_vec(vec![3, 1, 4, 1, 5]);
        container.sort();
        let sorted_items = container.to_vec();
        assert_eq!(sorted_items, vec![1, 1, 3, 4, 5]);

        let unsorted = Container::from_vec(vec![9, 2, 6]);
        let sorted = unsorted.sorted();
        assert_eq!(sorted.to_vec(), vec![2, 6, 9]);
    }

    #[test]
    fn test_product_ordering() {
        let p1 = Product {
            id: 1,
            name: "Cheap".to_string(),
            price_cents: 100,
        };
        let p2 = Product {
            id: 2,
            name: "Expensive".to_string(),
            price_cents: 500,
        };

        assert!(p1 < p2);
        assert_eq!(p1.cmp(&p2), Ordering::Less);

        let mut products = vec![p2.clone(), p1.clone()];
        products.sort();
        assert_eq!(products[0].price_cents, 100);
    }

    #[test]
    fn test_describable_trait() {
        let product = Product {
            id: 42,
            name: "Widget".to_string(),
            price_cents: 999,
        };

        assert_eq!(product.describe(), "Product: Widget (ID: 42)");
        assert_eq!(
            product.describe_detailed(),
            "Product: Widget (ID: 42) (detailed)"
        );
    }

    #[test]
    fn test_find_min_max() {
        let numbers = vec![5, 2, 8, 1, 9, 3];
        let (min, max) = find_min_max(&numbers).unwrap();
        assert_eq!(*min, 1);
        assert_eq!(*max, 9);

        let empty: Vec<i32> = vec![];
        assert!(find_min_max(&empty).is_none());
    }

    #[test]
    fn test_trait_objects() {
        struct Item {
            name: String,
        }

        impl Describable for Item {
            fn describe(&self) -> String {
                format!("Item: {}", self.name)
            }
        }

        let product = Product {
            id: 1,
            name: "Book".to_string(),
            price_cents: 1500,
        };

        let item = Item {
            name: "Pencil".to_string(),
        };

        let describables: Vec<&dyn Describable> = vec![&product, &item];
        let descriptions = describe_items(&describables);

        assert_eq!(descriptions[0], "Product: Book (ID: 1)");
        assert_eq!(descriptions[1], "Item: Pencil");
    }

    #[test]
    fn test_bounded_container() {
        let mut container: BoundedContainer<i32, 3> = BoundedContainer::new();

        assert_eq!(container.capacity(), 3);
        assert_eq!(container.len(), 0);
        assert!(!container.is_full());

        assert!(container.push(10).is_ok());
        assert!(container.push(20).is_ok());
        assert!(container.push(30).is_ok());

        assert_eq!(container.len(), 3);
        assert!(container.is_full());

        assert_eq!(container.push(40), Err(40));
        assert_eq!(container.to_vec(), vec![10, 20, 30]);
    }

    #[test]
    fn test_process_and_describe() {
        let numbers = vec![3, 1, 4, 1, 5];
        let result = process_and_describe(numbers);
        assert_eq!(result, "Sorted: [1, 1, 3, 4, 5]");
    }
}
