#![cfg_attr(not(feature = "std"), no_std)]

//! If you require a lot of items checks or mutation of individual items, you should use `map-set`.
//! If you require frequent iterating, you should use `vec-set`.

use frame_support::storage::IterableStorageMap;
use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch::DispatchResult, ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_std::collections::btree_set::BTreeSet;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// todo https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	trait Store for Module<T: Trait> as PalletTemplate {
		// todo https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		MembersVec get(fn members_vec): Vec<T::AccountId>;

		// Set<K> 等于 Map<K,()>
		MembersMap get(fn members_map): map hasher(blake2_128_concat) T::AccountId => ();
		// Because the map does not store its size internally
		MembersMapCount: u32;
	}
}

// todo https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T>
	where
	  	AccountId = <T as system::Trait>::AccountId
	{
		MemberAdded(AccountId),
		MemberRemoved(AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		AlreadyMember,
		NotMember,
		MembersLimitReached,
	}
}

pub const MAX_MEMBERS: u32 = 16;

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		// DB Reads: O(1) + Decoding: O(n) + Search: O(log n) + DB Writes: O(1) + Encoding: O(n)
		/// 添加元素始终会保证整个集合 sorted
		#[weight = 10_000]
		pub fn add_member_vec_set(origin) -> DispatchResult {
			let user = ensure_signed(origin)?;

			// DB Reads: O(1) + Decoding: O(n)
			let mut members = MembersVec::<T>::get();
			// 设置 limit 是为了保证性能，从而 #[weight = ] 能采用常熟
			ensure!(members.len() < MAX_MEMBERS as usize, Error::<T>::MembersLimitReached);

			// Search: O(log n)
			// Because the vec is always sorted, so binary_search makes O(log n).
			match members.binary_search(&user) {
				Ok(_) => Err(Error::<T>::AlreadyMember.into()),
				// If the search fails, the user is not a member and we learned the index where
				// they should be inserted to make vec sorted.
				Err(index) => {
					members.insert(index, user.clone());
					// DB Writes: O(1) + Encoding: O(n)
					MembersVec::<T>::put(members);
					Self::deposit_event(RawEvent::MemberAdded(user));
					Ok(())
				}
			}
		}


		#[weight = 10_000]
		fn remove_member_vec_set(origin) -> DispatchResult {
			let old_member = ensure_signed(origin)?;

			let mut members = MembersVec::<T>::get();
			match members.binary_search(&old_member) {
				Ok(index) => {
					members.remove(index);
					MembersVec::<T>::put(members);
					Self::deposit_event(RawEvent::MemberRemoved(old_member));
					Ok(())
				},
				Err(_) => Err(Error::<T>::NotMember.into()),
			}
		}

		/// If you require a lot of items checks or mutation of individual items, you should use `map-set`.
		// DB Reads: O(1) + Encoding: O(1) + DB Writes: O(1)
		#[weight = 10_000]
		fn add_member_map_set(origin) -> DispatchResult {
			let new_member = ensure_signed(origin)?;

			let member_count = MembersMapCount::get();
			ensure!(member_count < MAX_MEMBERS, Error::<T>::MembersLimitReached);
			// DB Reads: O(1)
			ensure!(!MembersMap::<T>::contains_key(&new_member), Error::<T>::AlreadyMember);
			// Encoding: O(1) DB Writes: O(1)
			MembersMap::<T>::insert(&new_member, ());
			MembersMapCount::put(member_count + 1);
			Self::deposit_event(RawEvent::MemberAdded(new_member));

			Ok(())
		}

		#[weight = 10_000]
		fn remove_member_map_set(origin) -> DispatchResult {
			let old_member = ensure_signed(origin)?;

			ensure!(MembersMap::<T>::contains_key(&old_member), Error::<T>::NotMember);

			MembersMap::<T>::remove(&old_member);
			// 如果 MembersMapCount 不用取出来比较，则直接在缓存中 mutate
			MembersMapCount::mutate(|v| *v -= 1);
			Self::deposit_event(RawEvent::MemberRemoved(old_member));
			Ok(())
		}
	}
}

/// If you require frequent iterating, you should use `vec-set`.
impl<T: Trait> Module<T> {
	// DB Reads: O(1) + Decoding: O(n) + Processing: O(n)
	fn _vec_iter() -> BTreeSet<T::AccountId> {
		// DB Reads: O(1) + Decoding: O(n)
		Self::members_vec()
			.into_iter()
			// Processing: O(n)
			.map(|x| x )
			.collect::<BTreeSet<_>>()
	}

	// DB Reads: O(n) Decoding: O(n) Processing: O(n)
	fn _map_iter() -> BTreeSet<T::AccountId> {
		// DB Reads: O(n) + Decoding: O(n)
		// IterableStorageMap::iter() 是一个一个 io 操作的
		<MembersMap<T> as IterableStorageMap<T::AccountId, ()>>::iter()
			// Processing: O(n)
			.map(|(k, _)| k)
			.collect::<BTreeSet<_>>()
	}
}

