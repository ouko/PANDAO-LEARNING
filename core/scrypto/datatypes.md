# Scrypto Data Types Summary
Scrypto, based on Rust, supports various data types for use in smart contracts (blueprints and components).

1. Primitive Types:

* Signed integers: i8, i16, ..., i128, isize (maps to i32)
* Unsigned integers: u8, u16, ..., u128, usize (maps to u32)
* Strings: String

2. Safe Types:

Wrappers around primitive types that panic on overflow (ensure transaction rejection):
* I8, I16, ..., I512
* U8, U16, ..., U512
Fixed-point decimals for precise calculations:
Decimal (192-bit, 18 decimal places)
PreciseDecimal (256-bit, 36 decimal places)

You might have noticed that there are no f32 or f64 listed in the previous section. That is because floating point arithmetic is not deterministic and does not work in distributed ledgers systems. Another technique we can use instead is fixed point arithmetic. We implemented this with the Decimal and PreciseDecimal types.

```rust
let a: Decimal = 10.into();
let b: Decimal = dec!(10);
let c: Decimal = dec!("10.333");
let d: Decimal = Decimal::from(20);
let e: Decimal = Decimal::from("20.123444");
```

3. Rust Types:

struct and enum are supported, with fields using supported data types.
Generics are not currently supported for custom structs and enums.
Enums require deriving ScryptoSbor for use in Scrypto.

4. Container Types:

* Option<T>: Optional values
* [T; N]: Arrays of fixed size
* (T, U, P, L, E): Tuples
* Vec<T>: Dynamically sized vectors
* BTreeSet<T>, BTreeMap<K, V>: B-Tree sets and maps
* HashSet<T>, HashMap<K, V>: Hash sets and maps

5. Scrypto-Specific Types:

### Blueprint and Component Types:
* PackageAddress: Address of a deployed package
* ComponentAddress: Address of a component instance
* Global<T>: Reference to a globally accessible object
* Globalizing<T>: Local component to be made global
* Attached<T>: Module attached to a global object
* Owned<T>: Locally owned component
* KeyValueStore: Lookup table for storing and retrieving data
* Cryptography Types:
* Hash: 32-byte hash digest (SHA256 currently supported)
* Secp256k1PublicKey: Public key for ECDSA (secp256k1 curve)
* Secp256k1PrivateKey: Private key for ECDSA (secp256k1 curve)
* Math Types:
* Decimal (described above)
* PreciseDecimal (described above)
* Resource Types:
* Bucket: Represents a collection of resources (fungible or non-fungible)
* FungibleBucket: Bucket for fungible resources only
* NonFungibleBucket: Bucket for non-fungible resources only
* Proof: Proof of ownership for a resource (fungible or non-fungible)
* FungibleProof, NonFungibleProof: Proofs for specific resource types
* CheckedProof: Proof validated at the application layer
* Vault: Represents a storage location for resources
* FungibleVault, NonFungibleVault: Vaults for specific resource types
* NonFungibleGlobalId, NonFungibleLocalId: Identifiers for non-fungible resources
* ResourceAddress: Address of a resource