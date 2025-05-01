#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, log};

#[contracttype]
pub struct ResumeEntry {
    pub title: String,
    pub organization: String,
    pub description: String,
    pub year: u32,
}

#[contracttype]
pub enum ResumeKey {
    Entries(Address),
}

#[contract]
pub struct ResumeContract;

#[contractimpl]
impl ResumeContract {
    // Add a resume entry (like a job or education record)
    pub fn add_entry(
        env: Env,
        user: Address,
        title: String,
        organization: String,
        description: String,
        year: u32,
    ) {
        user.require_auth();

        let key = ResumeKey::Entries(user.clone());
        let mut resume: Vec<ResumeEntry> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        let entry = ResumeEntry {
            title,
            organization,
            description,
            year,
        };

        resume.push_back(entry);
        env.storage().persistent().set(&key, &resume);

        log!(&env, "Resume updated for user: {}", user);
    }

    // Get a user's full resume
    pub fn get_resume(env: Env, user: Address) -> Vec<ResumeEntry> {
        let key = ResumeKey::Entries(user);
        env.storage().persistent().get(&key).unwrap_or(Vec::new(&env))
    }
}
