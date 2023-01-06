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

use crate::{relay_chain, ASSET_RESERVE_PARA_ID};
use frame_support::{
	construct_runtime, match_types, parameter_types,
	traits::{Everything, Nothing},
};
pub use polkadot_core_primitives::AccountId;
use polkadot_core_primitives::{Balance, BlockNumber, Hash};
use polkadot_parachain::primitives::Id as ParaId;
use polkadot_runtime_common::BlockHashCount;
use polkadot_runtime_parachains::{configuration, dmp, origin, shared, ump, Origin};
use rococo_runtime::{ExistentialDeposit, FirstMessageFactorPercent, MaxLocks, MaxReserves};
use sp_runtime::{
	generic,
	traits::{BlakeTwo256, IdentityLookup},
};
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, ChildParachainAsNative,
	ChildParachainConvertsVia, ChildSystemParachainAsSuperuser,
	CurrencyAdapter as XcmCurrencyAdapter, FixedWeightBounds, IsChildSystemParachain, IsConcrete,
	LocationInverter, SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation,
	TakeWeightCredit, UsingComponents, WeightInfoBounds,
};
use xcm_executor::{Config, XcmExecutor};

impl frame_system::Config for Runtime {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
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

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
}

impl shared::Config for Runtime {}

impl configuration::Config for Runtime {
	type WeightInfo = configuration::TestWeightInfo;
}

parameter_types! {
	pub const RocLocation: MultiLocation = Here.into();
	pub RococoNetwork: NetworkId =
		NetworkId::Named(b"Rococo".to_vec().try_into().expect("shorter than length limit; qed"));
	pub Ancestry: MultiLocation = Here.into();
	pub CheckAccount: AccountId = XcmPallet::check_account();
}

pub type SovereignAccountOf =
	(ChildParachainConvertsVia<ParaId, AccountId>, AccountId32Aliases<RococoNetwork, AccountId>);

pub type LocalAssetTransactor = XcmCurrencyAdapter<
	Balances,
	IsConcrete<RocLocation>,
	SovereignAccountOf,
	AccountId,
	CheckAccount,
>;

type LocalOriginConverter = (
	SovereignSignedViaLocation<SovereignAccountOf, RuntimeOrigin>,
	ChildParachainAsNative<Origin, RuntimeOrigin>,
	SignedAccountId32AsNative<RococoNetwork, RuntimeOrigin>,
	ChildSystemParachainAsSuperuser<ParaId, RuntimeOrigin>,
);

parameter_types! {
	pub const BaseXcmWeight: u64 = 1_000_000_000;
	pub const MaxInstructions: u32 = 100;
	pub const Rococo: MultiAssetFilter = Wild(AllOf { fun: WildFungible, id: Concrete(RocLocation::get()) });
	pub const Statemine: MultiLocation = Parachain(ASSET_RESERVE_PARA_ID).into();
	pub const RococoForStatemine: (MultiAssetFilter, MultiLocation) = (Rococo::get(), Statemine::get());
}

match_types! {
	pub type OnlyParachains: impl Contains<MultiLocation> = {
		MultiLocation { parents: 0, interior: X1(Parachain(_)) }
	};
}

pub type Barrier = (
	TakeWeightCredit,
	AllowTopLevelPaidExecutionFrom<Everything>,
	AllowUnpaidExecutionFrom<IsChildSystemParachain<ParaId>>,
	AllowKnownQueryResponses<XcmPallet>,
	AllowSubscriptionsFrom<OnlyParachains>,
);
pub type TrustedTeleporters = (xcm_builder::Case<RococoForStatemine>,);
pub type XcmRouter = super::RelayChainXcmRouter;

pub struct XcmConfig;
impl Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = LocalOriginConverter;
	type IsReserve = ();
	type IsTeleporter = TrustedTeleporters;
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher =
		WeightInfoBounds<weights::xcm::RococoXcmWeight<RuntimeCall>, RuntimeCall, MaxInstructions>;
	type Trader = UsingComponents<
		rococo_runtime_constants::fee::WeightToFee,
		RocLocation,
		AccountId,
		Balances,
		(),
	>;
	type ResponseHandler = XcmPallet;
	type AssetTrap = XcmPallet;
	type AssetClaims = XcmPallet;
	type SubscriptionService = XcmPallet;
}

pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RococoNetwork>;

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

#[allow(dead_code)]
pub(crate) fn check_account() -> AccountId {
	relay_chain::XcmPallet::check_account()
}

mod weights {

	pub(crate) mod pallet_balances {
		use frame_support::{traits::Get, weights::Weight};
		use sp_std::marker::PhantomData;

		/// Weight functions for `pallet_balances`.
		pub struct WeightInfo<T>(PhantomData<T>);
		impl<T: frame_system::Config> pallet_balances::WeightInfo for WeightInfo<T> {
			// Storage: System Account (r:1 w:1)
			fn transfer() -> Weight {
				Weight::from_ref_time(40_460_000 as u64)
					.saturating_add(T::DbWeight::get().reads(1 as u64))
					.saturating_add(T::DbWeight::get().writes(1 as u64))
			}
			// Storage: System Account (r:1 w:1)
			fn transfer_keep_alive() -> Weight {
				Weight::from_ref_time(29_508_000 as u64)
					.saturating_add(T::DbWeight::get().reads(1 as u64))
					.saturating_add(T::DbWeight::get().writes(1 as u64))
			}
			// Storage: System Account (r:1 w:1)
			fn set_balance_creating() -> Weight {
				Weight::from_ref_time(22_142_000 as u64)
					.saturating_add(T::DbWeight::get().reads(1 as u64))
					.saturating_add(T::DbWeight::get().writes(1 as u64))
			}
			// Storage: System Account (r:1 w:1)
			fn set_balance_killing() -> Weight {
				Weight::from_ref_time(25_653_000 as u64)
					.saturating_add(T::DbWeight::get().reads(1 as u64))
					.saturating_add(T::DbWeight::get().writes(1 as u64))
			}
			// Storage: System Account (r:2 w:2)
			fn force_transfer() -> Weight {
				Weight::from_ref_time(39_913_000 as u64)
					.saturating_add(T::DbWeight::get().reads(2 as u64))
					.saturating_add(T::DbWeight::get().writes(2 as u64))
			}
			// Storage: System Account (r:1 w:1)
			fn transfer_all() -> Weight {
				Weight::from_ref_time(34_497_000 as u64)
					.saturating_add(T::DbWeight::get().reads(1 as u64))
					.saturating_add(T::DbWeight::get().writes(1 as u64))
			}
			// Storage: System Account (r:1 w:1)
			fn force_unreserve() -> Weight {
				Weight::from_ref_time(19_749_000 as u64)
					.saturating_add(T::DbWeight::get().reads(1 as u64))
					.saturating_add(T::DbWeight::get().writes(1 as u64))
			}
		}
	}

	pub(crate) mod xcm {
		use super::super::Runtime;
		use frame_support::weights::Weight;
		use sp_std::prelude::*;
		use xcm::{
			latest::{prelude::*, Weight as XCMWeight},
			DoubleEncoded,
		};

		use pallet_xcm_benchmarks_fungible::WeightInfo as XcmBalancesWeight;
		use pallet_xcm_benchmarks_generic::WeightInfo as XcmGeneric;

		/// Types of asset supported by the Rococo runtime.
		pub enum AssetTypes {
			/// An asset backed by `pallet-balances`.
			Balances,
			/// Unknown asset.
			Unknown,
		}

		impl From<&MultiAsset> for AssetTypes {
			fn from(asset: &MultiAsset) -> Self {
				match asset {
					MultiAsset {
						id: Concrete(MultiLocation { parents: 0, interior: Here }),
						..
					} => AssetTypes::Balances,
					_ => AssetTypes::Unknown,
				}
			}
		}

		trait WeighMultiAssets {
			fn weigh_multi_assets(&self, balances_weight: Weight) -> XCMWeight;
		}

		// Rococo only knows about one asset, the balances pallet.
		const MAX_ASSETS: u32 = 1;

		impl WeighMultiAssets for MultiAssetFilter {
			fn weigh_multi_assets(&self, balances_weight: Weight) -> XCMWeight {
				let weight = match self {
					Self::Definite(assets) => assets
						.inner()
						.into_iter()
						.map(From::from)
						.map(|t| match t {
							AssetTypes::Balances => balances_weight,
							AssetTypes::Unknown => Weight::MAX,
						})
						.fold(Weight::zero(), |acc, x| acc.saturating_add(x)),
					Self::Wild(_) => balances_weight.saturating_mul(MAX_ASSETS as u64),
				};

				weight.ref_time()
			}
		}

		impl WeighMultiAssets for MultiAssets {
			fn weigh_multi_assets(&self, balances_weight: Weight) -> XCMWeight {
				let weight = self
					.inner()
					.into_iter()
					.map(|m| <AssetTypes as From<&MultiAsset>>::from(m))
					.map(|t| match t {
						AssetTypes::Balances => balances_weight,
						AssetTypes::Unknown => Weight::MAX,
					})
					.fold(Weight::zero(), |acc, x| acc.saturating_add(x));

				weight.ref_time()
			}
		}

		pub struct RococoXcmWeight<Call>(core::marker::PhantomData<Call>);
		impl<Call> XcmWeightInfo<Call> for RococoXcmWeight<Call> {
			fn withdraw_asset(assets: &MultiAssets) -> XCMWeight {
				assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::withdraw_asset())
			}
			fn reserve_asset_deposited(assets: &MultiAssets) -> XCMWeight {
				assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::reserve_asset_deposited())
			}
			fn receive_teleported_asset(assets: &MultiAssets) -> XCMWeight {
				assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::receive_teleported_asset())
			}
			fn query_response(
				_query_id: &u64,
				_response: &Response,
				_max_weight: &u64,
			) -> XCMWeight {
				XcmGeneric::<Runtime>::query_response().ref_time()
			}
			fn transfer_asset(assets: &MultiAssets, _dest: &MultiLocation) -> XCMWeight {
				assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::transfer_asset())
			}
			fn transfer_reserve_asset(
				assets: &MultiAssets,
				_dest: &MultiLocation,
				_xcm: &Xcm<()>,
			) -> XCMWeight {
				assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::transfer_reserve_asset())
			}
			fn transact(
				_origin_type: &OriginKind,
				_require_weight_at_most: &u64,
				_call: &DoubleEncoded<Call>,
			) -> XCMWeight {
				XcmGeneric::<Runtime>::transact().ref_time()
			}
			fn hrmp_new_channel_open_request(
				_sender: &u32,
				_max_message_size: &u32,
				_max_capacity: &u32,
			) -> XCMWeight {
				// XCM Executor does not currently support HRMP channel operations
				Weight::MAX.ref_time()
			}
			fn hrmp_channel_accepted(_recipient: &u32) -> XCMWeight {
				// XCM Executor does not currently support HRMP channel operations
				Weight::MAX.ref_time()
			}
			fn hrmp_channel_closing(
				_initiator: &u32,
				_sender: &u32,
				_recipient: &u32,
			) -> XCMWeight {
				// XCM Executor does not currently support HRMP channel operations
				Weight::MAX.ref_time()
			}
			fn clear_origin() -> XCMWeight {
				XcmGeneric::<Runtime>::clear_origin().ref_time()
			}
			fn descend_origin(_who: &InteriorMultiLocation) -> XCMWeight {
				XcmGeneric::<Runtime>::descend_origin().ref_time()
			}
			fn report_error(
				_query_id: &QueryId,
				_dest: &MultiLocation,
				_max_response_weight: &u64,
			) -> XCMWeight {
				XcmGeneric::<Runtime>::report_error().ref_time()
			}

			fn deposit_asset(
				assets: &MultiAssetFilter,
				_max_assets: &u32, // TODO use max assets?
				_dest: &MultiLocation,
			) -> XCMWeight {
				assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::deposit_asset())
			}
			fn deposit_reserve_asset(
				assets: &MultiAssetFilter,
				_max_assets: &u32, // TODO use max assets?
				_dest: &MultiLocation,
				_xcm: &Xcm<()>,
			) -> XCMWeight {
				assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::deposit_reserve_asset())
			}
			fn exchange_asset(_give: &MultiAssetFilter, _receive: &MultiAssets) -> XCMWeight {
				Weight::MAX.ref_time() // todo fix
			}
			fn initiate_reserve_withdraw(
				assets: &MultiAssetFilter,
				_reserve: &MultiLocation,
				_xcm: &Xcm<()>,
			) -> XCMWeight {
				assets.weigh_multi_assets(XcmGeneric::<Runtime>::initiate_reserve_withdraw())
			}
			fn initiate_teleport(
				assets: &MultiAssetFilter,
				_dest: &MultiLocation,
				_xcm: &Xcm<()>,
			) -> XCMWeight {
				assets.weigh_multi_assets(XcmBalancesWeight::<Runtime>::initiate_teleport())
			}
			fn query_holding(
				_query_id: &u64,
				_dest: &MultiLocation,
				_assets: &MultiAssetFilter,
				_max_response_weight: &u64,
			) -> XCMWeight {
				XcmGeneric::<Runtime>::query_holding().ref_time()
			}
			fn buy_execution(_fees: &MultiAsset, _weight_limit: &WeightLimit) -> XCMWeight {
				XcmGeneric::<Runtime>::buy_execution().ref_time()
			}
			fn refund_surplus() -> XCMWeight {
				XcmGeneric::<Runtime>::refund_surplus().ref_time()
			}
			fn set_error_handler(_xcm: &Xcm<Call>) -> XCMWeight {
				XcmGeneric::<Runtime>::set_error_handler().ref_time()
			}
			fn set_appendix(_xcm: &Xcm<Call>) -> XCMWeight {
				XcmGeneric::<Runtime>::set_appendix().ref_time()
			}
			fn clear_error() -> XCMWeight {
				XcmGeneric::<Runtime>::clear_error().ref_time()
			}
			fn claim_asset(_assets: &MultiAssets, _ticket: &MultiLocation) -> XCMWeight {
				XcmGeneric::<Runtime>::claim_asset().ref_time()
			}
			fn trap(_code: &u64) -> XCMWeight {
				XcmGeneric::<Runtime>::trap().ref_time()
			}
			fn subscribe_version(_query_id: &QueryId, _max_response_weight: &u64) -> XCMWeight {
				XcmGeneric::<Runtime>::subscribe_version().ref_time()
			}
			fn unsubscribe_version() -> XCMWeight {
				XcmGeneric::<Runtime>::unsubscribe_version().ref_time()
			}
		}

		mod pallet_xcm_benchmarks_fungible {
			use frame_support::{traits::Get, weights::Weight};
			use sp_std::marker::PhantomData;

			/// Weights for `pallet_xcm_benchmarks::fungible`.
			pub struct WeightInfo<T>(PhantomData<T>);
			impl<T: frame_system::Config> WeightInfo<T> {
				// Storage: System Account (r:1 w:1)
				pub(crate) fn withdraw_asset() -> Weight {
					Weight::from_ref_time(20_385_000 as u64)
						.saturating_add(T::DbWeight::get().reads(1 as u64))
						.saturating_add(T::DbWeight::get().writes(1 as u64))
				}
				// Storage: System Account (r:2 w:2)
				pub(crate) fn transfer_asset() -> Weight {
					Weight::from_ref_time(32_756_000 as u64)
						.saturating_add(T::DbWeight::get().reads(2 as u64))
						.saturating_add(T::DbWeight::get().writes(2 as u64))
				}
				// Storage: System Account (r:2 w:2)
				// Storage: XcmPallet SupportedVersion (r:1 w:0)
				// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
				// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
				// Storage: Configuration ActiveConfig (r:1 w:0)
				// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
				// Storage: Dmp DownwardMessageQueues (r:1 w:1)
				pub(crate) fn transfer_reserve_asset() -> Weight {
					Weight::from_ref_time(50_645_000 as u64)
						.saturating_add(T::DbWeight::get().reads(8 as u64))
						.saturating_add(T::DbWeight::get().writes(5 as u64))
				}
				// Storage: Benchmark Override (r:0 w:0)
				pub(crate) fn reserve_asset_deposited() -> Weight {
					Weight::from_ref_time(2_000_000_000_000 as u64)
				}
				// Storage: System Account (r:1 w:1)
				pub(crate) fn receive_teleported_asset() -> Weight {
					Weight::from_ref_time(19_595_000 as u64)
						.saturating_add(T::DbWeight::get().reads(1 as u64))
						.saturating_add(T::DbWeight::get().writes(1 as u64))
				}
				// Storage: System Account (r:1 w:1)
				pub(crate) fn deposit_asset() -> Weight {
					Weight::from_ref_time(21_763_000 as u64)
						.saturating_add(T::DbWeight::get().reads(1 as u64))
						.saturating_add(T::DbWeight::get().writes(1 as u64))
				}
				// Storage: System Account (r:1 w:1)
				// Storage: XcmPallet SupportedVersion (r:1 w:0)
				// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
				// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
				// Storage: Configuration ActiveConfig (r:1 w:0)
				// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
				// Storage: Dmp DownwardMessageQueues (r:1 w:1)
				pub(crate) fn deposit_reserve_asset() -> Weight {
					Weight::from_ref_time(40_930_000 as u64)
						.saturating_add(T::DbWeight::get().reads(7 as u64))
						.saturating_add(T::DbWeight::get().writes(4 as u64))
				}
				// Storage: System Account (r:1 w:1)
				// Storage: XcmPallet SupportedVersion (r:1 w:0)
				// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
				// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
				// Storage: Configuration ActiveConfig (r:1 w:0)
				// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
				// Storage: Dmp DownwardMessageQueues (r:1 w:1)
				pub(crate) fn initiate_teleport() -> Weight {
					Weight::from_ref_time(40_788_000 as u64)
						.saturating_add(T::DbWeight::get().reads(7 as u64))
						.saturating_add(T::DbWeight::get().writes(4 as u64))
				}
			}
		}

		mod pallet_xcm_benchmarks_generic {
			use frame_support::{traits::Get, weights::Weight};
			use sp_std::marker::PhantomData;

			/// Weights for `pallet_xcm_benchmarks::generic`.
			pub struct WeightInfo<T>(PhantomData<T>);
			impl<T: frame_system::Config> WeightInfo<T> {
				// Storage: XcmPallet SupportedVersion (r:1 w:0)
				// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
				// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
				// Storage: Configuration ActiveConfig (r:1 w:0)
				// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
				// Storage: Dmp DownwardMessageQueues (r:1 w:1)
				pub(crate) fn query_holding() -> Weight {
					Weight::from_ref_time(21_822_000 as u64)
						.saturating_add(T::DbWeight::get().reads(6 as u64))
						.saturating_add(T::DbWeight::get().writes(3 as u64))
				}
				pub(crate) fn buy_execution() -> Weight {
					Weight::from_ref_time(3_109_000 as u64)
				}
				// Storage: XcmPallet Queries (r:1 w:0)
				pub(crate) fn query_response() -> Weight {
					Weight::from_ref_time(12_087_000 as u64)
						.saturating_add(T::DbWeight::get().reads(1 as u64))
				}
				pub(crate) fn transact() -> Weight {
					Weight::from_ref_time(12_398_000 as u64)
				}
				pub(crate) fn refund_surplus() -> Weight {
					Weight::from_ref_time(3_247_000 as u64)
				}
				pub(crate) fn set_error_handler() -> Weight {
					Weight::from_ref_time(3_086_000 as u64)
				}
				pub(crate) fn set_appendix() -> Weight {
					Weight::from_ref_time(3_112_000 as u64)
				}
				pub(crate) fn clear_error() -> Weight {
					Weight::from_ref_time(3_118_000 as u64)
				}
				pub(crate) fn descend_origin() -> Weight {
					Weight::from_ref_time(4_054_000 as u64)
				}
				pub(crate) fn clear_origin() -> Weight {
					Weight::from_ref_time(3_111_000 as u64)
				}
				// Storage: XcmPallet SupportedVersion (r:1 w:0)
				// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
				// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
				// Storage: Configuration ActiveConfig (r:1 w:0)
				// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
				// Storage: Dmp DownwardMessageQueues (r:1 w:1)
				pub(crate) fn report_error() -> Weight {
					Weight::from_ref_time(18_425_000 as u64)
						.saturating_add(T::DbWeight::get().reads(6 as u64))
						.saturating_add(T::DbWeight::get().writes(3 as u64))
				}
				// Storage: XcmPallet AssetTraps (r:1 w:1)
				pub(crate) fn claim_asset() -> Weight {
					Weight::from_ref_time(7_144_000 as u64)
						.saturating_add(T::DbWeight::get().reads(1 as u64))
						.saturating_add(T::DbWeight::get().writes(1 as u64))
				}
				pub(crate) fn trap() -> Weight {
					Weight::from_ref_time(3_060_000 as u64)
				}
				// Storage: XcmPallet VersionNotifyTargets (r:1 w:1)
				// Storage: XcmPallet SupportedVersion (r:1 w:0)
				// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
				// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
				// Storage: Configuration ActiveConfig (r:1 w:0)
				// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
				// Storage: Dmp DownwardMessageQueues (r:1 w:1)
				pub(crate) fn subscribe_version() -> Weight {
					Weight::from_ref_time(21_642_000 as u64)
						.saturating_add(T::DbWeight::get().reads(7 as u64))
						.saturating_add(T::DbWeight::get().writes(4 as u64))
				}
				// Storage: XcmPallet VersionNotifyTargets (r:0 w:1)
				pub(crate) fn unsubscribe_version() -> Weight {
					Weight::from_ref_time(4_873_000 as u64)
						.saturating_add(T::DbWeight::get().writes(1 as u64))
				}
				// Storage: XcmPallet SupportedVersion (r:1 w:0)
				// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
				// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
				// Storage: Configuration ActiveConfig (r:1 w:0)
				// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
				// Storage: Dmp DownwardMessageQueues (r:1 w:1)
				pub(crate) fn initiate_reserve_withdraw() -> Weight {
					Weight::from_ref_time(22_809_000 as u64)
						.saturating_add(T::DbWeight::get().reads(6 as u64))
						.saturating_add(T::DbWeight::get().writes(3 as u64))
				}
			}
		}
	}
}
