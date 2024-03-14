# Transactions on Radix
### Composability:
Unlike traditional smart contracts, Radix transactions (manifests) can describe a series of calls to different on-ledger components.
### Atomicity:
These calls are executed atomically, meaning either all steps succeed or none do. This avoids unexpected partial execution.
### User-defined guarantees:
Wallets can add assurances about the transaction outcome, giving users more control over slippage and unexpected behavior.
### Flexibility:
Developers don't need custom code to combine calls. Manifests can be dynamically assembled with various components.
### Transparency:
Wallet software can clearly show users the steps involved in the transaction and guarantee the desired outcome.