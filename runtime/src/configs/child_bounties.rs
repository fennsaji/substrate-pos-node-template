use crate::*;

impl pallet_child_bounties::Config for Runtime {
	type Event = Event;
	type MaxActiveChildBountyCount = MaxActiveChildBountyCount;
	type ChildBountyValueMinimum = ChildBountyValueMinimum;
	type ChildBountyCuratorDepositBase = ChildBountyCuratorDepositBase;
	type WeightInfo = pallet_child_bounties::weights::SubstrateWeight<Runtime>;
}