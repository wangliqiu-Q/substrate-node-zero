
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_event, decl_module, dispatch::DispatchResult};
use frame_system::ensure_signed;

#[cfg(test)]
mod tests;


pub trait Trait: frame_system::Trait {
	// simple event
	type Event: From<Event> + Into<<Self as frame_system::Trait>::Event>;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		#[weight = 10_000]
		fn do_something(origin, input: u32) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			let new_number = input;
			// simple event
			Self::deposit_event(Event::EmitInput(new_number));
			Ok(())
		}
	}
}


decl_event!(
	// simple event
	pub enum Event {
		EmitInput(u32),
	}
);
