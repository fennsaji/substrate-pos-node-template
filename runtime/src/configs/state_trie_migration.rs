use crate::*;

parameter_types! {
	pub const SignedMigrationMaxLimits: pallet_state_trie_migration::MigrationLimits =
		pallet_state_trie_migration::MigrationLimits { size: 1024 * 1024 / 2, item: 512 };
	pub const MigrationSignedDepositPerItem: Balance = 1 * CENTS;
	pub const MigrationSignedDepositBase: Balance = 20 * DOLLARS;
}

impl pallet_state_trie_migration::Config for Runtime {
	type Event = Event;
	type ControlOrigin = EnsureRoot<AccountId>;
	type Currency = Balances;
	type SignedDepositPerItem = MigrationSignedDepositPerItem;
	type SignedDepositBase = MigrationSignedDepositBase;
	type SignedMigrationMaxLimits = SignedMigrationMaxLimits;
	// Warning: this is not advised, as it might allow the chain to be temporarily DOS-ed.
	// Preferably, if the chain's governance/maintenance team is planning on using a specific
	// account for the migration, put it here to make sure only that account can trigger the signed
	// migrations.
	type SignedFilter = EnsureSigned<Self::AccountId>;
	type WeightInfo = ();
}
