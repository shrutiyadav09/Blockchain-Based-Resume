# Blockchain-Based Resume

## Table of Contents
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Contract Details](#contract-details)

## Project Title

**Blockchain-Based Resume**

## Project Description

A Soroban-based smart contract that allows users to create verifiable resumes on the blockchain. Users can record work experience, education, and accomplishments immutably and share them with employers or networks confidently.

## Project Vision

To empower individuals with a trusted, tamper-proof resume system that ensures authenticity, ownership, and transparency — free from centralized gatekeeping.

## Key Features

- ✅ **Add Resume Entries**: Authenticated users can log new experience records (e.g., job, education).
- 📄 **View Resume**: Anyone can retrieve a user’s public resume entries.
- 🔒 **Verified Ownership**: Only the resume owner can modify their information.
- 🌍 **Global Portability**: A decentralized resume accessible from anywhere.

## Contract Details

### Contract Address: CC745G2WSR4AKEEIKLKQ3JGDRIYXXYSRPMJUPRMOKP3I2NSJOBKUHZ2M

### 1. `add_entry(user, title, organization, description, year)`
Stores a new resume entry for the user.

### 2. `get_resume(user) -> Vec<ResumeEntry>`
Returns all entries in the user's resume.

Each `ResumeEntry` includes:
- `title`: e.g., Software Engineer
- `organization`: e.g., OpenAI
- `description`: Summary of responsibilities or achievements
- `year`: Year of the experience

---

**Build your career with confidence — on-chain, for life.**  
Powered by [Soroban](https://soroban.stellar.org) for transparent digital identity.
