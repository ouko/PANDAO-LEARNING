use scrypto::prelude::*;

// in this code
// we're going to define a blue print for cat currency , which is nothing but fungible tokens
#[blueprint]
mod hello {
    struct AnimalDigitalCurrency {
        // as mentioned in core<docs> permanent values/state of the contract must always stored in Vault
        // buckets are temp resources holders and used to pass resoucres from users/components to components
        currency: Vault,
    }

    impl AnimalDigitalCurrency {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn instantiate_currency(name:String,symbol:String) -> Global<AnimalDigitalCurrency> {
            // Create a new token called "HelloToken," with a fixed supply of 1000, and put that supply into a bucket
            // owner role define the owner who can alter resource meta data even after component is created
            // divisibility() defines the divsiblity of resource , by default ( if not specified ) it is max
         let bucket = ResourceBuilder::new_fungible(OwnerRole::None)
         .divisibility(DIVISIBILITY_MAXIMUM)
         .metadata(metadata! (
            init {
                "name" => name, locked;
                "symbol" => symbol, locked;
            }
        ))
         .mint_initial_supply(100) ;

            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            Self {
                currency: Vault::with_bucket(bucket.into()),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        // This is a method, because it needs a reference to self.  Methods can only be called on components
        pub fn free_token(&mut self) -> Bucket {
            info!(
                "My balance is: {} HelloToken. Now giving away a token!",
                self.currency.amount()
            );
            // If the semi-colon is omitted on the last line, the last value seen is automatically returned
            // In this case, a bucket containing 1 HelloToken is returned
            self.currency.take(1)
        }
    }
}

 
