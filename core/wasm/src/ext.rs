use primitives::types::{AccountAlias, PromiseId};

pub mod ids {
    // Storate related
    pub const STORAGE_READ_LEN_FUNC: usize = 100;
    pub const STORAGE_READ_INTO_FUNC: usize = 110;
    pub const STORAGE_WRITE_FUNC: usize = 120;

    // Balance
    pub const BALANCE_FUNC: usize = 200;
    pub const TRANSFER_FUNC: usize = 210;

    // Contract
    /// Function from gas counter
    pub const GAS_FUNC: usize = 300;
    pub const ASSERT_HAS_MANA_FUNC: usize = 310;

    // Promises
    pub const PROMISE_CREATE_FUNC: usize = 400;
    pub const PROMISE_THEN_FUNC: usize = 410;
    pub const PROMISE_AND_FUNC: usize = 420;

    // Dev
    pub const PANIC_FUNC: usize = 1000;
    pub const DEBUG_FUNC: usize = 1010;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NotImplemented,
}

pub type Result<T> = ::std::result::Result<T, Error>;

pub trait External {
    fn storage_set(&mut self, key: &[u8], value: &[u8]) -> Result<()>;

    fn storage_get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;

    fn promise_create(
        &mut self,
        account_alias: AccountAlias,
        method_name: Vec<u8>,
        arguments: Vec<u8>,
        mana: u32,
        amount: u64,
    ) -> Result<PromiseId>;

    fn promise_then(
        &mut self,
        promise_id: PromiseId,
        method_name: Vec<u8>,
        arguments: Vec<u8>,
        mana: u32,
    ) -> Result<PromiseId>;

    fn promise_and(
        &mut self,
        promise_id1: PromiseId,
        promise_id2: PromiseId,
    ) -> Result<PromiseId>;

}