use crate::enums::{AccountType};
use soroban_sdk::{contracttype, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct AppUser {
    pub profile: String,
    pub account_type: AccountType,
    pub log_hash: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ConsultRequest {
    pub to_doctor: String,
    pub from_user: String,
    pub name: String,
    pub data_period: String,
    pub consult_hash: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ConsultAccepted {
    pub name: String,
    pub to_user: String,
    pub from_doctor: String,
    pub doctor_rsa: String,
    pub data_period: String,
    pub consult_hash: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ConsultData {
    pub name: String,
    pub to_doctor: String,
    pub from_user: String,
    pub user_rsa: String,
    pub data_hash: String,
    pub consult_hash: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ConsultResult {
    pub name: String,
    pub to_user: String,
    pub from_doctor: String,
    pub result_hash: String,
    pub consult_hash: String,
}