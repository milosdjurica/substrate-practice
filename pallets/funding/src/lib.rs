#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    // #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // ! Create events here
        OwnershipTransferred(T::AccountId, T::AccountId),
    }

    // ALICE ADDR ->  5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
    #[pallet::storage]
    #[pallet::getter(fn owner)]

    pub type Owner<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

    #[pallet::error]
    pub enum Error<T> {
        NotOwner,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Implement your pallet's callable functions here

        #[pallet::weight(Weight::default())]
        #[pallet::call_index(0)]
        pub fn transfer_ownership(origin: OriginFor<T>, new_owner: T::AccountId) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Ensure that the caller is the current owner
            ensure!(
                Some(sender.clone()) == Owner::<T>::get(),
                Error::<T>::NotOwner
            );

            // Set new owner
            Owner::<T>::put(&new_owner);

            // Emit event
            Self::deposit_event(Event::OwnershipTransferred(sender, new_owner));

            Ok(())
        }
    }
}
