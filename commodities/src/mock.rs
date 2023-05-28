use crate as pallet_commodities;
use frame_support::traits::{ConstU16, ConstU64, LockableCurrency, ReservableCurrency};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	
};
use pallet_balances::Pallet::<mock::Test>;


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		CommoditiesModule: pallet_commodities,
		Balances: pallet_balances,
	}
);

// impl pallet_balances::Config for Runtime {
// 	type MaxLocks = ConstU32<50>;
// 	type MaxReserves = ();
// 	type ReserveIdentifier = [u8; 8];
// 	/// The type for recording an account's balance.
// 	type Balance = Balance;
// 	/// The ubiquitous event type.
// 	type RuntimeEvent = RuntimeEvent;
// 	type DustRemoval = ();
// 	type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
// 	type AccountStore = System;
// 	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
// }

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_commodities::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
