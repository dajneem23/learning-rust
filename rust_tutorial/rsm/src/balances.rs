use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};
pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}
impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }
    pub fn get_balance(&mut self, who: &T::AccountId) -> T::Balance {
        //dereferencing
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }
    pub fn transfer(
        &mut self,
        caller: &T::AccountId,
        to: &T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.get_balance(&caller);
        let to_balance = self.get_balance(&to);
        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient Balance")?;

        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("Overflow when adding new balance")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::u128::{MAX, MIN};
    struct TestConfig;
    impl system::Config for TestConfig{
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce =  u32;
    }
    impl Config for TestConfig {
        type Balance = u128;
    }
    use crate::system;

    use super::*; // Import everything from the outer scope

    fn init_balances() {
        let mut map = Pallet::<TestConfig>::new();
        map.set_balance(&"alice".to_string(), 100);
        map.set_balance(&"bob".to_string(), 50);
        assert_eq!(map.get_balance(&"alice".to_string()), 100);
        assert_eq!(map.get_balance(&"bob".to_string()), 50);
        assert_eq!(map.get_balance(&"Charles".to_string()), 0);
    }

    #[test]
    fn transfer() {
        let mut map = Pallet::<TestConfig>::new();
        map.set_balance(&"alice".to_string(), 100);
        map.set_balance(&"bob".to_string(), 50);
        map.transfer(&"alice".to_string(), &"bob".to_string(), 10);
        assert_eq!(map.get_balance(&"alice".to_string()), 90);
        assert_eq!(map.get_balance(&"bob".to_string()), 60);

        map.set_balance(&"Min".to_string(), u128::MIN);
        map.set_balance(&"Max".to_string(), u128::MAX);
        let failed1 = map.transfer(&"Min".to_string(), &"Max".to_string(), 1);
        assert!(failed1.is_err());
        assert_eq!(failed1, Err("Insufficient Balance"));
    }
}
