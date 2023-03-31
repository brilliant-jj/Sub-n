use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result};
use system::ensure_signed;
use codec::{Encode, Decode};
use runtime_primitives::traits::{Hash, Zero};
use rstd::prelude::*;
use crate::srml::impl_trait_for_tuples::impl_for_tuples;

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// 存证模块 Trait
decl_storage! {
    trait Store for Module<T: Trait> as Claims {
        pub Proofs get(proof): map T::Hash => (T::AccountId, Vec<u8>, T::BlockNumber);
    }
}

// 存证模块 Trait
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        #[weight = 0]
        pub fn create_claim(origin, claim: Vec<u8>) -> Result {
            let sender = ensure_signed(origin)?;
            let block_number = <system::Module<T>>::block_number();

            let claim = Claim {
                sender: sender.clone(),
                claim: claim.clone(),
                created_at: block_number,
            };

            let claim_hash = (<system::Module<T>>::random_seed(), &sender, &claim)
                .using_encoded(<T as system::Trait>::Hashing::hash);
            Proofs::<T>::insert(&claim_hash, (sender.clone(), claim.clone(), block_number));

            Self::deposit_event(RawEvent::ClaimCreated(sender, claim_hash));
            Ok(())
        }

        #[weight = 0]
        pub fn revoke_claim(origin, claim: T::Hash) -> Result {
            let sender = ensure_signed(origin)?;

            let (owner, _, _) = Proofs::<T>::get(&claim);
            ensure!(sender == owner, "只有存证的所有者才能撤销存证.");

            Proofs::<T>::remove(&claim);

            Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));
            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        ClaimCreated(AccountId, T::Hash),
        ClaimRevoked(AccountId, T::Hash),
    }
);

// 存证模块 Trait
impl<T: Trait> Module<T> {
    pub fn get_claim(hash: &T::Hash) -> Option<Claim<T>> {
         <Proofs<T>>::get(hash).map(|(ref sender, ref claim, ref created_at)| Claim {
            sender: sender.clone(),
            claim: claim.clone(),
            created_at: created_at.clone(),
         })
    }
}
