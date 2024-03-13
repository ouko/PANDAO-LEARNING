# Resources: 
These are the building blocks for native assets on Radix. Unlike tokens in Ethereum, resources are built into the system and have guaranteed behaviors.
### Safe and predictable: 
The system enforces rules for resource creation, storage, and interaction, making them safer and more predictable to use.
### Physical analogy: 
Resources behave like real-world objects. You can place them in containers (buckets) and move them around, similar to how you handle physical items.
###  Developer control: 
While the system guarantees basic behaviors, developers can define custom rules for things like creating new resources (minting).
###  Wallet and explorer friendly: 
Since resources are defined by the system, wallets and explorers can understand their behavior without needing to read complex smart contract code. This allows for better user experience.
### Logic separation: 
Unlike Ethereum where token logic resides within smart contracts, Radix separates resource behavior from the code. This makes it more efficient and easier to understand.

##### In simpler terms, Radix offers a built-in system for creating and managing digital assets with a focus on safety, predictability, and ease of use for both developers and users.


# Disadvantages of blockchain that uses same contract for token transfer logic and token balance (eg ETH, SOLANA )
### Complexity: 
Smart contracts that manage both business logic and asset balances can become quite complex. This complexity increases the risk of bugs or vulnerabilities in the code, which can lead to security breaches or unintended behavior.

### Rigidity: 
Since the business logic is tightly coupled with the asset management code, making changes to one aspect often requires modifications to the entire smart contract. This can make upgrades or enhancements challenging and cumbersome.

### Scalability: 
The execution of smart contracts on platforms like Ethereum is limited by factors such as gas fees and block size. Complex smart contracts that handle both logic and asset management can contribute to scalability issues.

### Interoperability: 
Smart contracts that combine business logic with asset management may not be easily interoperable with other platforms or systems. This can hinder integration efforts and limit the versatility of the tokens.

