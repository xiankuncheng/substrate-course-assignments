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
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		traits::{tokens::ExistenceRequirement, Currency, Randomness, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_io::hashing::blake2_128;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type Currency: ReservableCurrency<Self::AccountId>;
		#[pallet::constant]
		type MaxKittyIndexLength: Get<u32>;
	}

	type KittyIndex = u32;
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		dna: [u8; 16],
		price: BalanceOf<T>,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]
	pub type KittiesCount<T: Config> = StorageValue<_, KittyIndex, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub(super) type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, KittyIndex, Kitty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> =
		StorageMap<_, Blake2_128Concat, KittyIndex, Option<T::AccountId>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreate(T::AccountId, KittyIndex),
		KittyTransfer(T::AccountId, T::AccountId, KittyIndex),
		KittyBuy(T::AccountId, T::AccountId, KittyIndex),
		KittySell(T::AccountId, T::AccountId, KittyIndex),
	}

	#[pallet::error]
	pub enum Error<T> {
		KittiesCountOverflow,
		NotOwner,
		SameParentIndex,
		InvalidKittyIndex,
		NotEnoughBalance,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create(origin: OriginFor<T>, price: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_id =
				Self::kitties_count().checked_add(1).ok_or(Error::<T>::KittiesCountOverflow)?;

			ensure!(kitty_id.lt(&T::MaxKittyIndexLength::get()), Error::<T>::KittiesCountOverflow);
			let dna = Self::random_value(&who);

			ensure!(T::Currency::can_reserve(&who, price), Error::<T>::NotEnoughBalance);
			T::Currency::reserve(&who, price)?;

			Self::create_kitty(kitty_id, dna, price, &who);

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

			let kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::InvalidKittyIndex)?;
			let kitty_host = Owner::<T>::get(kitty_id).ok_or(Error::<T>::NotOwner)?;
			ensure!(kitty_host == who, Error::<T>::NotOwner);

			ensure!(
				T::Currency::can_reserve(&new_owner, kitty.price),
				Error::<T>::NotEnoughBalance
			);
			T::Currency::unreserve(&who, kitty.price);
			T::Currency::reserve(&new_owner, kitty.price)?;
			Owner::<T>::insert(kitty_id, Some(new_owner.clone()));

			Self::deposit_event(Event::KittyTransfer(who, new_owner, kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id1: KittyIndex,
			kitty_id2: KittyIndex,
			price: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// kitty id not equal
			ensure!(kitty_id1 != kitty_id2, Error::<T>::SameParentIndex);

			// kitties exist
			let kitty1 = Self::kitties(kitty_id1).ok_or(Error::<T>::InvalidKittyIndex)?;
			let kitty2 = Self::kitties(kitty_id2).ok_or(Error::<T>::InvalidKittyIndex)?;

			// kitties belong to owner
			ensure!(Owner::<T>::get(kitty_id1) == Some(who.clone()), Error::<T>::NotOwner);
			ensure!(Owner::<T>::get(kitty_id2) == Some(who.clone()), Error::<T>::NotOwner);

			// get kitty id
			let kitty_id =
				Self::kitties_count().checked_add(1).ok_or(Error::<T>::KittiesCountOverflow)?;

			ensure!(kitty_id.lt(&T::MaxKittyIndexLength::get()), Error::<T>::KittiesCountOverflow);

			// dna generate
			let dna_1 = kitty1.dna;
			let dna_2 = kitty2.dna;

			let selector = Self::random_value(&who);
			let mut new_dna = [0u8; 16];

			for i in 0..new_dna.len() {
				new_dna[i] = (selector[i] & dna_1[i]) | (!selector[i] & dna_2[i]);
			}

			ensure!(T::Currency::can_reserve(&who, price), Error::<T>::NotEnoughBalance);
			T::Currency::reserve(&who, price)?;

			Self::create_kitty(kitty_id, new_dna, price, &who);

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

			let kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::InvalidKittyIndex)?;
			let kitty_host = Owner::<T>::get(kitty_id).ok_or(Error::<T>::NotOwner)?;
			ensure!(kitty_host == from, Error::<T>::NotOwner);

			ensure!(T::Currency::can_reserve(&who, kitty.price), Error::<T>::NotEnoughBalance);
			T::Currency::unreserve(&from, kitty.price);
			T::Currency::transfer(&from, &who, kitty.price, ExistenceRequirement::KeepAlive)?;

			Owner::<T>::insert(kitty_id, Some(who.clone()));

			Self::deposit_event(Event::KittyBuy(from, who, kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn sell(
			origin: OriginFor<T>,
			kitty_id: KittyIndex,
			dest: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::InvalidKittyIndex)?;
			let kitty_host = Owner::<T>::get(kitty_id).ok_or(Error::<T>::NotOwner)?;
			ensure!(kitty_host == who, Error::<T>::NotOwner);

			ensure!(T::Currency::can_reserve(&dest, kitty.price), Error::<T>::NotEnoughBalance);
			T::Currency::unreserve(&who, kitty.price);
			T::Currency::transfer(&who, &dest, kitty.price, ExistenceRequirement::KeepAlive)?;

			Owner::<T>::insert(kitty_id, Some(dest.clone()));

			Self::deposit_event(Event::KittySell(who, dest, kitty_id));

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

		fn create_kitty(kitty_id: u32, dna: [u8; 16], price: BalanceOf<T>, who: &T::AccountId) {
			let kitty = Kitty::<T> { dna, price };
			Kitties::<T>::insert(kitty_id, kitty);
			Owner::<T>::insert(kitty_id, Some(who.clone()));
			KittiesCount::<T>::put(kitty_id);
		}
	}
}
