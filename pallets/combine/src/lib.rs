#![cfg_attr(not(feature = "std"), no_std)]

//! runtime constant
//!
//!
//!

use frame_support::{
	decl_event, decl_module, decl_storage,
	dispatch::{DispatchError, DispatchResult},
	ensure,
	traits::Get,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::Zero;


#[cfg(test)]
mod tests;


pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	/// Maximum amount added per invocation
	type MaxAddend: Get<u32>;

	/// SingleValue is set to 0 every ClearFrequency number of blocks
	type ClearFrequency: Get<Self::BlockNumber>;
}

decl_storage! {
	trait Store for Module<T: Trait> as PalletZero {
		SingleValue get(fn single_value): u32;
	}
}

decl_event!(
	pub enum Event<T>
	where
		<T as system::Trait>::AccountId,
	{
		/// [initial amount, amount added, final amount]
		SingleValueAdded(u32, u32, u32),
		/// The parameter is the value before clearing. [old_val]
		SingleValueCleared(u32),

		Hold(AccountId),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
		// In order to make these constants and their values appear in the runtime metadata,
		const MaxAddend: u32 = T::MaxAddend::get();
		const ClearFrequency: T::BlockNumber = T::ClearFrequency::get();

		#[weight = 10_000]
		fn add_value(origin, val_to_add: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			ensure!(val_to_add <= T::MaxAddend::get(), "val_to_add > T::MaxAddend::get()");

			let c_val = <SingleValue>::get();

			let result = c_val.checked_add(val_to_add).ok_or(DispatchError::Other("Addition overflowed"))?;
			<SingleValue>::put(result);
			Self::deposit_event(RawEvent::SingleValueAdded( c_val, val_to_add, result));
			Ok(())
		}

		/// For testing purposes
		#[weight = 10_000]
		fn set_value(origin, value: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			<SingleValue>::put(value);
			Ok(())
		}

		/// `EXP`
		/// impl<T: system::Trait + Trait> OnFinalize<<T as system::Trait>::BlockNumber> for Module<T> {}
		/// `frame_support::traits::OnFinalize`
		/// 默认 Module<T> 的 OnFinalize 实现为空，这里会覆盖 OnFinalize 的默认方法
		fn on_finalize(n: T::BlockNumber) {
			// SingleValue is set to 0 every ClearFrequency number of blocks in the on_finalize
			// function that runs at the end of blocks execution.
			if (n % T::ClearFrequency::get()).is_zero() {
				let old_val = <SingleValue>::get();
				<SingleValue>::put(0u32);
				Self::deposit_event(RawEvent::SingleValueCleared(old_val));
			}
		}
	}
}
