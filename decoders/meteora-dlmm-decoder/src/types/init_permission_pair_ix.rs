use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitPermissionPairIx {
    pub active_id: i32,
    pub bin_step: u16,
    pub base_factor: u16,
    pub base_fee_power_factor: u8,
    pub activation_type: u8,
    pub protocol_share: u16,
}
