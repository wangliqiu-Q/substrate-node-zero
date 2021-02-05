//! Simple Token Transfer


use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
};
use frame_system::{self as system, ensure_signed};


pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as PalletZero {
		pub Balances get(fn get_balance): map hasher(blake2_128_concat) T::AccountId => u64;
		//                                          初始值
		// ---------------------------------------///////////
		pub TotalSupply get(fn total_supply): u64 = 21000000;

		Init get(fn is_init): bool;
	}
}

decl_event!(
	pub enum Event<T>
	where
		<T as system::Trait>::AccountId,
	{
		/// Token was initialized by user
		Initialized(AccountId),
		/// [from, to, value]
		TransferToken(AccountId, AccountId, u64),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		AlreadyInitialized,
		InsufficientFunds,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// 真正的 initialize tokens 的方式有 genesis config, claims process, lockdrop, and many more.
		/// 这里仅仅为了测试
		#[weight = 10_000]
		fn init_token(origin) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(!Self::is_init(), <Error<T>>::AlreadyInitialized);

			<Balances<T>>::insert(sender, Self::total_supply());
			Init::put(true);
			Ok(())
		}

		#[weight = 10_000]
		fn transfer_token(origin, to: T::AccountId, value: u64) -> DispatchResult {
			let user = ensure_signed(origin)?;
			let from_balance = Self::get_balance(&user);
			let to_balance = Self::get_balance(&to);

			// Calculate balances
			// checked_sub：u64 不能小于 0
			let updated_from_balance = from_balance.checked_sub(value).ok_or(<Error<T>>::InsufficientFunds)?;
			// 总量已经写死 21000000 ，不会超过 u64 max，所以 checked_add 不会返回 None
			// use `.expect()` provide a proof of why the potential panic will never happen.
			let updated_to_balance = to_balance.checked_add(value).expect("Entire supply tokens = 21000000;");

			// Write new balances to storage
			<Balances<T>>::insert(&user, updated_from_balance);
			<Balances<T>>::insert(&to, updated_to_balance);

			Self::deposit_event(RawEvent::TransferToken(user, to, value));
			Ok(())
		}
	}
}
