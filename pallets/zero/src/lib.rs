// tells the rust compiler that this crate should not use rust's standard library except when explicitly told to.
#![cfg_attr(not(feature = "std"), no_std)]

//! `Extrinsic`: a call from outside of the chain. Most of the time they are transactions.
//!

//! 自定义 pallet 需要步骤：
//! ```
//! // 1. Imports
//! use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch};
//! use frame_system::ensure_signed;
//!
//! // 2. Configuration
//! pub trait Trait: frame_system::Trait {  }
//!
//! // 3. Storage
//! decl_storage! {  	}
//!
//! // 4. Events
//! decl_event! {  		}
//!
//! // 5. Errors
//! decl_error! {   	}
//!
//! // 6. Callable Functions
//! decl_module! {  	}
//!
//! // 7. runtime/lib.rs
//! impl pallet_zero::Trait for Runtime {
//! 	type Event = Event;
//! }
//!
//! // 8. Module
//! construct_runtime!(
//! 	Zero: pallet_zero::{Module, Call, Event<T>},
//! )
//! ```
//!

use frame_support::{debug, decl_module, decl_event, decl_error, decl_storage, dispatch::DispatchResult, ensure};
use frame_system::{self, ensure_signed};
// Substrate runtimes are compiled to both Web Assembly and a regular native binary, and do not have
// access to rust's standard library.
// only able to print items that implement the `Printable` trait
// 启动参数必须加 -lruntime=debug
use sp_runtime::print;

#[cfg(test)]
mod tests;

/// configuration trait: access features from other pallets, or constants that affect the pallet's behavior.
pub trait Trait: frame_system::Trait {
	/// <Self as frame_system::Trait>::Event 为父 trait 的关联类型 Event
	/// From<Event<Self>> 中的 Event 为 decl_event! 所生成的 RawEvent<<T as system::Trait>::AccountId>
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}



decl_storage! {
	// SimpleMap 会实现 frame_support::storage::StorageMap
	// https://substrate.dev/rustdocs/v2.0.0/frame_support/storage/trait.StorageMap.html
	trait Store for Module<T: Trait> as SimpleMap {
		/// `SimpleMap` - the name of the storage map
		/// `get(fn simple_map)` - the name of a getter function that will return values from the map.
		/// `: map hasher(blake2_128_concat)` - declare type is map with blake2_128_concat hasher.
		/// `T::AccountId => u32` - key and value type of the map.
		SimpleMap get(fn simple_map): map hasher(blake2_128_concat) T::AccountId => u32;
		// Choosing a Hasher:
		// `blake2_128_concat`: keep your storage tree balanced. 比如防御某人用大量的 AccountId 来恶意攻击。
		// `twox_64_concat`: efficient than blake2, You should not use this hasher if chain users can
		// affect the storage keys.
		// `identity`: merely an identity function that returns the same value it receives. This hasher
		// is only an option when the key type in your storage map is already a hash.
	}
}

/// `EXP`
/// ```
/// pub enum RawEvent<AccountId> {
///     EmitInput(AccountId, u32),
/// }
///
/// pub type Event<T> = RawEvent<<T as frame_system::Trait>::AccountId>;
/// ```
fn expand_decl_event() {}

// T 为实现当前 pallet `Trait` 的 Runtime
// if events need types from the pallet's Configuration Trait, eg: AccountId
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Trait>::AccountId,
	{
		EmitInput(AccountId, u32),

		/// (user, value)
		InsertEntry(AccountId, u32),
		/// (user, value)
		GetEntry(AccountId, u32),
		/// (user, old_value, new_value)
		IncreaseEntry(AccountId, u32, u32),
	}
);

decl_error! {
	pub enum ZeroError for Module<T: Trait> {
		NoValueStored,
		MaxValueReached,
	}
}


/// `EXP`
/// ```
/// pub struct Module<T: Trait>(PhantomData<(T)>);
///
/// impl<T: Trait> Module<T> {
/// 	/// Deposits an event using `frame_system::Module::deposit_event`.
/// 	fn deposit_event(event: impl Into<<T as Trait>::Event>) {
/// 		<frame_system::Module<T>>::deposit_event(event.into())
/// 	}
/// }
///
/// pub enum Call<T: Trait> {
/// 	#[allow(non_camel_case_types)]
///     do_something(u32),
/// }
///
/// impl<T: Trait> Module<T> {
///     pub fn do_something(origin: T::Origin, input: u32) -> DispatchResult { /* snip */ }
/// }
/// ```
fn expand_decl_module() {}

// Dispatchable calls are defined here
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = ZeroError<T>;

		// 为了下面 Self::deposit_event 提供默认实现
		fn deposit_event() = default;

		/// weights affect the fees a user will have to pay to call the function.
		#[weight = 10_000]
		/// return: Result<(), sp_runtime::DispatchError>
		pub fn do_something(origin, input: u32) -> DispatchResult {
			let user = ensure_signed(origin)?;

			// could do something with the input here instead
			let new_number = input;
			expand_decl_event();
			expand_decl_module();

			print("Hello World");
			debug::info!("Request sent by: {:?}", user);

			// emit event
			Self::deposit_event(RawEvent::EmitInput(user, new_number));
			Ok(())
		}

		#[weight = 10_000]
		fn insert_entry(origin, value: u32) -> DispatchResult {
			let user = ensure_signed(origin)?;

			<SimpleMap<T>>::insert(&user, value);
			Self::deposit_event(RawEvent::InsertEntry(user, value));

			Ok(())
		}

		#[weight = 10_000]
		fn get_entry(origin, account: T::AccountId) -> DispatchResult {
			let user = ensure_signed(origin)?;
			ensure!(<SimpleMap<T>>::contains_key(&account), ZeroError::<T>::NoValueStored);

			// StorageMap api还有 take
			let value = <SimpleMap<T>>::get(account);
			Self::deposit_event(RawEvent::GetEntry(user, value));

			Ok(())
		}

		#[weight = 10_000]
		fn increase_entry(origin, add_this_val: u32) -> DispatchResult {
			let user = ensure_signed(origin)?;
			ensure!(<SimpleMap<T>>::contains_key(&user), ZeroError::<T>::NoValueStored);

			let original_value = <SimpleMap<T>>::get(&user);
			let new_value = original_value.checked_add(add_this_val).ok_or(ZeroError::<T>::MaxValueReached)?;
			<SimpleMap<T>>::insert(&user, new_value);
			Self::deposit_event(RawEvent::IncreaseEntry(user, original_value, new_value));

			Ok(())
		}
	}
}


