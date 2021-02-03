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

use frame_support::{debug, decl_module, decl_event, dispatch::DispatchResult};
use frame_system::{self  , ensure_signed};
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
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// Dispatchable calls are defined here
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Events are emitted from dispatchable calls using the `deposit_event` method.
		// 为了下面 Self::deposit_event 提供默认实现
		fn deposit_event() = default;

		/// weights affect the fees a user will have to pay to call the function.
		#[weight = 10_000]
		/// return: Result<(), sp_runtime::DispatchError>
		pub fn do_something(origin, input: u32) -> DispatchResult {
			let user = ensure_signed(origin)?;

			// could do something with the input here instead
			let new_number = input;

			print("Hello World");
			debug::info!("Request sent by: {:?}", user);

			// emit event
			Self::deposit_event(RawEvent::EmitInput(user, new_number));
			Ok(())
		}
	}
}

// T 为实现当前 pallet `Trait` 的 Runtime
// if events need types from the pallet's Configuration Trait, eg: AccountId
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Trait>::AccountId,
	{
		EmitInput(AccountId, u32),
	}
);
