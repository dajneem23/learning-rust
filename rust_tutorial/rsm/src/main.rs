use system::Config;

mod balances;
mod system;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber= u32;
    pub type Nonce= u32;
}
impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber= types::BlockNumber;
    type Nonce = types::Nonce;

}
impl balances::Config for Runtime {
    type Balance =  types::Balance;
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
}
impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charles = "charles".to_string();

    runtime.balances.set_balance(&alice, 100);

    runtime.system.inc_block_number();

    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&alice);

    let _res = runtime
        .balances
        .transfer(&alice.clone(), &bob.clone(), 30)
        .map_err(|e| println!("ERRR {:?}", e));

    runtime.system.inc_block_number();
    runtime.system.inc_nonce(&alice);

    let _res = runtime
        .balances
        .transfer(&alice.clone(), &&charles.clone(), 10)
        .map_err(|e| println!("ERRR {:?}", e));

    runtime.system.inc_block_number();
    runtime.system.inc_nonce(&alice);

    println!("{:#?}", runtime);
}
