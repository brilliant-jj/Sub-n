// 定义存证内容的结构体
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode)]
pub struct Claim {
    pub sender: T::AccountId,
    pub claim: Vec<u8>,
    pub created_at: T::BlockNumber,
}

// 定义存证模块结构体
pub struct Module<T: Trait> {
    claims: map::Map<T::Hash, Claim<T>>,
}
