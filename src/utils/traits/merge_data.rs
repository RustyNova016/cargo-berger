use core::fmt::Debug;
use core::hash::Hash;
use std::collections::HashMap;
use std::path::PathBuf;

use itertools::Itertools;

/// Merge the data between two structs.
///
/// This overwrites existing fields, but not optional fields
pub trait OverwriteMergeData {
    fn merge_data_mut(&mut self, other: Self);

    fn merge_data(mut self, other: Self) -> Self
    where
        Self: Sized,
    {
        self.merge_data_mut(other);
        self
    }
}

impl OverwriteMergeData for bool {
    fn merge_data_mut(&mut self, other: Self) {
        *self = other
    }
}

impl OverwriteMergeData for String {
    fn merge_data_mut(&mut self, other: Self) {
        *self = other
    }
}

impl<T> OverwriteMergeData for Option<T>
where
    T: OverwriteMergeData + Debug,
{
    #[inline]
    fn merge_data_mut(&mut self, other: Self) {
        if let Some(val) = other {
            *self = Some(val);
        };
    }
}

impl OverwriteMergeData for PathBuf {
    fn merge_data_mut(&mut self, other: Self) {
        *self = other
    }
}

impl<K, V> OverwriteMergeData for HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: OverwriteMergeData,
{
    fn merge_data_mut(&mut self, mut other: Self) {
        let keys = self
            .keys()
            .chain(other.keys())
            .unique()
            .cloned()
            .collect_vec();

        for key in keys {
            let a = self.get_mut(&key);
            let Some(b) = other.remove(&key) else {
                continue;
            };

            if let Some(a) = a {
                a.merge_data_mut(b);
                continue;
            }

            self.insert(key, b);
        }
    }
}
