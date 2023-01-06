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

//! Relay chain runtime mock.

use frame_support::{
	construct_runtime, parameter_types,
	traits::{Everything, Nothing},
};
use polkadot_core_primitives::BlockNumber;
use sp_core::H256;
use sp_runtime::{generic, traits::IdentityLookup, AccountId32};

use polkadot_parachain::primitives::Id as ParaId;
use polkadot_runtime_parachains::{configuration, dmp, hrmp, origin, paras, shared, ump};
use rococo_runtime::FirstMessageFactorPercent;
use sp_runtime::traits::BlakeTwo256;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowUnpaidExecutionFrom, ChildParachainAsNative,
	ChildParachainConvertsVia, ChildSystemParachainAsSuperuser,
	CurrencyAdapter as XcmCurrencyAdapter, FixedRateOfFungible, FixedWeightBounds, IsConcrete,
	LocationInverter, SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation,
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
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
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

impl shared::Config for Runtime {}

impl configuration::Config for Runtime {
	type WeightInfo = configuration::TestWeightInfo;
}

parameter_types! {
	pub const KsmLocation: MultiLocation = Here.into();
	pub const KusamaNetwork: NetworkId = NetworkId::Kusama;
	pub const AnyNetwork: NetworkId = NetworkId::Any;
	pub Ancestry: MultiLocation = Here.into();
	pub UnitWeightCost: u64 = 1_000;
}

pub type SovereignAccountOf =
	(ChildParachainConvertsVia<ParaId, AccountId>, AccountId32Aliases<KusamaNetwork, AccountId>);

pub type LocalAssetTransactor =
	XcmCurrencyAdapter<Balances, IsConcrete<KsmLocation>, SovereignAccountOf, AccountId, ()>;

type LocalOriginConverter = (
	SovereignSignedViaLocation<SovereignAccountOf, RuntimeOrigin>,
	ChildParachainAsNative<origin::Origin, RuntimeOrigin>,
	SignedAccountId32AsNative<KusamaNetwork, RuntimeOrigin>,
	ChildSystemParachainAsSuperuser<ParaId, RuntimeOrigin>,
);

parameter_types! {
	pub const BaseXcmWeight: u64 = 1_000;
	pub KsmPerSecond: (AssetId, u128) = (Concrete(KsmLocation::get()), 1);
	pub const MaxInstructions: u32 = 100;
}

pub type XcmRouter = super::RelayChainXcmRouter;
pub type Barrier = AllowUnpaidExecutionFrom<Everything>;

pub struct XcmConfig;
impl Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = LocalOriginConverter;
	type IsReserve = ();
	type IsTeleporter = ();
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
	type Trader = FixedRateOfFungible<KsmPerSecond, ()>;
	type ResponseHandler = ();
	type AssetTrap = ();
	type AssetClaims = ();
	type SubscriptionService = ();
}

pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, KusamaNetwork>;

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	// Anyone can execute XCM messages locally...
	type ExecuteXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Everything;
	type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
	type LocationInverter = LocationInverter<Ancestry>;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

// impl paras::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// 	type UnsignedPriority = ParasUnsignedPriority;
// 	type NextSessionRotation = Babe;
// 	type WeightInfo = weights::runtime_parachains_paras::WeightInfo<Runtime>;
// }

impl ump::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type UmpSink = ump::XcmSink<XcmExecutor<XcmConfig>, Runtime>;
	type FirstMessageFactorPercent = FirstMessageFactorPercent;
	type ExecuteOverweightOrigin = frame_system::EnsureRoot<AccountId>;
	type WeightInfo = ump::TestWeightInfo;
}

impl dmp::Config for Runtime {}

// impl hrmp::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// 	type RuntimeOrigin = RuntimeOrigin;
// 	type Currency = Balances;
// 	type WeightInfo = weights::runtime_parachains_hrmp::WeightInfo<Runtime>;
// }

impl origin::Config for Runtime {}

impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
}

#[frame_support::pallet]
pub mod mock_paras_sudo_wrapper {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type XcmRouter: SendXcm;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight((1_000, DispatchClass::Operational))]
		pub fn sudo_queue_downward_xcm(
			origin: OriginFor<T>,
			id: ParaId,
			xcm: Box<xcm::opaque::VersionedXcm>,
		) -> DispatchResult {
			ensure_root(origin)?;
			let dest = MultiLocation::new(0, Junctions::X1(Parachain(id.into())));
			let message = Xcm::<()>::try_from(*xcm).unwrap();
			T::XcmRouter::send_xcm(dest, message)
				.map_err(|_| sp_runtime::DispatchError::Other("Sudo routing failed"))
		}
	}
}

impl mock_paras_sudo_wrapper::Config for Runtime {
	type XcmRouter = XcmRouter;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
	RuntimeCall: From<C>,
{
	type Extrinsic = UncheckedExtrinsic;
	type OverarchingCall = RuntimeCall;
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
		ParasOrigin: origin::{Pallet, Origin},
		ParasUmp: ump::{Pallet, Call, Storage, Event},
		XcmPallet: pallet_xcm::{Pallet, Call, Storage, Event<T>, Origin, Config},

		ParasSudoWrapper: mock_paras_sudo_wrapper::{Pallet, Call},
		Sudo: pallet_sudo::{Pallet, Call, Storage, Event<T>, Config<T>},
	}
);

mod weights {
	pub(crate) mod runtime_parachains_hrmp {
		use frame_support::{traits::Get, weights::Weight};
		use sp_std::marker::PhantomData;

		/// Weight functions for `runtime_parachains::hrmp`.
		pub struct WeightInfo<T>(PhantomData<T>);
		impl<T: frame_system::Config> super::super::hrmp::WeightInfo for WeightInfo<T> {
			// Storage: Paras ParaLifecycles (r:2 w:0)
			// Storage: Configuration ActiveConfig (r:1 w:0)
			// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
			// Storage: Hrmp HrmpChannels (r:1 w:0)
			// Storage: Hrmp HrmpEgressChannelsIndex (r:1 w:0)
			// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
			// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
			// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
			// Storage: Dmp DownwardMessageQueues (r:1 w:1)
			fn hrmp_init_open_channel() -> Weight {
				Weight::from_ref_time(40_520_000 as u64)
					.saturating_add(T::DbWeight::get().reads(10 as u64))
					.saturating_add(T::DbWeight::get().writes(5 as u64))
			}
			// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
			// Storage: Configuration ActiveConfig (r:1 w:0)
			// Storage: Paras ParaLifecycles (r:1 w:0)
			// Storage: Hrmp HrmpIngressChannelsIndex (r:1 w:0)
			// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:1 w:1)
			// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
			// Storage: Dmp DownwardMessageQueues (r:1 w:1)
			fn hrmp_accept_open_channel() -> Weight {
				Weight::from_ref_time(39_646_000 as u64)
					.saturating_add(T::DbWeight::get().reads(7 as u64))
					.saturating_add(T::DbWeight::get().writes(4 as u64))
			}
			// Storage: Hrmp HrmpChannels (r:1 w:0)
			// Storage: Hrmp HrmpCloseChannelRequests (r:1 w:1)
			// Storage: Hrmp HrmpCloseChannelRequestsList (r:1 w:1)
			// Storage: Configuration ActiveConfig (r:1 w:0)
			// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
			// Storage: Dmp DownwardMessageQueues (r:1 w:1)
			fn hrmp_close_channel() -> Weight {
				Weight::from_ref_time(36_691_000 as u64)
					.saturating_add(T::DbWeight::get().reads(6 as u64))
					.saturating_add(T::DbWeight::get().writes(4 as u64))
			}
			// Storage: Hrmp HrmpIngressChannelsIndex (r:128 w:127)
			// Storage: Hrmp HrmpEgressChannelsIndex (r:1 w:1)
			// Storage: Hrmp HrmpChannels (r:127 w:127)
			// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:0 w:1)
			// Storage: Hrmp HrmpChannelContents (r:0 w:127)
			// Storage: Hrmp HrmpOpenChannelRequestCount (r:0 w:1)
			/// The range of component `i` is `[0, 127]`.
			/// The range of component `e` is `[0, 127]`.
			fn force_clean_hrmp(i: u32, e: u32) -> Weight {
				Weight::from_ref_time(0 as u64)
					// Standard Error: 16_000
					.saturating_add(
						Weight::from_ref_time(7_248_000 as u64).saturating_mul(i as u64),
					)
					// Standard Error: 16_000
					.saturating_add(
						Weight::from_ref_time(7_311_000 as u64).saturating_mul(e as u64),
					)
					.saturating_add(T::DbWeight::get().reads(2 as u64))
					.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(i as u64)))
					.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(e as u64)))
					.saturating_add(T::DbWeight::get().writes(4 as u64))
					.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(i as u64)))
					.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(e as u64)))
			}
			// Storage: Configuration ActiveConfig (r:1 w:0)
			// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:0)
			// Storage: Hrmp HrmpOpenChannelRequests (r:2 w:2)
			// Storage: Paras ParaLifecycles (r:4 w:0)
			// Storage: Hrmp HrmpIngressChannelsIndex (r:2 w:2)
			// Storage: Hrmp HrmpEgressChannelsIndex (r:2 w:2)
			// Storage: Hrmp HrmpOpenChannelRequestCount (r:2 w:2)
			// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:2 w:2)
			// Storage: Hrmp HrmpChannels (r:0 w:2)
			/// The range of component `c` is `[0, 128]`.
			fn force_process_hrmp_open(c: u32) -> Weight {
				Weight::from_ref_time(0 as u64)
					// Standard Error: 19_000
					.saturating_add(
						Weight::from_ref_time(15_783_000 as u64).saturating_mul(c as u64),
					)
					.saturating_add(T::DbWeight::get().reads(2 as u64))
					.saturating_add(T::DbWeight::get().reads((7 as u64).saturating_mul(c as u64)))
					.saturating_add(T::DbWeight::get().writes(1 as u64))
					.saturating_add(T::DbWeight::get().writes((6 as u64).saturating_mul(c as u64)))
			}
			// Storage: Hrmp HrmpCloseChannelRequestsList (r:1 w:0)
			// Storage: Hrmp HrmpChannels (r:2 w:2)
			// Storage: Hrmp HrmpEgressChannelsIndex (r:2 w:2)
			// Storage: Hrmp HrmpIngressChannelsIndex (r:2 w:2)
			// Storage: Hrmp HrmpCloseChannelRequests (r:0 w:2)
			// Storage: Hrmp HrmpChannelContents (r:0 w:2)
			/// The range of component `c` is `[0, 128]`.
			fn force_process_hrmp_close(c: u32) -> Weight {
				Weight::from_ref_time(0 as u64)
					// Standard Error: 12_000
					.saturating_add(
						Weight::from_ref_time(9_624_000 as u64).saturating_mul(c as u64),
					)
					.saturating_add(T::DbWeight::get().reads(1 as u64))
					.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(c as u64)))
					.saturating_add(T::DbWeight::get().writes(1 as u64))
					.saturating_add(T::DbWeight::get().writes((5 as u64).saturating_mul(c as u64)))
			}
			// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
			// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
			// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
			/// The range of component `c` is `[0, 128]`.
			fn hrmp_cancel_open_request(c: u32) -> Weight {
				Weight::from_ref_time(30_548_000 as u64)
					// Standard Error: 1_000
					.saturating_add(Weight::from_ref_time(89_000 as u64).saturating_mul(c as u64))
					.saturating_add(T::DbWeight::get().reads(3 as u64))
					.saturating_add(T::DbWeight::get().writes(3 as u64))
			}
			// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
			// Storage: Hrmp HrmpOpenChannelRequests (r:2 w:2)
			/// The range of component `c` is `[0, 128]`.
			fn clean_open_channel_requests(c: u32) -> Weight {
				Weight::from_ref_time(1_732_000 as u64)
					// Standard Error: 4_000
					.saturating_add(
						Weight::from_ref_time(2_574_000 as u64).saturating_mul(c as u64),
					)
					.saturating_add(T::DbWeight::get().reads(1 as u64))
					.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
					.saturating_add(T::DbWeight::get().writes(1 as u64))
					.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
			}
		}
	}

	pub(crate) mod runtime_parachains_paras {
		use frame_support::{traits::Get, weights::Weight};
		use sp_std::marker::PhantomData;

		/// Weight functions for `runtime_parachains::paras`.
		pub struct WeightInfo<T>(PhantomData<T>);
		impl<T: frame_system::Config> super::super::paras::WeightInfo for WeightInfo<T> {
			// Storage: Paras CurrentCodeHash (r:1 w:1)
			// Storage: Paras CodeByHashRefs (r:1 w:1)
			// Storage: Paras PastCodeMeta (r:1 w:1)
			// Storage: Paras PastCodePruning (r:1 w:1)
			// Storage: Paras PastCodeHash (r:0 w:1)
			// Storage: Paras CodeByHash (r:0 w:1)
			/// The range of component `c` is `[1, 3145728]`.
			fn force_set_current_code(c: u32) -> Weight {
				Weight::from_ref_time(0 as u64)
					// Standard Error: 0
					.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(c as u64))
					.saturating_add(T::DbWeight::get().reads(4 as u64))
					.saturating_add(T::DbWeight::get().writes(6 as u64))
			}
			// Storage: Paras Heads (r:0 w:1)
			/// The range of component `s` is `[1, 1048576]`.
			fn force_set_current_head(s: u32) -> Weight {
				Weight::from_ref_time(0 as u64)
					// Standard Error: 0
					.saturating_add(Weight::from_ref_time(1_000 as u64).saturating_mul(s as u64))
					.saturating_add(T::DbWeight::get().writes(1 as u64))
			}
			// Storage: Configuration ActiveConfig (r:1 w:0)
			// Storage: Paras FutureCodeHash (r:1 w:1)
			// Storage: Paras CurrentCodeHash (r:1 w:0)
			// Storage: Paras UpgradeCooldowns (r:1 w:1)
			// Storage: Paras PvfActiveVoteMap (r:1 w:0)
			// Storage: Paras CodeByHash (r:1 w:1)
			// Storage: Paras UpcomingUpgrades (r:1 w:1)
			// Storage: System Digest (r:1 w:1)
			// Storage: Paras CodeByHashRefs (r:1 w:1)
			// Storage: Paras FutureCodeUpgrades (r:0 w:1)
			// Storage: Paras UpgradeRestrictionSignal (r:0 w:1)
			/// The range of component `c` is `[1, 3145728]`.
			fn force_schedule_code_upgrade(c: u32) -> Weight {
				Weight::from_ref_time(0 as u64)
					// Standard Error: 0
					.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(c as u64))
					.saturating_add(T::DbWeight::get().reads(9 as u64))
					.saturating_add(T::DbWeight::get().writes(8 as u64))
			}
			// Storage: Paras FutureCodeUpgrades (r:1 w:0)
			// Storage: Paras Heads (r:0 w:1)
			// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
			/// The range of component `s` is `[1, 1048576]`.
			fn force_note_new_head(s: u32) -> Weight {
				Weight::from_ref_time(0 as u64)
					// Standard Error: 0
					.saturating_add(Weight::from_ref_time(1_000 as u64).saturating_mul(s as u64))
					.saturating_add(T::DbWeight::get().reads(1 as u64))
					.saturating_add(T::DbWeight::get().writes(2 as u64))
			}
			// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
			// Storage: Paras ActionsQueue (r:1 w:1)
			fn force_queue_action() -> Weight {
				Weight::from_ref_time(24_187_000 as u64)
					.saturating_add(T::DbWeight::get().reads(2 as u64))
					.saturating_add(T::DbWeight::get().writes(1 as u64))
			}
			// Storage: Paras PvfActiveVoteMap (r:1 w:0)
			// Storage: Paras CodeByHash (r:1 w:1)
			/// The range of component `c` is `[1, 3145728]`.
			fn add_trusted_validation_code(c: u32) -> Weight {
				Weight::from_ref_time(0 as u64)
					// Standard Error: 0
					.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(c as u64))
					.saturating_add(T::DbWeight::get().reads(2 as u64))
					.saturating_add(T::DbWeight::get().writes(1 as u64))
			}
			// Storage: Paras CodeByHashRefs (r:1 w:0)
			// Storage: Paras CodeByHash (r:0 w:1)
			fn poke_unused_validation_code() -> Weight {
				Weight::from_ref_time(7_273_000 as u64)
					.saturating_add(T::DbWeight::get().reads(1 as u64))
					.saturating_add(T::DbWeight::get().writes(1 as u64))
			}
			// Storage: Configuration ActiveConfig (r:1 w:0)
			// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
			// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
			// Storage: Paras PvfActiveVoteMap (r:1 w:1)
			fn include_pvf_check_statement() -> Weight {
				Weight::from_ref_time(96_047_000 as u64)
					.saturating_add(T::DbWeight::get().reads(4 as u64))
					.saturating_add(T::DbWeight::get().writes(1 as u64))
			}
			// Storage: Configuration ActiveConfig (r:1 w:0)
			// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
			// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
			// Storage: Paras PvfActiveVoteMap (r:1 w:1)
			// Storage: Paras PvfActiveVoteList (r:1 w:1)
			// Storage: Paras UpcomingUpgrades (r:1 w:1)
			// Storage: System Digest (r:1 w:1)
			// Storage: Paras FutureCodeUpgrades (r:0 w:100)
			fn include_pvf_check_statement_finalize_upgrade_accept() -> Weight {
				Weight::from_ref_time(630_640_000 as u64)
					.saturating_add(T::DbWeight::get().reads(7 as u64))
					.saturating_add(T::DbWeight::get().writes(104 as u64))
			}
			// Storage: Configuration ActiveConfig (r:1 w:0)
			// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
			// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
			// Storage: Paras PvfActiveVoteMap (r:1 w:1)
			// Storage: Paras PvfActiveVoteList (r:1 w:1)
			// Storage: Paras CodeByHashRefs (r:1 w:1)
			// Storage: Paras CodeByHash (r:0 w:1)
			// Storage: Paras UpgradeGoAheadSignal (r:0 w:100)
			// Storage: Paras FutureCodeHash (r:0 w:100)
			fn include_pvf_check_statement_finalize_upgrade_reject() -> Weight {
				Weight::from_ref_time(599_325_000 as u64)
					.saturating_add(T::DbWeight::get().reads(6 as u64))
					.saturating_add(T::DbWeight::get().writes(204 as u64))
			}
			// Storage: Configuration ActiveConfig (r:1 w:0)
			// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
			// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
			// Storage: Paras PvfActiveVoteMap (r:1 w:1)
			// Storage: Paras PvfActiveVoteList (r:1 w:1)
			// Storage: Paras ActionsQueue (r:1 w:1)
			fn include_pvf_check_statement_finalize_onboarding_accept() -> Weight {
				Weight::from_ref_time(505_499_000 as u64)
					.saturating_add(T::DbWeight::get().reads(6 as u64))
					.saturating_add(T::DbWeight::get().writes(3 as u64))
			}
			// Storage: Configuration ActiveConfig (r:1 w:0)
			// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
			// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
			// Storage: Paras PvfActiveVoteMap (r:1 w:1)
			// Storage: Paras PvfActiveVoteList (r:1 w:1)
			// Storage: Paras CodeByHashRefs (r:1 w:1)
			// Storage: Paras ParaLifecycles (r:0 w:100)
			// Storage: Paras CodeByHash (r:0 w:1)
			// Storage: Paras CurrentCodeHash (r:0 w:100)
			// Storage: Paras UpcomingParasGenesis (r:0 w:100)
			fn include_pvf_check_statement_finalize_onboarding_reject() -> Weight {
				Weight::from_ref_time(668_669_000 as u64)
					.saturating_add(T::DbWeight::get().reads(6 as u64))
					.saturating_add(T::DbWeight::get().writes(304 as u64))
			}
		}
	}
}
