use crate::{
    comput::impacts::{ImpactCategory, EF31},
    utils::matrix::MappedMatrixBuilder,
};

#[rustfmt::skip]
#[derive(Debug, serde::Deserialize)]
pub struct EF31Impacts {
    pub gwp100: Option<f64>,
    pub acidification_ae: Option<f64>,
    pub biogenic_gwp100: Option<f64>,
    pub fossil_gwp100: Option<f64>,
    pub climate_change_land_use: Option<f64>,
    pub particul_matter: Option<f64>,
    pub ecotoxicity_freshwater: Option<f64>,
    pub ecotoxicity_freshwater_inorganics: Option<f64>,
    pub ecotoxicity_freshwater_organics: Option<f64>,
    pub eutrophication_marine: Option<f64>,
    pub eutrophication_freshwater: Option<f64>,
    pub eutrophication_terrestrial: Option<f64>,
    pub human_toxicity_carcinogenic: Option<f64>,
    pub human_toxicity_carcinogenic_inorganics: Option<f64>,
    pub human_toxicity_carcinogenic_organics: Option<f64>,
    pub human_toxicity_non_cacrinogenic: Option<f64>,
    pub human_toxicity_non_cacinogenic_inorganics: Option<f64>,
    pub human_toxicity_non_cacinogenic_organics: Option<f64>,
    pub ionising_radiation: Option<f64>,
    pub land_use: Option<f64>,
    pub ozone_depletion: Option<f64>,
    pub photochemical_oxidant: Option<f64>,
    pub energy_resources_non_renewable: Option<f64>,
    pub energy_resources_metals_minerals: Option<f64>,
    pub water_use: Option<f64>,
}

#[rustfmt::skip]
impl EF31Impacts {

    /// Order is important
    pub fn add_triplets(&self, a: &mut MappedMatrixBuilder<ImpactCategory, String>, col: String) {
        a.add_triplet(ImpactCategory::EF31(EF31::Gwp100), col.clone(), self.gwp100.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::Acidification), col.clone(), self.acidification_ae.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::BiogenicGwp100), col.clone(), self.biogenic_gwp100.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::FossilGwp100), col.clone(), self.fossil_gwp100.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::ClimateChangeLandUse), col.clone(), self.climate_change_land_use.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::OzoneDepletion), col.clone(), self.ozone_depletion.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::ParticulMatter), col.clone(), self.particul_matter.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::EcotoxicityFreshwater), col.clone(), self.ecotoxicity_freshwater.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::EcotoxicityFreshwaterInorganics), col.clone(), self.ecotoxicity_freshwater_inorganics.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::EcotoxicityFreshwaterOrganics), col.clone(), self.ecotoxicity_freshwater_organics.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::EutrophicationMarine), col.clone(), self.eutrophication_marine.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::EutrophicationFreshwater), col.clone(), self.eutrophication_freshwater.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::EutrophicationTerrestrial), col.clone(), self.eutrophication_terrestrial.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::HumanToxicityCarcinogenic), col.clone(), self.human_toxicity_carcinogenic.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::HumanToxicityCarcinogenicInorganics), col.clone(), self.human_toxicity_carcinogenic_inorganics.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::HumanToxicityCarcinogenicOrganics), col.clone(), self.human_toxicity_carcinogenic_organics.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::HumanToxicityNonCacrinogenic), col.clone(), self.human_toxicity_non_cacrinogenic.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::HumanToxicityNonCacinogenicInorganics), col.clone(), self.human_toxicity_non_cacinogenic_inorganics.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::HumanToxicityNonCacinogenicOrganics), col.clone(), self.human_toxicity_non_cacinogenic_organics.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::IonisingRadiation), col.clone(), self.ionising_radiation.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::LandUse), col.clone(), self.land_use.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::PhotochemicalOxidant), col.clone(), self.photochemical_oxidant.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::EnergyResourcesNonRenewable), col.clone(), self.energy_resources_non_renewable.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::EnergyResourcesMetalsMinerals), col.clone(), self.energy_resources_metals_minerals.unwrap_or(0.));
        a.add_triplet(ImpactCategory::EF31(EF31::WaterUse), col.clone(), self.water_use.unwrap_or(0.));
    }
}
