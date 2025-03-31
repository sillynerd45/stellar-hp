use soroban_sdk::{contracterror, contracttype, Env, Map, String, Vec};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ContractKey {
    Admin,
    Worker,
}

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AccountType {
    User = 0,
    HealthWorker = 1,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    UserExist = 1,
    UserNotExist = 2,
    DataNotExist = 3,
    WrongAuth = 4,
}