use bimap::BiHashMap;
use std::ops::{Add, AddAssign};
use std::{hash::Hash, sync::Arc};

use crate::utils::matrix::cs::Cs;
use crate::utils::matrix::MappedMatrix;

/// 2D matrix with `String` values maped to each rows and columns.
#[derive(Debug, Clone)]
pub struct MappedVector<T>
where
    T: std::cmp::Eq + Hash + Clone,
{
    pub mapping: Arc<BiHashMap<T, usize>>,
    pub values: Vec<f64>,
}

impl<T> MappedVector<T>
where
    T: std::cmp::Eq + Hash + Clone,
{
    pub fn empty() -> Self {
        Self {
            mapping: Arc::new(BiHashMap::new()),
            values: vec![],
        }
    }

    pub fn new(mapping: Arc<BiHashMap<T, usize>>, values: Vec<f64>) -> Self {
        Self { mapping, values }
    }

    /// Modify the value associated with the given label.
    /// If a value already existed, it is overwritten and returned.
    /// TODO: Should return a result
    pub fn set(&mut self, label: T, value: f64) -> Option<f64> {
        if self.contains(&label) {
            let index = self.mapping.get_by_left(&label).unwrap();
            let res = self.values[*index];
            self.values[*index] += value;
            Some(res)
        } else {
            None
        }
    }

    /// Returns the index of the row corresponding to `id`.
    /// This may fail if `id` has no corresponding row.
    pub fn row(&self, id: &T) -> Option<&usize> {
        self.mapping.get_by_left(id)
    }

    /// Returns the value mapped to the row `index`.
    /// This may fail if `index` is out of range, or
    /// rare cases, if the row was not mapped to any value
    /// (although this should not happen).
    pub fn irow(&self, index: &usize) -> Option<&T> {
        self.mapping.get_by_right(index)
    }

    /// Number of elements in the mapped matrix.
    pub fn nrows(&self) -> usize {
        self.mapping.len()
    }

    /// Test weither any row was mapped to `id`.
    pub fn contains(&self, id: &T) -> bool {
        self.mapping.contains_left(id)
    }

    /// Returns the index of the row corresponding to `id`.
    /// This may fail if `id` has no corresponding row.
    pub fn map(&self, id: &T) -> Option<&usize> {
        self.mapping.get_by_left(id)
    }

    pub fn diag(&self) -> MappedMatrix<T, T> {
        let n = self.nrows();
        let cs = Cs::new(
            n,
            n,
            (0i32..=n as i32).collect(),
            (0i32..n as i32).collect(),
            self.values.clone(),
        );
        MappedMatrix::new(self.mapping.clone(), self.mapping.clone(), cs, None, None)
    }
}

#[macro_export]
macro_rules! MV {
    (
      $(
          $label:expr => $val:expr
      ),* $(,)?
    ) => {{
        let mut vector = MappedVector::new();
        $(
            vector.add($label, $val);
        )*
        vector
    }};
}

impl<T> Add for MappedVector<T>
where
    T: std::cmp::Eq + Hash + Clone,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let values = self
            .values
            .iter()
            .zip(other.values)
            .map(|(v1, v2)| v1 + v2)
            .collect();
        // TODO: Verify that mappings are the same (DEBUG only)
        Self {
            mapping: self.mapping.clone(),
            values,
        }
    }
}

impl<T> AddAssign for MappedVector<T>
where
    T: std::cmp::Eq + Hash + Clone,
{
    fn add_assign(&mut self, rhs: Self) {
        for (l, r) in self.values.iter_mut().zip(rhs.values) {
            *l += r;
        }
    }
}
