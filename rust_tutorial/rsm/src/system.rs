use std::{collections::BTreeMap, ops::AddAssign};

use num::{CheckedAdd, CheckedSub, One, Zero};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + CheckedSub + CheckedAdd + Copy + AddAssign;
    type Nonce: Zero + One + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}
impl<T: Config> Pallet<T>
// where
//     AccountId: Ord + Clone,
//     BlockNumber: Zero + One + CheckedSub + CheckedAdd + Copy + AddAssign,
//     Nonce: Zero + One + CheckedSub + CheckedAdd + Copy,
{
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }
    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one()
    }
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = self.nonce.get(who).unwrap_or(&T::Nonce::zero()).clone();
        self.nonce.insert(who.clone(), nonce + T::Nonce::one());
    }
    pub fn nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }
}
mod tests {
    use super::*; // Import everything from the outer scope
    struct TestConfig ;
    impl Config for TestConfig{
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce =  u32;
    }

    #[test]
    fn test_nonce() {
        let mut system= Pallet::< TestConfig> ::new();
        let alex = &String::from("alex");
        system.inc_nonce(alex);
        assert_eq!(system.nonce(alex), 1);
        let ben = &String::from("ben");
        system.inc_nonce(ben);
        system.inc_nonce(ben);
        system.inc_nonce(ben);
        assert_eq!(system.nonce(ben), 3);
        assert_eq!(system.nonce(&String::from("alice")), 0);
    }
    #[test]
    fn test_block() {
        let mut system= Pallet::< TestConfig>::new();
        system.inc_block_number();
        system.inc_block_number();
        system.inc_block_number();
        system.inc_block_number();
        assert_eq!(system.block_number(), 4);
    }
    
}
