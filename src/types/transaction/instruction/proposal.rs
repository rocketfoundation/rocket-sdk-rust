use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ProposalType {
    CreateInstrument,
    CreateValidator,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum ProposalDataType {
    CreateInstrument(CreateInstrumentData),
    CreateValidator(CreateValidatorData),
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CreateProposalData {
    pub proposal_type: ProposalType,
    pub proposal_data: ProposalDataType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CreateInstrumentData {
    pub instrument_name: String,
    pub instrument_symbol: String,
    pub instrument_description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CreateValidatorData {
    pub validator_name: String,
    pub validator_description: String,
}
