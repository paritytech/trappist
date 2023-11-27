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

//! Asset Reserve Parachain runtime mock.

use frame_support::{
	construct_runtime, parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU32, Everything, Nothing},
	weights::constants::RocksDbWeight,
};
use pallet_xcm::XcmPassthrough;
pub use parachains_common::{AccountId, AssetId, Balance, Index};
use polkadot_runtime_common::BlockHashCount;
use sp_core::H256;
use sp_runtime::traits::{AccountIdLookup, BlakeTwo256, ConvertInto};
use sp_std::prelude::*;
pub use asset_hub_kusama::xcm_config::LocationToAccountId;
use asset_hub_kusama::{
	common::{
		impls::ToStakingPot, xcm_config::AssetFeeAsExistentialDepositMultiplier, BlockNumber,
		Header,
	},
	constants::fee::WeightToFee,
	xcm_config::{
		AssetTransactors, AssetsPalletLocation, Barrier, FungiblesTransactor, KsmLocation,
		MaxInstructions, RelayNetwork, XcmAssetFeesReceiver,
	},
	ApprovalDeposit, AssetAccountDeposit, AssetDeposit, AssetsForceOrigin, AssetsStringLimit,
	CollatorSelectionUpdateOrigin, ExistentialDeposit, MaxCandidates, MaxInvulnerables,
	MetadataDepositBase, MetadataDepositPerByte, MinCandidates, Period, PotId, RuntimeBlockLength,
	RuntimeBlockWeights, SS58Prefix, Session, Version,
};
use xcm::latest::prelude::*;
use xcm_builder::{
	AsPrefixedGeneralIndex, ConvertedConcreteAssetId, EnsureXcmOrigin, LocationInverter,
	NativeAsset, ParentAsSuperuser, RelayChainAsNative, SiblingParachainAsNative,
	SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, UsingComponents,
	WeightInfoBounds,
};
use xcm_executor::{
	traits::{Convert, JustTry},
	XcmExecutor,
};

impl frame_system::Config for Runtime {
	type BaseCallFilter = Everything;
	type BlockWeights = RuntimeBlockWeights;
	type BlockLength = RuntimeBlockLength;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = Index;
	type BlockNumber = BlockNumber;
	type Hash = H256;
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
	type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
	type SS58Prefix = SS58Prefix;
	type OnSetCode = (); //cumulus_pallet_parachain_system::ParachainSetCode<Self>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl super::mock_msg_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl pallet_assets::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = AssetId;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type ForceOrigin = AssetsForceOrigin;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = AssetAccountDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = AssetsStringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
	type RemoveItemsLimit = ConstU32<1000>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
	type CallbackHandle = ();
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
}

impl pallet_collator_selection::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type UpdateOrigin = CollatorSelectionUpdateOrigin;
	type PotId = PotId;
	type MaxCandidates = MaxCandidates;
	type MinCandidates = MinCandidates;
	type MaxInvulnerables = MaxInvulnerables;
	type KickThreshold = Period;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type ValidatorRegistration = Session;
	//type WeightInfo = weights::pallet_collator_selection::WeightInfo<Runtime>;
	type WeightInfo = ();
}

pub type XcmRouter = crate::ParachainXcmRouter<MsgQueue>;
pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Everything;
	type Weigher = WeightInfoBounds<
		weights::xcm::StatemineXcmWeight<RuntimeCall>,
		RuntimeCall,
		MaxInstructions,
	>;

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

pub type XcmOriginToTransactDispatchOrigin = (
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	ParentAsSuperuser<RuntimeOrigin>,
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	XcmPassthrough<RuntimeOrigin>,
);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = AssetTransactors;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = ();
	type IsTeleporter = NativeAsset;
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher = WeightInfoBounds<
		weights::xcm::StatemineXcmWeight<RuntimeCall>,
		RuntimeCall,
		MaxInstructions,
	>;
	type Trader = (
		UsingComponents<WeightToFee, KsmLocation, AccountId, Balances, ToStakingPot<Runtime>>,
		cumulus_primitives_utility::TakeFirstAssetTrader<
			AccountId,
			AssetFeeAsExistentialDepositMultiplier<
				Runtime,
				WeightToFee,
				pallet_assets::BalanceToAssetBalance<Balances, Runtime, ConvertInto>,
			>,
			ConvertedConcreteAssetId<
				AssetId,
				Balance,
				AsPrefixedGeneralIndex<AssetsPalletLocation, AssetId, JustTry>,
				JustTry,
			>,
			Assets,
			cumulus_primitives_utility::XcmFeesTo32ByteAccount<
				FungiblesTransactor,
				AccountId,
				XcmAssetFeesReceiver,
			>,
		>,
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

		// Collator support
		CollatorSelection: pallet_collator_selection::{Pallet, Call, Storage, Event<T>, Config<T>} = 21,

		// XCM helpers
		PolkadotXcm: pallet_xcm::{Pallet, Call, Event<T>, Origin} = 31,
		CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin} = 32,

		Assets: pallet_assets::{Pallet, Call, Storage, Event<T>} = 50,
	}
);

#[allow(dead_code)]
pub(crate) fn check_account() -> AccountId {
	PolkadotXcm::check_account()
}

#[allow(dead_code)]
pub(crate) fn sovereign_account(para_id: u32) -> AccountId {
	LocationToAccountId::convert_ref(MultiLocation::new(1, X1(Parachain(para_id)))).unwrap()
}

mod weights {
	pub(crate) mod frame_system {
		use frame_support::{traits::Get, weights::Weight};
		use sp_std::marker::PhantomData;

		/// Weight functions for `frame_system`.
		pub struct WeightInfo<T>(PhantomData<T>);
		impl<T: frame_system::Config> frame_system::WeightInfo for WeightInfo<T> {
			/// The range of component `b` is `[0, 3932160]`.
			fn remark(b: u32) -> Weight {
				// Minimum execution time: 3_919 nanoseconds.
				Weight::from_ref_time(3_976_000)
					// Standard Error: 0
					.saturating_add(Weight::from_ref_time(411).saturating_mul(b.into()))
			}
			/// The range of component `b` is `[0, 3932160]`.
			fn remark_with_event(b: u32) -> Weight {
				// Minimum execution time: 13_679 nanoseconds.
				Weight::from_ref_time(13_807_000)
					// Standard Error: 0
					.saturating_add(Weight::from_ref_time(1_770).saturating_mul(b.into()))
			}
			// Storage: System Digest (r:1 w:1)
			// Storage: unknown [0x3a686561707061676573] (r:0 w:1)
			fn set_heap_pages() -> Weight {
				// Minimum execution time: 8_917 nanoseconds.
				Weight::from_ref_time(9_108_000)
					.saturating_add(T::DbWeight::get().reads(1))
					.saturating_add(T::DbWeight::get().writes(2))
			}
			// Storage: Skipped Metadata (r:0 w:0)
			/// The range of component `i` is `[0, 1000]`.
			fn set_storage(i: u32) -> Weight {
				// Minimum execution time: 4_078 nanoseconds.
				Weight::from_ref_time(4_134_000)
					// Standard Error: 2_191
					.saturating_add(Weight::from_ref_time(624_841).saturating_mul(i.into()))
					.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
			}
			// Storage: Skipped Metadata (r:0 w:0)
			/// The range of component `i` is `[0, 1000]`.
			fn kill_storage(i: u32) -> Weight {
				// Minimum execution time: 4_078 nanoseconds.
				Weight::from_ref_time(4_149_000)
					// Standard Error: 965
					.saturating_add(Weight::from_ref_time(446_865).saturating_mul(i.into()))
					.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
			}
			// Storage: Skipped Metadata (r:0 w:0)
			/// The range of component `p` is `[0, 1000]`.
			fn kill_prefix(p: u32) -> Weight {
				// Minimum execution time: 5_538 nanoseconds.
				Weight::from_ref_time(5_728_000)
					// Standard Error: 1_272
					.saturating_add(Weight::from_ref_time(972_809).saturating_mul(p.into()))
					.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			}
		}
	}

	pub(crate) mod pallet_balances {
		use frame_support::{traits::Get, weights::Weight};
		use sp_std::marker::PhantomData;

		/// Weight functions for `pallet_balances`.
		pub struct WeightInfo<T>(PhantomData<T>);
		impl<T: frame_system::Config> pallet_balances::WeightInfo for WeightInfo<T> {
			// Storage: System Account (r:1 w:1)
			fn transfer() -> Weight {
				// Minimum execution time: 41_318 nanoseconds.
				Weight::from_ref_time(41_955_000)
					.saturating_add(T::DbWeight::get().reads(1))
					.saturating_add(T::DbWeight::get().writes(1))
			}
			// Storage: System Account (r:1 w:1)
			fn transfer_keep_alive() -> Weight {
				// Minimum execution time: 31_411 nanoseconds.
				Weight::from_ref_time(32_017_000)
					.saturating_add(T::DbWeight::get().reads(1))
					.saturating_add(T::DbWeight::get().writes(1))
			}
			// Storage: System Account (r:1 w:1)
			fn set_balance_creating() -> Weight {
				// Minimum execution time: 22_832 nanoseconds.
				Weight::from_ref_time(23_419_000)
					.saturating_add(T::DbWeight::get().reads(1))
					.saturating_add(T::DbWeight::get().writes(1))
			}
			// Storage: System Account (r:1 w:1)
			fn set_balance_killing() -> Weight {
				// Minimum execution time: 26_415 nanoseconds.
				Weight::from_ref_time(26_818_000)
					.saturating_add(T::DbWeight::get().reads(1))
					.saturating_add(T::DbWeight::get().writes(1))
			}
			// Storage: System Account (r:2 w:2)
			fn force_transfer() -> Weight {
				// Minimum execution time: 41_662 nanoseconds.
				Weight::from_ref_time(42_609_000)
					.saturating_add(T::DbWeight::get().reads(2))
					.saturating_add(T::DbWeight::get().writes(2))
			}
			// Storage: System Account (r:1 w:1)
			fn transfer_all() -> Weight {
				// Minimum execution time: 37_061 nanoseconds.
				Weight::from_ref_time(37_705_000)
					.saturating_add(T::DbWeight::get().reads(1))
					.saturating_add(T::DbWeight::get().writes(1))
			}
			// Storage: System Account (r:1 w:1)
			fn force_unreserve() -> Weight {
				// Minimum execution time: 20_666 nanoseconds.
				Weight::from_ref_time(20_929_000)
					.saturating_add(T::DbWeight::get().reads(1))
					.saturating_add(T::DbWeight::get().writes(1))
			}
		}
	}

	pub(crate) mod pallet_collator_selection {
		use frame_support::{traits::Get, weights::Weight};
		use sp_std::marker::PhantomData;

		/// Weight functions for `pallet_collator_selection`.
		pub struct WeightInfo<T>(PhantomData<T>);
		impl<T: frame_system::Config> pallet_collator_selection::WeightInfo for WeightInfo<T> {
			// Storage: Session NextKeys (r:1 w:0)
			// Storage: CollatorSelection Invulnerables (r:0 w:1)
			/// The range of component `b` is `[1, 100]`.
			fn set_invulnerables(b: u32) -> Weight {
				// Minimum execution time: 21_009 nanoseconds.
				Weight::from_ref_time(22_782_519)
					// Standard Error: 3_817
					.saturating_add(Weight::from_ref_time(2_244_637).saturating_mul(b.into()))
					.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b.into())))
					.saturating_add(T::DbWeight::get().writes(1))
			}
			// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
			fn set_desired_candidates() -> Weight {
				// Minimum execution time: 13_625 nanoseconds.
				Weight::from_ref_time(14_070_000).saturating_add(T::DbWeight::get().writes(1))
			}
			// Storage: CollatorSelection CandidacyBond (r:0 w:1)
			fn set_candidacy_bond() -> Weight {
				// Minimum execution time: 13_691 nanoseconds.
				Weight::from_ref_time(14_032_000).saturating_add(T::DbWeight::get().writes(1))
			}
			// Storage: CollatorSelection Candidates (r:1 w:1)
			// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
			// Storage: CollatorSelection Invulnerables (r:1 w:0)
			// Storage: Session NextKeys (r:1 w:0)
			// Storage: CollatorSelection CandidacyBond (r:1 w:0)
			// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
			/// The range of component `c` is `[1, 999]`.
			fn register_as_candidate(c: u32) -> Weight {
				// Minimum execution time: 44_294 nanoseconds.
				Weight::from_ref_time(41_574_350)
					// Standard Error: 1_029
					.saturating_add(Weight::from_ref_time(132_223).saturating_mul(c.into()))
					.saturating_add(T::DbWeight::get().reads(5))
					.saturating_add(T::DbWeight::get().writes(2))
			}
			// Storage: CollatorSelection Candidates (r:1 w:1)
			// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
			/// The range of component `c` is `[6, 1000]`.
			fn leave_intent(c: u32) -> Weight {
				// Minimum execution time: 34_163 nanoseconds.
				Weight::from_ref_time(28_470_095)
					// Standard Error: 1_039
					.saturating_add(Weight::from_ref_time(126_663).saturating_mul(c.into()))
					.saturating_add(T::DbWeight::get().reads(1))
					.saturating_add(T::DbWeight::get().writes(2))
			}
			// Storage: System Account (r:2 w:2)
			// Storage: System BlockWeight (r:1 w:1)
			// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
			fn note_author() -> Weight {
				// Minimum execution time: 30_143 nanoseconds.
				Weight::from_ref_time(30_631_000)
					.saturating_add(T::DbWeight::get().reads(3))
					.saturating_add(T::DbWeight::get().writes(4))
			}
			// Storage: CollatorSelection Candidates (r:1 w:1)
			// Storage: CollatorSelection LastAuthoredBlock (r:1000 w:1)
			// Storage: System Account (r:1 w:1)
			// Storage: CollatorSelection Invulnerables (r:1 w:0)
			// Storage: System BlockWeight (r:1 w:1)
			/// The range of component `r` is `[1, 1000]`.
			/// The range of component `c` is `[1, 1000]`.
			fn new_session(_r: u32, c: u32) -> Weight {
				// Minimum execution time: 19_764 nanoseconds.
				Weight::from_ref_time(20_011_000)
					// Standard Error: 764_093
					.saturating_add(Weight::from_ref_time(27_541_884).saturating_mul(c.into()))
					.saturating_add(T::DbWeight::get().reads(4))
					.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
					.saturating_add(T::DbWeight::get().writes(1))
					.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			}
		}
	}

	pub(crate) mod xcm {

		use super::super::Runtime;
		use frame_support::weights::Weight;
		use pallet_xcm_benchmarks_fungible::WeightInfo as XcmFungibleWeight;
		use pallet_xcm_benchmarks_generic::WeightInfo as XcmGeneric;
		use sp_std::{cmp, prelude::*};
		use xcm::{
			latest::{prelude::*, Weight as XCMWeight},
			DoubleEncoded,
		};

		trait WeighMultiAssets {
			fn weigh_multi_assets(&self, weight: Weight) -> XCMWeight;
		}

		const MAX_ASSETS: u32 = 100;

		impl WeighMultiAssets for MultiAssetFilter {
			fn weigh_multi_assets(&self, weight: Weight) -> XCMWeight {
				let weight = match self {
					Self::Definite(assets) =>
						weight.saturating_mul(assets.inner().into_iter().count() as u64),
					Self::Wild(_) => weight.saturating_mul(MAX_ASSETS as u64),
				};
				weight.ref_time()
			}
		}

		impl WeighMultiAssets for MultiAssets {
			fn weigh_multi_assets(&self, weight: Weight) -> XCMWeight {
				weight.saturating_mul(self.inner().into_iter().count() as u64).ref_time()
			}
		}

		pub struct StatemineXcmWeight<Call>(core::marker::PhantomData<Call>);
		impl<Call> XcmWeightInfo<Call> for StatemineXcmWeight<Call> {
			fn withdraw_asset(assets: &MultiAssets) -> XCMWeight {
				assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::withdraw_asset())
			}
			// Currently there is no trusted reserve
			fn reserve_asset_deposited(_assets: &MultiAssets) -> XCMWeight {
				u64::MAX
			}
			fn receive_teleported_asset(assets: &MultiAssets) -> XCMWeight {
				assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::receive_teleported_asset())
			}
			fn query_response(
				_query_id: &u64,
				_response: &Response,
				_max_weight: &u64,
			) -> XCMWeight {
				XcmGeneric::<Runtime>::query_response().ref_time()
			}
			fn transfer_asset(assets: &MultiAssets, _dest: &MultiLocation) -> XCMWeight {
				assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::transfer_asset())
			}
			fn transfer_reserve_asset(
				assets: &MultiAssets,
				_dest: &MultiLocation,
				_xcm: &Xcm<()>,
			) -> XCMWeight {
				assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::transfer_reserve_asset())
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
				_max_assets: &u32,
				_dest: &MultiLocation,
			) -> XCMWeight {
				// Hardcoded till the XCM pallet is fixed
				let hardcoded_weight = Weight::from_ref_time(1_000_000_000 as u64).ref_time();
				let weight =
					assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::deposit_asset());
				cmp::min(hardcoded_weight, weight)
			}
			fn deposit_reserve_asset(
				assets: &MultiAssetFilter,
				_max_assets: &u32,
				_dest: &MultiLocation,
				_xcm: &Xcm<()>,
			) -> XCMWeight {
				assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::deposit_reserve_asset())
			}
			fn exchange_asset(_give: &MultiAssetFilter, _receive: &MultiAssets) -> XCMWeight {
				Weight::MAX.ref_time()
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
				// Hardcoded till the XCM pallet is fixed
				let hardcoded_weight = Weight::from_ref_time(200_000_000 as u64).ref_time();
				let weight =
					assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::initiate_teleport());
				cmp::min(hardcoded_weight, weight)
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
					Weight::from_ref_time(32_154_000 as u64)
						.saturating_add(T::DbWeight::get().reads(1 as u64))
						.saturating_add(T::DbWeight::get().writes(1 as u64))
				}
				// Storage: System Account (r:2 w:2)
				pub(crate) fn transfer_asset() -> Weight {
					Weight::from_ref_time(37_328_000 as u64)
						.saturating_add(T::DbWeight::get().reads(2 as u64))
						.saturating_add(T::DbWeight::get().writes(2 as u64))
				}
				// Storage: System Account (r:2 w:2)
				// Storage: ParachainInfo ParachainId (r:1 w:0)
				// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
				// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
				// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
				// Storage: ParachainSystem HostConfiguration (r:1 w:0)
				// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
				pub(crate) fn transfer_reserve_asset() -> Weight {
					Weight::from_ref_time(53_253_000 as u64)
						.saturating_add(T::DbWeight::get().reads(8 as u64))
						.saturating_add(T::DbWeight::get().writes(4 as u64))
				}
				pub(crate) fn receive_teleported_asset() -> Weight {
					Weight::from_ref_time(6_378_000 as u64)
				}
				// Storage: System Account (r:1 w:1)
				pub(crate) fn deposit_asset() -> Weight {
					Weight::from_ref_time(33_783_000 as u64)
						.saturating_add(T::DbWeight::get().reads(1 as u64))
						.saturating_add(T::DbWeight::get().writes(1 as u64))
				}
				// Storage: System Account (r:1 w:1)
				// Storage: ParachainInfo ParachainId (r:1 w:0)
				// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
				// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
				// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
				// Storage: ParachainSystem HostConfiguration (r:1 w:0)
				// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
				pub(crate) fn deposit_reserve_asset() -> Weight {
					Weight::from_ref_time(51_293_000 as u64)
						.saturating_add(T::DbWeight::get().reads(7 as u64))
						.saturating_add(T::DbWeight::get().writes(3 as u64))
				}
				// Storage: ParachainInfo ParachainId (r:1 w:0)
				// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
				// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
				// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
				// Storage: ParachainSystem HostConfiguration (r:1 w:0)
				// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
				pub(crate) fn initiate_teleport() -> Weight {
					Weight::from_ref_time(28_390_000 as u64)
						.saturating_add(T::DbWeight::get().reads(6 as u64))
						.saturating_add(T::DbWeight::get().writes(2 as u64))
				}
			}
		}

		mod pallet_xcm_benchmarks_generic {
			use frame_support::{traits::Get, weights::Weight};
			use sp_std::marker::PhantomData;

			/// Weights for `pallet_xcm_benchmarks::generic`.
			pub struct WeightInfo<T>(PhantomData<T>);
			impl<T: frame_system::Config> WeightInfo<T> {
				// Storage: ParachainInfo ParachainId (r:1 w:0)
				// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
				// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
				// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
				// Storage: ParachainSystem HostConfiguration (r:1 w:0)
				// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
				pub(crate) fn query_holding() -> Weight {
					Weight::from_ref_time(892_211_000 as u64)
						.saturating_add(T::DbWeight::get().reads(6 as u64))
						.saturating_add(T::DbWeight::get().writes(2 as u64))
				}
				pub(crate) fn buy_execution() -> Weight {
					Weight::from_ref_time(8_728_000 as u64)
				}
				// Storage: PolkadotXcm Queries (r:1 w:0)
				pub(crate) fn query_response() -> Weight {
					Weight::from_ref_time(16_766_000 as u64)
						.saturating_add(T::DbWeight::get().reads(1 as u64))
				}
				pub(crate) fn transact() -> Weight {
					Weight::from_ref_time(19_546_000 as u64)
				}
				pub(crate) fn refund_surplus() -> Weight {
					Weight::from_ref_time(8_907_000 as u64)
				}
				pub(crate) fn set_error_handler() -> Weight {
					Weight::from_ref_time(5_393_000 as u64)
				}
				pub(crate) fn set_appendix() -> Weight {
					Weight::from_ref_time(5_453_000 as u64)
				}
				pub(crate) fn clear_error() -> Weight {
					Weight::from_ref_time(5_417_000 as u64)
				}
				pub(crate) fn descend_origin() -> Weight {
					Weight::from_ref_time(6_700_000 as u64)
				}
				pub(crate) fn clear_origin() -> Weight {
					Weight::from_ref_time(5_365_000 as u64)
				}
				// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
				// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
				// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
				// Storage: ParachainSystem HostConfiguration (r:1 w:0)
				// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
				pub(crate) fn report_error() -> Weight {
					Weight::from_ref_time(15_258_000 as u64)
						.saturating_add(T::DbWeight::get().reads(5 as u64))
						.saturating_add(T::DbWeight::get().writes(2 as u64))
				}
				// Storage: PolkadotXcm AssetTraps (r:1 w:1)
				pub(crate) fn claim_asset() -> Weight {
					Weight::from_ref_time(21_485_000 as u64)
						.saturating_add(T::DbWeight::get().reads(1 as u64))
						.saturating_add(T::DbWeight::get().writes(1 as u64))
				}
				pub(crate) fn trap() -> Weight {
					Weight::from_ref_time(5_334_000 as u64)
				}
				// Storage: PolkadotXcm VersionNotifyTargets (r:1 w:1)
				// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
				// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
				// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
				// Storage: ParachainSystem HostConfiguration (r:1 w:0)
				// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
				pub(crate) fn subscribe_version() -> Weight {
					Weight::from_ref_time(18_035_000 as u64)
						.saturating_add(T::DbWeight::get().reads(6 as u64))
						.saturating_add(T::DbWeight::get().writes(3 as u64))
				}
				// Storage: PolkadotXcm VersionNotifyTargets (r:0 w:1)
				pub(crate) fn unsubscribe_version() -> Weight {
					Weight::from_ref_time(7_661_000 as u64)
						.saturating_add(T::DbWeight::get().writes(1 as u64))
				}
				// Storage: ParachainInfo ParachainId (r:1 w:0)
				// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
				// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
				// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
				// Storage: ParachainSystem HostConfiguration (r:1 w:0)
				// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
				pub(crate) fn initiate_reserve_withdraw() -> Weight {
					Weight::from_ref_time(1_090_619_000 as u64)
						.saturating_add(T::DbWeight::get().reads(6 as u64))
						.saturating_add(T::DbWeight::get().writes(2 as u64))
				}
			}
		}
	}
}
