use crate::*;


parameter_types! {
	pub const MaxMembers: u32 = 100;
}


impl pallet_membership::Config<pallet_membership::Instance1> for Runtime {
	type Event = Event;
	type AddOrigin = EnsureRootOrHalfCouncil;
	type RemoveOrigin = EnsureRootOrHalfCouncil;
	type SwapOrigin = EnsureRootOrHalfCouncil;
	type ResetOrigin = EnsureRootOrHalfCouncil;
	type PrimeOrigin = EnsureRootOrHalfCouncil;
	type MembershipInitialized = TechnicalCommittee;
	type MembershipChanged = TechnicalCommittee;
	type MaxMembers = MaxMembers;
	type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>;
}
