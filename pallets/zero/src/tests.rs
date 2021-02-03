use super::RawEvent;
use crate::{Module, Trait};
use frame_support::{
	assert_noop, assert_ok, dispatch::DispatchError, impl_outer_origin, impl_outer_event, parameter_types,
};
use frame_system::{self as system, RawOrigin};
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};

impl_outer_origin! {
	pub enum Origin for TestRuntime {}
}

// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestRuntime;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl system::Trait for TestRuntime {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Index = u64;
	type Call = ();
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

mod pallet_zero {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for TestRuntime {
		pallet_zero<T>,
		system<T>,
	}
}



impl Trait for TestRuntime {
	type Event = TestEvent;
}

pub type System = system::Module<TestRuntime>;
pub type GenericEvent = Module<TestRuntime>;

struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let storage = system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();
		let mut ext = TestExternalities::from(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}


#[test]
fn say_hello_no_root() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			GenericEvent::do_something(RawOrigin::Root.into(), 32),
			DispatchError::BadOrigin
		);
	})
}

#[test]
fn test() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(GenericEvent::do_something(Origin::signed(1), 32));

		// construct event that should be emitted in the method call directly above
		let expected_event = TestEvent::generic_event(RawEvent::EmitInput(1, 32));

		// iterate through array of `EventRecord`s
		assert_eq!(
			System::events()[0].event,
			expected_event,
		);
	})
}
