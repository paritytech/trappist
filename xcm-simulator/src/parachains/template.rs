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

//! Parachain runtime mock.

use frame_support::{
	construct_runtime, parameter_types,
	traits::{Everything, Nothing},
};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup, AccountId32};
use sp_std::prelude::*;
use asset_hub_kusama::common::BlockNumber;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowUnpaidExecutionFrom, CurrencyAdapter as XcmCurrencyAdapter,
	EnsureXcmOrigin, FixedRateOfFungible, FixedWeightBounds, IsConcrete, LocationInverter,
	NativeAsset, ParentIsPreset, SiblingParachainConvertsVia, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation,
};
use xcm_executor::{Config, XcmExecutor};

pub type AccountId = AccountId32;
pub type Balance = u128;

parameter_types! {
	pub const BlockHashCount: BlockNumber = 250;
}

impl frame_system::Config for Runtime {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub ExistentialDeposit: Balance = 1;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	pub const KsmLocation: MultiLocation = MultiLocation::parent();
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
	pub Ancestry: MultiLocation = Parachain(MsgQueue::parachain_id().into()).into();
}

pub type LocationToAccountId = (
	ParentIsPreset<AccountId>,
	SiblingParachainConvertsVia<Sibling, AccountId>,
	AccountId32Aliases<RelayNetwork, AccountId>,
);

pub type XcmOriginToCallOrigin = (
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
	pub const UnitWeightCost: u64 = 1;
	pub KsmPerSecond: (AssetId, u128) = (Concrete(Parent.into()), 1);
	pub const MaxInstructions: u32 = 100;
}

pub type LocalAssetTransactor =
	XcmCurrencyAdapter<Balances, IsConcrete<KsmLocation>, LocationToAccountId, AccountId, ()>;

pub type XcmRouter = crate::ParachainXcmRouter<MsgQueue>;
pub type Barrier = AllowUnpaidExecutionFrom<Everything>;

pub struct XcmConfig;
impl Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = XcmOriginToCallOrigin;
	type IsReserve = NativeAsset;
	type IsTeleporter = ();
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type Trader = FixedRateOfFungible<KsmPerSecond, ()>;
	type ResponseHandler = ();
	type AssetTrap = ();
	type AssetClaims = ();
	type SubscriptionService = ();
}

impl super::mock_msg_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
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
		PolkadotXcm: pallet_xcm::{Pallet, Call, Event<T>, Origin},
	}
);
