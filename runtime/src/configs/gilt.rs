use crate::*;

parameter_types! {
	pub IgnoredIssuance: Balance = Treasury::pot();
	pub const QueueCount: u32 = 300;
	pub const MaxQueueLen: u32 = 1000;
	pub const FifoQueueLen: u32 = 500;
	pub const Period: BlockNumber = 30 * DAYS;
	pub const MinFreeze: Balance = 100 * DOLLARS;
	pub const IntakePeriod: BlockNumber = 10;
	pub const MaxIntakeBids: u32 = 10;
}

impl pallet_gilt::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type AdminOrigin = frame_system::EnsureRoot<AccountId>;
	type Deficit = ();
	type Surplus = ();
	type IgnoredIssuance = IgnoredIssuance;
	type QueueCount = QueueCount;
	type MaxQueueLen = MaxQueueLen;
	type FifoQueueLen = FifoQueueLen;
	type Period = Period;
	type MinFreeze = MinFreeze;
	type IntakePeriod = IntakePeriod;
	type MaxIntakeBids = MaxIntakeBids;
	type WeightInfo = pallet_gilt::weights::SubstrateWeight<Runtime>;
}
