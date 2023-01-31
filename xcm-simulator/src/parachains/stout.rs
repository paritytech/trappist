// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Stout Parachain runtime mock.

use stout_runtime::{
	constants::{
		currency::{CENTS, EXISTENTIAL_DEPOSIT, UNITS},
		fee::WeightToFee,
	},
	xcm_config::{
		Barrier, CollatorSelectionUpdateOrigin, FungiblesTransactor, LocationToAccountId,
		MaxInstructions, RelayLocation, RelayNetwork, Reserves, SelfReserve, UnitWeightCost,
		XUsdPerSecond,
	},
	BlockNumber, DealWithFees, Hash, Header, Index, Period, PotId, RuntimeBlockLength,
	RuntimeBlockWeights, Session, UnitBody, Version,
};
pub use stout_runtime::{AccountId, AssetId, Balance};
use frame_support::{
	construct_runtime, parameter_types,
	traits::{AsEnsureOriginWithArg, EitherOfDiverse, Everything, Nothing},
	weights::constants::RocksDbWeight,
};
use frame_system::EnsureRoot;
use pallet_xcm::{EnsureXcm, IsMajorityOfBody, XcmPassthrough};
use polkadot_runtime_common::BlockHashCount;
use sp_core::{ConstU128, ConstU16, ConstU32};
use sp_runtime::traits::{AccountIdLookup, BlakeTwo256};
use sp_std::prelude::*;
use xcm::latest::prelude::*;
use xcm_builder::{
	CurrencyAdapter, EnsureXcmOrigin, FixedRateOfFungible, FixedWeightBounds, IsConcrete,
	LocationInverter, ParentAsSuperuser, RelayChainAsNative, SiblingParachainAsNative,
	SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, UsingComponents,
};
use xcm_executor::{Config, XcmExecutor};

impl frame_system::Config for Runtime {
	type BaseCallFilter = Everything;
	type BlockWeights = RuntimeBlockWeights;
	type BlockLength = RuntimeBlockLength;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = Index;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type DbWeight = RocksDbWeight;
	type Version = Version;
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = frame_system::weights::SubstrateWeight<Runtime>;
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl super::mock_msg_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

pub type AssetsForceOrigin =
	EitherOfDiverse<EnsureRoot<AccountId>, EnsureXcm<IsMajorityOfBody<RelayLocation, UnitBody>>>;

impl pallet_assets::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = AssetId;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type ForceOrigin = AssetsForceOrigin;
	type AssetDeposit = ConstU128<UNITS>;
	type AssetAccountDeposit = ConstU128<{ UNITS }>;
	type MetadataDepositBase = ConstU128<{ UNITS }>;
	type MetadataDepositPerByte = ConstU128<{ 10 * CENTS }>;
	type ApprovalDeposit = ConstU128<{ 10 * CENTS }>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
}

impl pallet_collator_selection::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type UpdateOrigin = CollatorSelectionUpdateOrigin;
	type PotId = PotId;
	type MaxCandidates = ConstU32<1000>;
	type MinCandidates = ConstU32<5>;
	type MaxInvulnerables = ConstU32<100>;
	type KickThreshold = Period;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type ValidatorRegistration = Session;
	type WeightInfo = pallet_collator_selection::weights::SubstrateWeight<Runtime>;
}

impl pallet_sudo::Config for Runtime {
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
}

pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Everything;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Nothing;
	type XcmReserveTransferFilter = Everything;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type LocationInverter = LocationInverter<Ancestry>;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

parameter_types! {
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(MsgQueue::parachain_id().into()).into();
}

pub type AssetTransactors = (CurrencyTransactor, FungiblesTransactor);
pub type CurrencyTransactor =
	CurrencyAdapter<Balances, IsConcrete<RelayLocation>, LocationToAccountId, AccountId, ()>;
pub type XcmOriginToTransactDispatchOrigin = (
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	ParentAsSuperuser<RuntimeOrigin>,
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	XcmPassthrough<RuntimeOrigin>,
);
pub type XcmRouter = crate::ParachainXcmRouter<MsgQueue>;

pub struct XcmConfig;
impl Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = AssetTransactors;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = Reserves;
	type IsTeleporter = ();
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type Trader = (
		FixedRateOfFungible<XUsdPerSecond, ()>,
		UsingComponents<WeightToFee, SelfReserve, AccountId, Balances, DealWithFees<Runtime>>,
	);
	type ResponseHandler = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		MsgQueue: super::mock_msg_queue::{Pallet, Storage, Event<T>},
		CollatorSelection: pallet_collator_selection::{Pallet, Call, Storage, Event<T>, Config<T>} = 21,
		PolkadotXcm: pallet_xcm::{Pallet, Call, Event<T>, Origin} = 31,
		CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin} = 32,
		Sudo: pallet_sudo = 40,
		Assets: pallet_assets = 43,
	}
);
