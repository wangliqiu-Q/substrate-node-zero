#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch::DispatchResult, ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		MembersCache get(fn members): Vec<T::AccountId>;
	}
}

// https://substrate.dev/docs/en/knowledgebase/runtime/events
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

// binary_search: O(log n)
pub const MAX_MEMBERS: usize = 16;

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		/// 添加元素始终会保证整个集合 sorted
		#[weight = 10_000]
		pub fn add_member(origin) -> DispatchResult {
			let user = ensure_signed(origin)?;

			let mut members = MembersCache::<T>::get();
			// 设置 limit 是为了保证性能，从而 #[weight = ] 能采用常熟
			ensure!(members.len() < MAX_MEMBERS, Error::<T>::MembersLimitReached);

			// Because the vec is always sorted, so binary_search makes O(log n).
			match members.binary_search(&user) {
				Ok(_) => Err(Error::<T>::AlreadyMember.into()),
				// If the search fails, the user is not a member and we learned the index where
				// they should be inserted to make vec sorted.
				Err(index) => {
					members.insert(index, user.clone());
					MembersCache::<T>::put(members);
					Self::deposit_event(RawEvent::MemberAdded(user));
					Ok(())
				}
			}
		}


		#[weight = 10_000]
		fn remove_member(origin) -> DispatchResult {
			let old_member = ensure_signed(origin)?;

			let mut members = MembersCache::<T>::get();

			match members.binary_search(&old_member) {
				Ok(index) => {
					members.remove(index);
					MembersCache::<T>::put(members);
					Self::deposit_event(RawEvent::MemberRemoved(old_member));
					Ok(())
				},
				Err(_) => Err(Error::<T>::NotMember.into()),
			}
		}
	}
}
