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
//! // 7. runtime/set
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

use frame_support::{decl_module, decl_event, decl_error, decl_storage, dispatch::DispatchResult, ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
// Substrate runtimes are compiled to both Web Assembly and a regular native binary, and do not have
// access to rust's standard library.
// only able to print items that implement the `Printable` trait
// 启动参数必须加 -lruntime=debug
use sp_runtime::print;
use frame_support::debug::native;

/// `Configuration Trait`
/// 该 Trait 所声明的关联类型都必须在 runtime/lib.rs 中 impl pallet_zero::Trait for Runtime { } 具体化，其中
/// Runtime 为单元结构体，聚合实现了所有 pallet 的 Trait
/// pallet 的所有本地类型都会携带泛型 <T: Trait> ，其中 T 就是 Runtime
pub trait Trait: system::Trait {
	/// <Self as system::Trait>::Event 为父 trait 的关联类型 Event
	/// From<Event<Self>> 中的 Event 为 decl_event! 所生成的 RawEvent<<T as system::Trait>::AccountId>
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

type GroupIndex = u32;

// 猜测是内部公用了一个存储实例 Storage ，只不过前缀不同 module_prefix + storage_prefix
//
decl_storage! {
	//									module_prefix
	// ---------------------------------//////////
	trait Store for Module<T: Trait> as PalletZero {
		/// `EXP`
		/// impl<T: Trait> StorageMap<T::AccountId, u32> for UserMap<T>
		/// impl<T: Trait> StoragePrefixedMap<u32> for UserMap<T>
		/// `frame_support::storage::StorageMap`
		/// https://substrate.dev/rustdocs/v2.0.0/frame_support/storage/trait.StorageMap.html
		///
		/// `UserMap` - 类单元结构体
		/// `get(fn simple_map)` - 为当前 pallet Module<T> 实现 simple_map 方法，内部用 get 方法实现。
		/// `: map hasher(blake2_128_concat)` - declare type is map with blake2_128_concat hasher.
		/// `T::AccountId => u32` - key and value type of the map.
		///
		/// Choosing a Hasher:
		/// `blake2_128_concat`: keep your storage tree balanced. 比如防御某人用大量的 AccountId 来恶意攻击。
		/// `twox_64_concat`: efficient than blake2, You should not use this hasher if chain users can
		/// affect the storage keys.
		/// `identity`: merely an identity function that returns the same value it receives. This hasher
		/// is only an option when the key type in your storage map is already a hash.
		UserMap get(fn simple_map): map hasher(blake2_128_concat) T::AccountId => u32;

		/// `EXP`
		/// impl<T: Trait> StorageValue<T::AccountId> for UserCache<T>
		/// `frame_support::storage::StorageValue`
		/// https://substrate.dev/rustdocs/v2.0.0/frame_support/storage/trait.StorageValue.html
		UserCache get(fn user_cache): T::AccountId;

		/// `double_map`: `remove_prefix(first_key)` remove all values with the first_key identifier
		UserScore get(fn user_score):
			//                                    first key                             second key
			// ----------------------------------//////////----------------------------////////////
			double_map hasher(blake2_128_concat) GroupIndex, hasher(blake2_128_concat) T::AccountId => u32;
		/// Get GroupIndex for user
		UserGroup get(fn group_membership): map hasher(blake2_128_concat) T::AccountId => GroupIndex;

	}
}

/// `EXP`
/// ```
/// pub enum RawEvent<AccountId> {
///     EmitInput(AccountId, u32),
/// }
///
/// pub type Event<T> = RawEvent<<T as system::Trait>::AccountId>;
/// ```
fn _expand_decl_event() {}

// T 为实现当前 pallet `Trait` 的 Runtime
// if events need types from the pallet's Configuration Trait, eg: AccountId
decl_event!(
	pub enum Event<T>
	where
		// Id = <T as system::Trait>::AccountId	// 指定类型别名 Id
		<T as system::Trait>::AccountId,
	{
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [AccountId]
		Init(AccountId),

		/// [user, value]
		InsertEntry(AccountId, u32),
		/// [user, value]
		GetEntry(AccountId, u32),
		/// [user, old_value, new_value]
		IncreaseEntry(AccountId, u32, u32),

		/// [old_user, new_user]
		UpdateCache(AccountId, AccountId),

		/// [GroupIndex]
		RemoveGroup(GroupIndex),

	}
);


/// `EXP`
/// pub enum ZeroError<T: Trait> { /* */ }
/// impl<T: Trait> From<ZeroError<T>> for &'static str
/// impl<T: Trait> From<ZeroError<T>> for sp_runtime::DispatchError
fn _expand_decl_error() {}

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
/// 	/// Deposits an event using `system::Module::deposit_event`.
/// 	fn deposit_event(event: impl Into<<T as Trait>::Event>) {
/// 		<system::Module<T>>::deposit_event(event.into())
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
fn _expand_decl_module() {}

// Dispatchable calls are defined here
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// initialize Error
		type Error = ZeroError<T>;

		// 为了下面 Self::deposit_event 提供默认实现
		fn deposit_event() = default;

		/// weights affect the fees a user will have to pay to call the function.
		#[weight = 10_000]
		/// return: Result<(), sp_runtime::DispatchError>
		pub fn init(origin) -> DispatchResult {
			let user = ensure_signed(origin)?;

			print("Hello World");
			// debug::info!("user: {:?}", user); // wasm 也会编译，只打印 user:
			native::info!("user: {:?}", user);	// wasm 不会编译。

			<UserScore<T>>::insert(&1u32, &user, 111u32);
			<UserGroup<T>>::insert(&user, &1u32);

			// emit event
			Self::deposit_event(RawEvent::Init(user));
			Ok(())
		}

		#[weight = 10_000]
		fn insert_entry(origin, value: u32) -> DispatchResult {
			let user = ensure_signed(origin)?;

			<UserMap<T>>::insert(&user, value);
			Self::deposit_event(RawEvent::InsertEntry(user, value));

			Ok(())
		}

		#[weight = 10_000]
		fn get_entry(origin) -> DispatchResult {
			let user = ensure_signed(origin)?;
			ensure!(<UserMap<T>>::contains_key(&user), ZeroError::<T>::NoValueStored);

			// StorageMap api还有 take
			let value = <UserMap<T>>::get(&user);
			Self::deposit_event(RawEvent::GetEntry(user, value));

			Ok(())
		}

		#[weight = 10_000]
		fn increase_entry(origin, add_this_val: u32) -> DispatchResult {
			let user = ensure_signed(origin)?;
			ensure!(<UserMap<T>>::contains_key(&user), ZeroError::<T>::NoValueStored);

			let original_value = <UserMap<T>>::get(&user);
			let new_value = original_value.checked_add(add_this_val).ok_or(ZeroError::<T>::MaxValueReached)?;
			<UserMap<T>>::insert(&user, new_value);
			Self::deposit_event(RawEvent::IncreaseEntry(user, original_value, new_value));

			Ok(())
		}

		/// storage 的 io 操作有一定的 cost ，应该尽量避免。
		/// 比如以下非 Copy 的类型
		#[weight = 10_000]
		fn update_cache(origin) -> DispatchResult {
			let user = ensure_signed(origin)?;
			let existing_user = <UserCache<T>>::get();
			// 因为 Self::deposit_event 会 move 掉 old_king
			let old_user = existing_user.clone();
			// 尽量避免走 io ，应该采用 clone
			// let old_user = <UserCache<T>>::get();

			<UserCache<T>>::put(user.clone());
			Self::deposit_event(RawEvent::UpdateCache(old_user, user));

			Ok(())
		}

		#[weight = 10_000]
		fn remove_group(origin) -> DispatchResult {
			let user = ensure_signed(origin)?;

			let group_id = <UserGroup<T>>::get(&user);
			// remove all group members from UserScore at once
			<UserScore<T>>::remove_prefix(&group_id);
			// <UserScore<T>>::remove(&group_id, &user);	// just remove user

			Self::deposit_event(RawEvent::RemoveGroup(group_id));
			Ok(())
		}

	}
}

fn _demo() {
	let _x = 3u64.checked_sub(4u64);
}
