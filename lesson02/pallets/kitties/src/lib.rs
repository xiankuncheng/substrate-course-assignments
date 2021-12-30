#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Randomness};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_io::hashing::blake2_128;

	#[derive(Encode, Decode, TypeInfo)]
	pub struct Kitty(pub [u8; 16]);

	type KittyIndex = u32;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]
	pub type KittiesCount<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub(super) type Kitties<T> =
		StorageMap<_, Blake2_128Concat, KittyIndex, Option<Kitty>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> =
		StorageMap<_, Blake2_128Concat, KittyIndex, Option<T::AccountId>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreate(T::AccountId, KittyIndex),
		KittyTransfer(T::AccountId, T::AccountId, KittyIndex),
	}

	#[pallet::error]
	pub enum Error<T> {
		KittiesCountOverflow,
		NotOwner,
		SameParentIndex,
		InvalidKittyIndex,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_id =
				Self::kitties_count().checked_add(1).ok_or(Error::<T>::KittiesCountOverflow)?;
			let dna = Self::random_value(&who);

			Self::create_kitty(kitty_id, dna, &who);

			Self::deposit_event(Event::KittyCreate(who, kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(
			origin: OriginFor<T>,
			new_owner: T::AccountId,
			kitty_id: KittyIndex,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id), Error::<T>::NotOwner);

			Owner::<T>::insert(kitty_id, Some(new_owner.clone()));

			Self::deposit_event(Event::KittyTransfer(who, new_owner, kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id1: KittyIndex,
			kitty_id2: KittyIndex,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// kitty id not equal
			ensure!(kitty_id1 != kitty_id2, Error::<T>::SameParentIndex);

			// kitties exist
			let kitty1 = Self::kitties(kitty_id1).ok_or(Error::<T>::InvalidKittyIndex)?;
			let kitty2 = Self::kitties(kitty_id2).ok_or(Error::<T>::InvalidKittyIndex)?;

			// kitties belong to owner
			Owner::<T>::get(kitty_id1).ok_or(Error::<T>::NotOwner)?;
			Owner::<T>::get(kitty_id2).ok_or(Error::<T>::NotOwner)?;

			// get kitty id
			let kitty_id =
				Self::kitties_count().checked_add(1).ok_or(Error::<T>::KittiesCountOverflow)?;

			// dna generate
			let dna_1 = kitty1.0;
			let dna_2 = kitty2.0;

			let selector = Self::random_value(&who);
			let mut new_dna = [0u8; 16];

			for i in 0..new_dna.len() {
				new_dna[i] = (selector[i] & dna_1[i]) | (!selector[i] & dna_2[i]);
			}

			Self::create_kitty(kitty_id, new_dna, &who);

			Self::deposit_event(Event::KittyCreate(who, kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn buy(
			origin: OriginFor<T>,
			kitty_id: KittyIndex,
			from: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_host = Owner::<T>::get(kitty_id).ok_or(Error::<T>::InvalidKittyIndex)?;
			ensure!(kitty_host == from, Error::<T>::NotOwner);

			Owner::<T>::insert(kitty_id, Some(who.clone()));

			Self::deposit_event(Event::KittyTransfer(from, who, kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn sell(
			origin: OriginFor<T>,
			kitty_id: KittyIndex,
			dest: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_host = Owner::<T>::get(kitty_id).ok_or(Error::<T>::InvalidKittyIndex)?;
			ensure!(kitty_host == who, Error::<T>::NotOwner);

			Owner::<T>::insert(kitty_id, Some(who.clone()));

			Self::deposit_event(Event::KittyTransfer(who, dest, kitty_id));

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}

		fn create_kitty(kitty_id: u32, dna: [u8; 16], who: &T::AccountId) {
			Kitties::<T>::insert(kitty_id, Some(Kitty(dna)));
			Owner::<T>::insert(kitty_id, Some(who.clone()));
			KittiesCount::<T>::put(kitty_id);
		}
	}
}
