# SPEC-001: Radix Blockchain DAO Launcher

## Table of Contents

1. [Background](#background)
2. [Requirements](#requirements)
   - [Must Have](#must-have)
   - [Should Have](#should-have)
   - [Could Have](#could-have)
   - [Won't Have for now](#wont-have-for-now)
3. [Method](#method)
   - [Architecture Design](#architecture-design)
   - [Blockchain Integration](#blockchain-integration)
   - [Data Storage Strategy](#data-storage-strategy)
   - [Database Schema](#database-schema)
   - [User Authentication and Wallet Management](#user-authentication-and-wallet-management)

## Background

The purpose of the Radix Blockchain DAO Launcher website is to democratize access to blockchain technology, particularly for individuals new to the blockchain space who may not have extensive technical experience. The Radix blockchain's unique offering of modular smart contracts, or components, provides an ideal foundation for this project. By leveraging these components, the website will offer multiple DAO blueprints from which users can choose, simplifying the process of creating and managing Decentralized Autonomous Organizations (DAOs). This initiative is inspired by the gap in the current market for accessible blockchain-based tools, aiming to lower the entry barrier for creating and participating in DAOs.

## Requirements

### Must Have

- User Interface (UI) for selecting DAO blueprints from a curated list.
- Functionality for users to launch their own DAO component based on the selected blueprint.
- Mechanism for inviting members to the DAO and managing their roles.
- Voting system within each DAO to facilitate democratic decision-making processes.
- Simple login system and wallet connection for Radix blockchain interaction.

### Should Have

- Customizable DAO governance models to provide flexibility in DAO management (post-MVP).

### Could Have

- Integration with Google or other OAuth providers for authentication.
- Automated creation and deployment of Radix wallets for users, enhancing the ease of entry into the Radix ecosystem.

### Won't Have for now

- Advanced customization options for DAO blueprints in the MVP version.
- Complex identity verification processes beyond basic OAuth integrations.

## Method

### Architecture Design

The system will be built using a **Microservices Architecture** to ensure scalability and ease of updates. This architecture facilitates the development and maintenance of the platform by allowing individual services to be updated, deployed, and scaled independently. Key services will include:

- **User Service**
- **DAO Service**
- **Voting Service**
- **Blockchain Interaction Service**
- **Wallet Service**

### Blockchain Integration

The **Blockchain Interaction Service** will use a middle layer approach to interact with the Radix blockchain. This service will abstract the complexity of blockchain interactions, providing a simplified interface for other services, handle the deployment of DAO components, and verify on-chain data to ensure integrity and authenticity.

### Data Storage Strategy

A **Hybrid Data Storage Strategy** will be employed, combining off-chain and on-chain storage:

- **Off-Chain Storage**: For quick access and manipulation (e.g., user profiles, DAO blueprints, memberships).
- **On-Chain Verification**: For security and immutability (e.g., DAO governance rules, transaction records).

### Database Schema

A relational database like **PostgreSQL** will be used, with schemas for:

- **Users**
- **DAOs**
- **Memberships**
- **Votes**

### User Authentication and Wallet Management

- **OAuth Integration**: Initially supporting Google, with plans to expand.
- **Wallet Management**: Users will have full control over their Radix wallets.

This document outlines the foundational aspects of the Radix Blockchain DAO Launcher project, setting the stage for the development of a scalable and user-friendly platform.

