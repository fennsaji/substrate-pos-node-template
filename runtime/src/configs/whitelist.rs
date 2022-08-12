use crate::*;

impl pallet_whitelist::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type WhitelistOrigin = EnsureRoot<AccountId>;
	type DispatchWhitelistedOrigin = EnsureRoot<AccountId>;
	type PreimageProvider = Preimage;
	type WeightInfo = pallet_whitelist::weights::SubstrateWeight<Runtime>;
}
