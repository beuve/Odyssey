use bimap::BiHashMap;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::comput::impacts::ImpactCategory;

#[derive(PartialEq, std::cmp::Eq, Clone, Serialize, Deserialize, Debug, Hash, EnumIter)]
pub enum EF31 {
    Gwp100,
    Acidification,
    BiogenicGwp100,
    FossilGwp100,
    ClimateChangeLandUse,
    ParticulMatter,
    EcotoxicityFreshwater,
    EcotoxicityFreshwaterInorganics,
    EcotoxicityFreshwaterOrganics,
    EutrophicationMarine,
    EutrophicationFreshwater,
    EutrophicationTerrestrial,
    HumanToxicityCarcinogenic,
    HumanToxicityCarcinogenicInorganics,
    HumanToxicityCarcinogenicOrganics,
    HumanToxicityNonCacrinogenic,
    HumanToxicityNonCacinogenicInorganics,
    HumanToxicityNonCacinogenicOrganics,
    IonisingRadiation,
    LandUse,
    OzoneDepletion,
    PhotochemicalOxidant,
    EnergyResourcesNonRenewable,
    EnergyResourcesMetalsMinerals,
    WaterUse,
}

impl EF31 {
    pub fn get_mapping() -> BiHashMap<ImpactCategory, usize> {
        let mut mapping = BiHashMap::new();
        EF31::iter().enumerate().for_each(|(i, c)| {
            let _ = mapping.insert(ImpactCategory::EF31(c), i);
        });
        mapping
    }
}
