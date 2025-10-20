use std::fs::File;

use crate::{
    comput::impacts::ImpactCategory,
    errors::Result,
    parsers::impacts::ef31::EF31Impacts,
    utils::{
        constants::DATABASES_PATH,
        matrix::{MappedMatrix, MappedMatrixBuilder},
    },
};

#[rustfmt::skip]
#[derive(Debug, serde::Deserialize)]
pub struct EcoinventEF31Impacts {
    #[serde(rename = "elementary_flow_id")]
    pub elementary_id: String,
    
    #[serde(rename = "climate change|global warming potential (GWP100)")]
    pub gwp100: Option<f64>,
    
    #[serde(rename = "acidification|accumulated exceedance (AE)")]
    pub acidification_ae: Option<f64>,
    
    #[serde(rename = "climate change: biogenic|global warming potential (GWP100)")]
    pub biogenic_gwp100: Option<f64>,
    
    #[serde(rename = "climate change: fossil|global warming potential (GWP100)")]
    pub fossil_gwp100: Option<f64>,
    
    #[serde(rename = "climate change: land use and land use change|global warming potential (GWP100)")]
    pub climate_change_land_use: Option<f64>,
    
    #[serde(rename = "particulate matter formation|impact on human health")]
    pub particul_matter: Option<f64>,
    
    #[serde(rename = "ecotoxicity: freshwater|comparative toxic unit for ecosystems (CTUe)")]
    pub ecotoxicity_freshwater: Option<f64>,
   
    #[serde(rename = "ecotoxicity: freshwater, inorganics|comparative toxic unit for ecosystems (CTUe)")]
    pub ecotoxicity_freshwater_inorganics: Option<f64>,
    
    #[serde(rename = "ecotoxicity: freshwater, organics|comparative toxic unit for ecosystems (CTUe)")]
    pub ecotoxicity_freshwater_organics: Option<f64>,
    
    #[serde(rename = "eutrophication: marine|fraction of nutrients reaching marine end compartment (N)")]
    pub eutrophication_marine: Option<f64>,
    
    #[serde(rename = "eutrophication: freshwater|fraction of nutrients reaching freshwater end compartment (P)")]
    pub eutrophication_freshwater: Option<f64>,
    
    #[serde(rename = "eutrophication: terrestrial|accumulated exceedance (AE)")]
    pub eutrophication_terrestrial: Option<f64>,
   
    #[serde(rename = "human toxicity: carcinogenic|comparative toxic unit for human (CTUh)")]
    pub human_toxicity_carcinogenic: Option<f64>,
    
    #[serde(rename = "human toxicity: carcinogenic, inorganics|comparative toxic unit for human (CTUh)")]
    pub human_toxicity_carcinogenic_inorganics: Option<f64>,
    
    #[serde(rename = "human toxicity: carcinogenic, organics|comparative toxic unit for human (CTUh)")]
    pub human_toxicity_carcinogenic_organics: Option<f64>,
    
    #[serde(rename = "human toxicity: non-carcinogenic|comparative toxic unit for human (CTUh)")]
    pub human_toxicity_non_cacrinogenic: Option<f64>,
    
    #[serde(rename = "human toxicity: non-carcinogenic, inorganics|comparative toxic unit for human (CTUh)")]
    pub human_toxicity_non_cacinogenic_inorganics: Option<f64>,
    
    #[serde(rename = "human toxicity: non-carcinogenic, organics|comparative toxic unit for human (CTUh)")]
    pub human_toxicity_non_cacinogenic_organics: Option<f64>,
    
    #[serde(rename = "ionising radiation: human health|human exposure efficiency relative to u235")]
    pub ionising_radiation: Option<f64>,
    
    #[serde(rename = "land use|soil quality index")]
    pub land_use: Option<f64>,

    #[serde(rename= "ozone depletion|ozone depletion potential (ODP)")]
    pub ozone_depletion: Option<f64>,
    
    #[serde(rename = "photochemical oxidant formation: human health|tropospheric ozone concentration increase")]
    pub photochemical_oxidant: Option<f64>,
    
    #[serde(rename = "energy resources: non-renewable|abiotic depletion potential (ADP): fossil fuels")]
    pub energy_resources_non_renewable: Option<f64>,
    
    #[serde(rename = "material resources: metals/minerals|abiotic depletion potential (ADP): elements (ultimate reserves)")]
    pub energy_resources_metals_minerals: Option<f64>,
    
    #[serde(rename = "water use|user deprivation potential (deprivation-weighted water consumption)")]
    pub water_use: Option<f64>,
}

impl From<EcoinventEF31Impacts> for EF31Impacts {
    fn from(source: EcoinventEF31Impacts) -> Self {
        EF31Impacts {
            gwp100: source.gwp100,
            acidification_ae: source.acidification_ae,
            biogenic_gwp100: source.biogenic_gwp100,
            fossil_gwp100: source.fossil_gwp100,
            climate_change_land_use: source.climate_change_land_use,
            particul_matter: source.particul_matter,
            ecotoxicity_freshwater: source.ecotoxicity_freshwater,
            ecotoxicity_freshwater_inorganics: source.ecotoxicity_freshwater_inorganics,
            ecotoxicity_freshwater_organics: source.ecotoxicity_freshwater_organics,
            eutrophication_marine: source.eutrophication_marine,
            eutrophication_freshwater: source.eutrophication_freshwater,
            eutrophication_terrestrial: source.eutrophication_terrestrial,
            human_toxicity_carcinogenic: source.human_toxicity_carcinogenic,
            human_toxicity_carcinogenic_inorganics: source.human_toxicity_carcinogenic_inorganics,
            human_toxicity_carcinogenic_organics: source.human_toxicity_carcinogenic_organics,
            human_toxicity_non_cacrinogenic: source.human_toxicity_non_cacrinogenic,
            human_toxicity_non_cacinogenic_inorganics: source
                .human_toxicity_non_cacinogenic_inorganics,
            human_toxicity_non_cacinogenic_organics: source.human_toxicity_non_cacinogenic_organics,
            ionising_radiation: source.ionising_radiation,
            land_use: source.land_use,
            ozone_depletion: source.ozone_depletion,
            photochemical_oxidant: source.photochemical_oxidant,
            energy_resources_non_renewable: source.energy_resources_non_renewable,
            energy_resources_metals_minerals: source.energy_resources_metals_minerals,
            water_use: source.water_use,
        }
    }
}

pub fn construct_impact_matrix(
    version: &str,
    intervention: &MappedMatrix<String, String>,
) -> Result<MappedMatrix<ImpactCategory, String>> {
    let file = File::open(
        DATABASES_PATH
            .join("ecoinvent_lcia")
            .join(format!("{}/methods_mapped", version))
            .join(format!("EF v3.1_mapped_{}.csv", version)),
    )?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut mat = MappedMatrixBuilder::new();
    mat.copy_rows_into_cols(intervention);
    mat.copy_vec_into_rows(&ImpactCategory::get_empty_vector());
    for result in rdr.deserialize() {
        let record: EcoinventEF31Impacts = result?;
        let elementary_id = record.elementary_id.clone();
        if intervention.contains_row(&elementary_id) {
            EF31Impacts::from(record).add_triplets(&mut mat, elementary_id);
        }
    }
    Ok(mat.build())
}
