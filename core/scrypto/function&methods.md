Functions and Methods in Scrypto Blueprints
Functions and methods are essential building blocks for Scrypto blueprints, defining how components behave and interact with the network.

1. Functions:

Similar to static functions in other languages.
Don't have internal state and can be called directly on a blueprint.
Often used for component instantiation.
Can have input parameters and return values, but no reference to self.
Example:

```Rust
pub fn instantiate_my_component(init_count: u32) -> Global<Component> {
  // ... (component instantiation logic)
}
```
Use code with caution.
2. Methods:

Require a reference to self.
Can read and modify a component's internal state.
Only callable on instantiated components (blueprints don't have state).
There are two types of method references:

&self: Statements can only read the component state.
&mut self: Statements can read and write the component state.
Example Method Signatures:

```rust
pub fn get_value(&self) -> u32 { ... } (read-only)
pub fn update_value(&mut self, new_value: u32) { ... } (read-write)
```
3. Visibility:

Public functions and methods (preceded by pub) are callable by external entities (transactions, other functions/methods).
Methods are only invokable on instantiated components.
Checking Available Functions/Methods:

Bash
resim export-abi <PACKAGE_ADDRESS> <BLUEPRINT_NAME>
Use code with caution.
4. Private Functions/Methods:

Functions/methods marked with use scrypto::prelude::*; are only callable from within the blueprint.
Remove the pub keyword to make a function/method private.
Example:

```Rust
pub fn free_gumball(&mut self) -> Bucket {
  self.print_vault_info();  // Call to private method
  self.gumballs.take(1)
}

fn print_vault_info(&self) {
  // ... (private logic)
}
```

In summary, functions handle general tasks without internal state, while methods manage a component's state and require an instance to be called upon. Understanding these concepts is crucial for building functional Scrypto blueprints.