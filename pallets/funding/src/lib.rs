
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::event] // <-- Step 3. code block will replace this.
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // ! Create events here
    }
    //  #[pallet::error]   // <-- Step 4. code block will replace this.
    //  #[pallet::storage] // <-- Step 5. code block will replace this.
    //  #[pallet::call]    // <-- Step 6. code block will replace this.
}