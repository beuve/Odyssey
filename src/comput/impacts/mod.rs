use std::{hash::Hash, sync::Arc};

use bimap::BiHashMap;
use serde::{Deserialize, Serialize};

pub mod ef31;

pub use ef31::EF31;

use crate::utils::matrix::MappedVector;

#[derive(PartialEq, std::cmp::Eq, Clone, Serialize, Deserialize, Debug, Hash)]
pub enum ImpactCategory {
    EF31(EF31),
}

impl ImpactCategory {
    pub fn get_empty_vector() -> MappedVector<ImpactCategory> {
        let mut mappings = BiHashMap::new();
        mappings.extend(EF31::get_mapping());
        let length = mappings.len();
        MappedVector::new(Arc::new(mappings), vec![0.; length])
    }
}
