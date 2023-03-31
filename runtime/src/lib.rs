use pallet_template as claims;
...
parameter_types! {
    pub const ClaimMaxCount: u32 = 10;
}
impl claims::Trait for Runtime {
    type Event = Event;
}
impl system::Trait for Runtime {
    ...
}
impl Trait for Runtime {
    type Event = Event;
    type Call = Call;
    type Currency = Balances;
    type Randomness = RandomnessCollectiveFlip;
    type ClaimMaxCount = ClaimMaxCount;
}
...
impl_opaque_keys! {
    pub struct SessionKeys {
        pub grandpa: GrandpaId,
        pub babe: BabeId,
        pub im_online: ImOnlineId,
        pub authority_discovery: AuthorityDiscoveryId,
    }
}
...
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        ...
        ClaimsModule: claims::{Module, Call, Storage, Event<T>},
    }
);
...
decl_event!(
    pub enum Event
    // 泛型 <T> 表示当前事件类型的来源账户类型。
    where
        AccountId = <T as system::Trait>::AccountId,
    BlockNumber = <T as system::Trait>::BlockNumber,
    Balance = Balance,
    <T as Trait>::Hash
    {
        ...
        ClaimsModule(crate::claims::RawEvent<AccountId, <T as Trait>::Hash>),
    }
);
...
impl<T: Trait> system::offchain::CreateSignedTransaction<T> for Module<T> {
    ...
    type Call = Call<T>;
    type AdditionalSigned = (
        #[codec(compact)]
        T::BlockNumber,
        #[codec(compact)]
        Balance,
        T::Hash
    );
    ...
    fn offchain_worker(block_number: T::BlockNumber) {
        ...
        let claim = hex::encode(&env::random_bytes(16));
        let call = Call::ClaimsModule(claims::Call::create_claim(claim.into()));
        ...
    }
}
