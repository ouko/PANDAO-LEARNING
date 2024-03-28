# Fungible Resources
### A quantity of fungible resources can be freely split into smaller quantities, and smaller quantities can be recombined. Typical tokens (including the XRD token), where no two tokens have an individual identity, are created as fungible resources. Example uses of fungible resources (but not limited to) include:
* Utility or governance tokens
* Stablecoins
* Fractionalized shares
* Liquidity provider tokens
* Tokenized representations of commodities
* Authorization badges 


# NonFungible Resources
### With non-fungible resources, each individual resource unit is uniquely addressable and not divisible. You can think of a non-fungible resource as a grouped set of individual tokens that have the same behavior, but where each unit is a standalone token with its own identity (and its own unique associated data). Here are some examples of non-fungibles use cases:
* Tickets to an event with seat numbers
* Representations of unique numbered documents or products
* Deeds of ownership of property or other real assets
* Transactable unique debt positions or derivatives



Imagine resources on the Radix network like valuable items. They need to be stored securely and tracked whenever they are moved around. This is where resource containers come in. They act like specialized boxes for your resources, ensuring everything is organized and accounted for.

Here's a breakdown of the two types of resource containers on Radix:

# Buckets: The Temporary Holding Pens

Think of buckets as temporary storage for resources during a transaction. Like a box you use while rearranging furniture, buckets hold resources only for the duration of the transaction.
Once the transaction is complete, the resources cannot stay in the bucket. They need to be moved to a more permanent location.
Vaults: The Permanent Homes

# Vaults are like secure safes for your resources. They are permanent containers designed for long-term storage.
Every resource on Radix must ultimately reside in a vault at the end of each transaction. This ensures all resources are properly accounted for and never get lost in the shuffle.
Why Separate Containers?

# Security: Vaults provide a more secure environment for valuable resources compared to temporary buckets.
* Organization: Separating temporary movement (buckets) from permanent storage (vaults) keeps things organized and simplifies tracking resource movement.
* Transaction Efficiency: Since buckets only exist within a transaction, they help streamline the transaction process.
Analogy in Action:

Imagine you're sending a digital painting (a resource) to a friend (another account) on Radix.

You first place the painting in a bucket (temporary container) for the transfer.
The bucket with the painting is then moved within the transaction to your friend's account.
Once the transaction is complete, the painting cannot stay in the bucket. It needs to be placed in a permanent vault within your friend's account for safekeeping.
By using buckets and vaults, Radix ensures secure, organized, and efficient management of resources on its network.