#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod flipper {
    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    impl Flipper {
        /// Initiate the contract state with provided value.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Initiate the contract state with value equals flase.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Returns the current value
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        /// Flip the current value.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::flipper::*;

    #[ink::test]
    fn default_works() {
        let flipper = Flipper::default();
        assert_eq!(flipper.get(), false);
    }

    #[ink::test]
    fn flip_works() {
        let mut flipper = Flipper::new(false);
        assert_eq!(flipper.get(), false);

        flipper.flip();
        assert_eq!(flipper.get(), true);
    }
}
