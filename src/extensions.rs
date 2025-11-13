//! Type-safe request storage.

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Store arbitrary data by type.
#[derive(Default)]
pub struct Extensions {
    map: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Extensions {
    /// Create with pre-allocated capacity (4).
    #[inline]
    pub fn new() -> Self {
        Self {
            map: HashMap::with_capacity(4),
        }
    }

    /// Create with custom capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
        }
    }

    /// Insert value (replaces existing).
    #[inline]
    pub fn insert<T: Send + Sync + 'static>(&mut self, value: T) -> Option<T> {
        self.map
            .insert(TypeId::of::<T>(), Box::new(value))
            .and_then(|boxed| boxed.downcast::<T>().ok())
            .map(|boxed| *boxed)
    }

    /// Get reference to value.
    #[inline]
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    /// Get mutable reference to value.
    #[inline]
    pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }

    /// Remove value.
    #[inline]
    pub fn remove<T: Send + Sync + 'static>(&mut self) -> Option<T> {
        self.map
            .remove(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast::<T>().ok())
            .map(|boxed| *boxed)
    }

    /// Check if value exists.
    #[inline]
    pub fn contains<T: Send + Sync + 'static>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }

    /// Clear all values.
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Get count.
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Check if empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl std::fmt::Debug for Extensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Extensions")
            .field("count", &self.map.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut ext = Extensions::new();
        ext.insert(42u32);
        ext.insert("hello".to_string());

        assert_eq!(ext.get::<u32>(), Some(&42));
        assert_eq!(ext.get::<String>().map(|s| s.as_str()), Some("hello"));
    }

    #[test]
    fn test_capacity() {
        let ext = Extensions::with_capacity(10);
        assert!(ext.map.capacity() >= 10);
    }

    #[test]
    fn test_remove() {
        let mut ext = Extensions::new();
        ext.insert(100u64);

        assert_eq!(ext.remove::<u64>(), Some(100));
        assert_eq!(ext.get::<u64>(), None);
    }

    #[test]
    fn test_contains() {
        let mut ext = Extensions::new();
        ext.insert(true);

        assert!(ext.contains::<bool>());
        assert!(!ext.contains::<u32>());
    }
}
