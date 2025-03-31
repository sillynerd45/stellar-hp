#![no_std]
use crate::custom_types::{AppUser, ConsultAccepted, ConsultData, ConsultRequest, ConsultResult};
use crate::enums::*;
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, BytesN, Env, Map, String, Vec};
use crate::constants::ONE_MONTH_TTL;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(env: Env, stellar_hp_admin: Address) {
        env.storage().instance().set(&ContractKey::Admin, &stellar_hp_admin);
    }

    pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        let admin: Address = env.storage().instance().get(&ContractKey::Admin).unwrap();
        admin.require_auth();

        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }

    pub fn sign_up(
        env: Env,
        user: Address,
        profile: String,
        account_type: AccountType,
        log_hash: String,
        worker_name: String,
    ) -> Result<u32, Error> {
        user.require_auth();

        if env.storage().persistent().has(&user) {
            return Err(Error::UserExist);
        }

        let app_user = AppUser { profile, account_type, log_hash: log_hash.clone() };
        env.storage().persistent().set(&user, &app_user);

        if account_type.eq(&AccountType::HealthWorker) {
            let mut worker_list: Map<String, String> = env.storage().temporary().get(&ContractKey::Worker).unwrap_or(Map::new(&env));
            worker_list.set(log_hash, worker_name);

            env.storage().temporary().set(&ContractKey::Worker, &worker_list);
            env.storage().temporary().extend_ttl(&ContractKey::Worker, ONE_MONTH_TTL, ONE_MONTH_TTL);
        }

        Ok(0)
    }

    pub fn get_log_hash(env: Env, user: Address) -> Result<String, Error> {
        if !env.storage().persistent().has(&user) {
            return Err(Error::UserNotExist);
        }

        let app_user: AppUser = env.storage().persistent().get(&user).unwrap();
        Ok(app_user.log_hash)
    }

    pub fn get_profile(env: Env, user: Address) -> Result<String, Error> {
        if !env.storage().persistent().has(&user) {
            return Err(Error::UserNotExist);
        }

        let app_user: AppUser = env.storage().persistent().get(&user).unwrap();
        Ok(app_user.profile)
    }

    pub fn get_health_workers(env: Env) -> Result<Map<String, String>, Error> {
        if !env.storage().temporary().has(&ContractKey::Worker) {
            return Err(Error::DataNotExist);
        }

        Ok(env.storage().temporary().get(&ContractKey::Worker).unwrap_or(Map::new(&env)))
    }

    pub fn insert_log(
        env: Env,
        user: Address,
        year: u32,
        month: u32,
        date: u32,
        log_value: String,
        year_hash: String,
        month_hash: String,
        date_hash: String,
    ) -> Result<u32, Error> {
        user.require_auth();

        if !env.storage().persistent().has(&user) {
            return Err(Error::UserNotExist);
        }

        let app_user: AppUser = env.storage().persistent().get(&user).unwrap();

        // load log_data or create empty map
        let mut log_data: Map<u32, String> = env.storage().temporary().get(&app_user.log_hash).unwrap_or(Map::new(&env));
        let year_hash_exist = log_data.contains_key(year);
        if !year_hash_exist {
            create_year_entry(&env, year, month, date, year_hash, month_hash, date_hash, app_user.log_hash, &mut log_data, log_value);
            return Ok(0);
        }

        // load year_data or create empty map
        let mut year_data: Map<u32, String> = env.storage().temporary().get(&log_data.get_unchecked(year)).unwrap_or(Map::new(&env));
        let month_hash_exist = year_data.contains_key(month);
        if !month_hash_exist {
            let current_year_hash = log_data.get_unchecked(year);
            create_month_entry(&env, month, date, current_year_hash, month_hash, date_hash, &mut year_data, log_value);
            return Ok(0);
        }

        // load month_data or create empty map
        let mut month_data: Map<u32, String> = env.storage().temporary().get(&year_data.get_unchecked(month)).unwrap_or(Map::new(&env));
        let date_hash_exist = month_data.contains_key(date);
        if !date_hash_exist {
            let current_month_hash = year_data.get_unchecked(month);
            create_date_entry(&env, date, current_month_hash, date_hash, &mut month_data, log_value);
            return Ok(0);
        }

        // load date_data
        let current_entry_hash = month_data.get_unchecked(date);
        create_log_entry(&env, current_entry_hash, log_value);

        Ok(0)
    }

    pub fn read_all_log(
        env: Env,
        user: Address,
    ) -> Result<Vec<String>, Error> {
        if !env.storage().persistent().has(&user) {
            return Err(Error::UserNotExist);
        }

        let mut logs_data: Vec<String> = Vec::new(&env);
        let app_user: AppUser = env.storage().persistent().get(&user).unwrap();

        // load log_data or create empty map
        let log_data: Map<u32, String> = env.storage().temporary().get(&app_user.log_hash).unwrap_or(Map::new(&env));

        // iterate over each year_hash available in log_data
        for year_hash in log_data.values() {
            // get year_data and iterate over each month_hash available
            let year_data: Map<u32, String> = env.storage().temporary().get(&year_hash).unwrap_or(Map::new(&env));
            for month_hash in year_data.values() {
                // get month_data and iterate over each day_hash available
                let month_data: Map<u32, String> = env.storage().temporary().get(&month_hash).unwrap_or(Map::new(&env));
                for date_hash in month_data.values() {
                    // get date_data and push to logs_data
                    let date_data: String = env.storage().temporary().get(&date_hash).unwrap_or(String::from_str(&env, ""));
                    logs_data.push_back(date_data);
                }
            }
        }

        Ok(logs_data)
    }

    pub fn consult_request(
        env: Env,
        user: Address,
        to_doctor: String,
        from_user: String,
        name: String,
        data_period: String,
        consult_hash: String,
    ) -> Result<u32, Error> {
        user.require_auth();

        if !env.storage().persistent().has(&user) {
            return Err(Error::UserNotExist);
        }

        let app_user: AppUser = env.storage().persistent().get(&user).unwrap();
        if app_user.log_hash.ne(&from_user) {
            return Err(Error::WrongAuth);
        }

        let send_consult_request = ConsultRequest { to_doctor, from_user, name, data_period, consult_hash };
        env.events().publish((symbol_short!("request"),), send_consult_request);

        Ok(0)
    }

    pub fn consult_accepted(
        env: Env,
        doctor: Address,
        name: String,
        to_user: String,
        from_doctor: String,
        doctor_rsa: String,
        data_period: String,
        consult_hash: String,
    ) -> Result<u32, Error> {
        doctor.require_auth();

        if !env.storage().persistent().has(&doctor) {
            return Err(Error::UserNotExist);
        }

        let app_user: AppUser = env.storage().persistent().get(&doctor).unwrap();
        if app_user.log_hash.ne(&from_doctor) {
            return Err(Error::WrongAuth);
        }

        let send_consult_accepted = ConsultAccepted { name, to_user, from_doctor, doctor_rsa, data_period, consult_hash };
        env.events().publish((symbol_short!("accepted"),), send_consult_accepted);

        Ok(0)
    }

    pub fn consult_data(
        env: Env,
        user: Address,
        name: String,
        to_doctor: String,
        from_user: String,
        user_rsa: String,
        data_hash: String,
        data: String,
        consult_hash: String,
    ) -> Result<u32, Error> {
        user.require_auth();

        if !env.storage().persistent().has(&user) {
            return Err(Error::UserNotExist);
        }

        let app_user: AppUser = env.storage().persistent().get(&user).unwrap();
        if app_user.log_hash.ne(&from_user) {
            return Err(Error::WrongAuth);
        }

        env.storage().temporary().set(&data_hash, &data);

        let send_consult_data = ConsultData { name, to_doctor, from_user, user_rsa, data_hash, consult_hash };
        env.events().publish((symbol_short!("data"),), send_consult_data);

        Ok(0)
    }

    pub fn consult_result(
        env: Env,
        doctor: Address,
        name: String,
        to_user: String,
        from_doctor: String,
        result_hash: String,
        result_data: String,
        consult_hash: String,
    ) -> Result<u32, Error> {
        doctor.require_auth();

        if !env.storage().persistent().has(&doctor) {
            return Err(Error::UserNotExist);
        }

        let app_user: AppUser = env.storage().persistent().get(&doctor).unwrap();
        if app_user.log_hash.ne(&from_doctor) {
            return Err(Error::WrongAuth);
        }

        env.storage().temporary().set(&result_hash, &result_data);

        let send_consult_data = ConsultResult { name, to_user, from_doctor, result_hash, consult_hash };
        env.events().publish((symbol_short!("result"),), send_consult_data);

        Ok(0)
    }

    pub fn get_single_log(
        env: Env,
        data_hash: String,
    ) -> Result<String, Error> {
        if !env.storage().temporary().has(&data_hash) {
            return Err(Error::DataNotExist);
        }
        Ok(env.storage().temporary().get(&data_hash).unwrap())
    }
}

pub fn create_year_entry(
    env: &Env,
    year: u32,
    month: u32,
    date: u32,
    year_hash: String,
    month_hash: String,
    date_hash: String,
    log_hash: String,
    log_data: &mut Map<u32, String>,
    log_value: String,
) {
    log_data.set(year, year_hash.clone());
    env.storage().temporary().set(&log_hash, log_data);
    env.storage().temporary().extend_ttl(&log_hash, ONE_MONTH_TTL, ONE_MONTH_TTL);

    let mut year_data = Map::new(&env);
    create_month_entry(&env, month, date, year_hash, month_hash, date_hash, &mut year_data, log_value);
}

pub fn create_month_entry(
    env: &Env,
    month: u32,
    date: u32,
    year_hash: String,
    month_hash: String,
    date_hash: String,
    year_data: &mut Map<u32, String>,
    log_value: String,
) {
    year_data.set(month, month_hash.clone());
    env.storage().temporary().set(&year_hash, year_data);
    env.storage().temporary().extend_ttl(&year_hash, ONE_MONTH_TTL, ONE_MONTH_TTL);

    let mut month_data = Map::new(&env);
    create_date_entry(&env, date, month_hash, date_hash, &mut month_data, log_value);
}

pub fn create_date_entry(
    env: &Env,
    date: u32,
    month_hash: String,
    date_hash: String,
    month_data: &mut Map<u32, String>,
    log_value: String,
) {
    month_data.set(date, date_hash.clone());
    env.storage().temporary().set(&month_hash, month_data);
    env.storage().temporary().extend_ttl(&month_hash, ONE_MONTH_TTL, ONE_MONTH_TTL);

    create_log_entry(&env, date_hash, log_value);
}

pub fn create_log_entry(
    env: &Env,
    date_hash: String,
    log_value: String,
) {
    env.storage().temporary().set(&date_hash, &log_value);
    env.storage().temporary().extend_ttl(&date_hash, ONE_MONTH_TTL, ONE_MONTH_TTL);
}

mod custom_types;
mod enums;
mod test;
mod constants;


