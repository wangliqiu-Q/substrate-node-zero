use frame_support::{codec::{Decode, Encode}, decl_module, decl_event,  decl_storage, dispatch::DispatchResult,};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use frame_support::debug::native;


// 							InnerThing<T> 中的 T 要转 balances::Trait
// ------------------------------///////////////
pub trait Trait: system::Trait + balances::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}


// type ThingOf<T> = SuperThing<<T as system::Trait>::Hash, <T as balances::Trait>::Balance>;

/// `Hash` `Balance` come from the system and balances pallets' configuration traits, we must
/// specify them as generics when declaring the struct.
#[derive(Encode, Decode, Clone, Default, sp_core::RuntimeDebug)]
pub struct InnerThing<Hash, Balance> {
	number: u32,
	hash: Hash,
	balance: Balance,
}

#[derive(Encode, Decode, Clone, Default, sp_core::RuntimeDebug)]
pub struct SuperThing<Hash, Balance> {
	number: u32,
	inner: InnerThing<Hash, Balance>,
}


decl_storage! {
	trait Store for Module<T: Trait> as PalletZero {
		SuperThingMap get(fn super_thing_map):
			map hasher(blake2_128_concat) T::AccountId => SuperThing<T::Hash, T::Balance>;
			// map hasher(blake2_128_concat) T::AccountId => ThingOf<T> // 这种写法，需要定义以下别名
			// type ThingOf<T> = SuperThing<<T as system::Trait>::Hash, <T as balances::Trait>::Balance>;
	}
}


decl_event!(
	pub enum Event<T>
	where
		<T as system::Trait>::AccountId,
		<T as system::Trait>::Hash,
		<T as balances::Trait>::Balance,
	{
		StoreCustomStruct(AccountId, u32, Hash, Balance),
	}
);


decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		#[weight = 10_000]
		fn insert_custom_struct(origin, number: u32, hash: T::Hash, balance: T::Balance) -> DispatchResult {
			let user = ensure_signed(origin)?;
			let inner_thing = InnerThing { number, hash, balance };
			let super_thing = SuperThing { number, inner: inner_thing};
			<SuperThingMap<T>>::insert(&user, super_thing);

			let thing = Self::super_thing_map(&user);	// <SuperThingMap<T>>::get(&user)
			native::info!("custom_struct: {:?}", thing);
			Self::deposit_event(RawEvent::StoreCustomStruct(user, thing.number, thing.inner.hash, thing.inner.balance));
			Ok(())
		}

	}
}


