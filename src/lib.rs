use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UNAgencyState {
    pub agency_id: u32,
    pub agency_name: String,
    pub project_name: String,
    pub portfolio: String,
    pub strategy: String,
    pub target_chain: String,
    pub is_active: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MultilateralAuditRecord {
    pub audit_id: String,
    pub timestamp: u64,
    pub merkle_root_hash: String,
    pub verified_by_pqc: bool,
}

pub fn get_metadata() -> UNAgencyState {
    UNAgencyState {
        agency_id: 2,
        agency_name: "Food and Agriculture Organization (FAO)".to_string(),
        project_name: "FAO AgroMesh AI".to_string(),
        portfolio: "Global Food Security & Agriculture".to_string(),
        strategy: "Programmatic Utility ($FOOD) -- Global Food Reserve Settlement and Humanitarian Procurement".to_string(),
        target_chain: "Internet Computer (ICP) + Solana".to_string(),
        is_active: true,
    }
}
