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
    T: OverwriteMergeData,
{
    #[inline]
    fn merge_data_mut(&mut self, other: Self) {
        if let Some(val) = other {
            *self = Some(val);
        };
    }
}
