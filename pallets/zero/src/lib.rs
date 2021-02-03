// tells the rust compiler that this crate should not use rust's standard library except when explicitly told to.
#![cfg_attr(not(feature = "std"), no_std)]

//! `Extrinsic`: a call from outside of the chain. Most of the time they are transactions.


use frame_support::{debug, decl_module, dispatch::DispatchResult};
use frame_system::{self as system, ensure_signed};
// Substrate runtimes are compiled to both Web Assembly and a regular native binary, and do not have
// access to rust's standard library.
// only able to print items that implement the `Printable` trait
// 启动参数必须加 -lruntime=debug
use sp_runtime::print;

#[cfg(test)]
mod tests;

/// configuration trait: access features from other pallets, or constants that affect the pallet's behavior.
pub trait Trait: system::Trait {}

// Dispatchable calls are defined here
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		/// weights affect the fees a user will have to pay to call the function.
		#[weight = 10_000]
		/// return: Result<(), sp_runtime::DispatchError>
		pub fn say_hello(origin) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			print("Hello World");
			debug::info!("Request sent by: {:?}", caller);

			Ok(())
		}
	}
}
