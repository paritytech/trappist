// This file is part of Trappist.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cumulus_primitives_core::ParaId;
use frame_support::{
	match_types, parameter_types,
	traits::{Contains, ContainsPair, EitherOfDiverse, Everything, Get, Nothing, PalletInfoAccess},
	weights::Weight,
};
use frame_system::EnsureRoot;
use pallet_xcm::{EnsureXcm, IsMajorityOfBody, XcmPassthrough};
use parachains_common::AssetIdForTrustBackedAssets;
use polkadot_parachain_primitives::primitives::Sibling;
use polkadot_runtime_common::xcm_sender::{ExponentialPrice, NoPriceForMessageDelivery};
use sp_core::ConstU32;
use sp_std::{marker::PhantomData, vec::Vec};
use xcm::latest::{prelude::*, Fungibility::Fungible, MultiAsset, MultiLocation};
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, CurrencyAdapter,
	DenyReserveTransferToRelayChain, DenyThenTry, DescribeAllTerminal, DescribeFamily,
	EnsureXcmOrigin, FixedRateOfFungible, FungiblesAdapter, HashedDescription, IsConcrete,
	MintLocation, NativeAsset, NoChecking, ParentAsSuperuser, ParentIsPreset, RelayChainAsNative,
	SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit, TrailingSetTopicAsId,
	UsingComponents, WeightInfoBounds, WithComputedOrigin,
};
use xcm_executor::{traits::JustTry, XcmExecutor};

use xcm_primitives::{AsAssetMultiLocation, ConvertedRegisteredAssetId, TrappistDropAssets};

use crate::{
	constants::fee::{default_fee_per_second, WeightToFee},
	impls::ToAuthor,
	weights,
	weights::TrappistDropAssetsWeigher,
	TransactionByteFee, CENTS,
};

use super::{
	AccountId, AllPalletsWithSystem, AssetRegistry, Assets, Balance, Balances, ParachainInfo,
	ParachainSystem, PolkadotXcm, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin, XcmpQueue,
};

parameter_types! {
	pub const RelayLocation: MultiLocation = MultiLocation::parent();
	pub const RelayNetwork: NetworkId = NetworkId::Rococo;
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
	pub SelfReserve: MultiLocation = MultiLocation::here();
	pub AssetsPalletLocation: MultiLocation =
		PalletInstance(<Assets as PalletInfoAccess>::index() as u8).into();
	// Be mindful with incoming teleports if you implement this
	pub CheckAccount: (AccountId, MintLocation) = (PolkadotXcm::check_account(), MintLocation::Local);
	pub PlaceholderAccount: AccountId = PolkadotXcm::check_account();
	pub const ExecutiveBody: BodyId = BodyId::Executive;
	pub const MaxAssetsIntoHolding: u32 = 64;
	pub UniversalLocation: InteriorMultiLocation = (
		GlobalConsensus(NetworkId::Rococo),
		Parachain(ParachainInfo::parachain_id().into()),
	).into();
}

/// We allow root and the Relay Chain council to execute privileged collator selection operations.
pub type CollatorSelectionUpdateOrigin = EitherOfDiverse<
	EnsureRoot<AccountId>,
	EnsureXcm<IsMajorityOfBody<RelayLocation, ExecutiveBody>>,
>;

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the parent `AccountId`.
	ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
	// Foreign locations alias into accounts according to a hash of their standard description.
	HashedDescription<AccountId, DescribeFamily<DescribeAllTerminal>>,
);

/// `AssetId/Balancer` converter for `TrustBackedAssets`
pub type TrustBackedAssetsConvertedConcreteId =
	assets_common::TrustBackedAssetsConvertedConcreteId<AssetsPalletLocation, Balance>;

/// Means for transacting the native currency on this chain.
pub type LocalAssetTransactor = CurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<SelfReserve>,
	// Convert an XCM MultiLocation into a local account id:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports.
	(),
>;

/// Means for transacting assets besides the native currency on this chain.
pub type LocalFungiblesTransactor = FungiblesAdapter<
	// Use this fungibles implementation:
	Assets,
	// Use this currency when it is a fungible asset matching the given location or name:
	TrustBackedAssetsConvertedConcreteId,
	// Convert an XCM MultiLocation into a local account id:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports of `Assets`.
	NoChecking,
	// We don't track any teleports of `Assets`, but a placeholder account is provided due to trait
	// bounds.
	PlaceholderAccount,
>;

/// Means for transacting reserved fungible assets.
/// AsAssetMultiLocation uses pallet_asset_registry to convert between AssetId and MultiLocation.
pub type ReservedFungiblesTransactor = FungiblesAdapter<
	// Use this fungibles implementation:
	Assets,
	// Use this currency when it is a registered fungible asset matching the given location or name
	// Assets not found in AssetRegistry will not be used
	ConvertedRegisteredAssetId<
		AssetIdForTrustBackedAssets,
		Balance,
		AsAssetMultiLocation<AssetIdForTrustBackedAssets, AssetRegistry>,
		JustTry,
	>,
	// Convert an XCM MultiLocation into a local account id:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports of `Assets`.
	NoChecking,
	// We don't track any teleports of `Assets`, but a placeholder account is provided due to trait
	// bounds.
	PlaceholderAccount,
>;

/// Means for transacting assets on this chain.
pub type AssetTransactors =
	(LocalAssetTransactor, ReservedFungiblesTransactor, LocalFungiblesTransactor);

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will convert to a `Relay` origin when
	// recognised.
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognised.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Superuser converter for the Relay-chain (Parent) location. This will allow it to issue a
	// transaction from the Root origin.
	ParentAsSuperuser<RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
	/// The asset ID for the asset that we use to pay for message delivery fees.
	pub FeeAssetId: AssetId = Concrete(RelayLocation::get());
	/// The base fee for the message delivery fees.
	pub const BaseDeliveryFee: u128 = CENTS.saturating_mul(3);
}

pub type PriceForParentDelivery =
	ExponentialPrice<FeeAssetId, BaseDeliveryFee, TransactionByteFee, ParachainSystem>;

parameter_types! {
	// One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
	pub UnitWeightCost: Weight = Weight::from_parts(1_000_000_000u64, 0);
	pub const MaxInstructions: u32 = 100;
}

match_types! {
	pub type ParentOrParentsExecutivePlurality: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Executive, .. }) }
	};
}
match_types! {
	pub type ParentOrSiblings: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(_) }
	};
}
match_types! {
	pub type AssetHub: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: X1(Parachain(1000)) }
	};
}

pub type Barrier = TrailingSetTopicAsId<
	DenyThenTry<
		DenyReserveTransferToRelayChain,
		(
			TakeWeightCredit,
			// Expected responses are OK.
			AllowKnownQueryResponses<PolkadotXcm>,
			// Allow XCMs with some computed origins to pass through.
			WithComputedOrigin<
				(
					// If the message is one that immediately attemps to pay for execution, then
					// allow it.
					AllowTopLevelPaidExecutionFrom<Everything>,
					// Parent, its pluralities (i.e. governance bodies), and the Fellows plurality
					// get free execution.
					AllowUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
					// Subscriptions for version tracking are OK.
					AllowSubscriptionsFrom<ParentOrSiblings>,
				),
				UniversalLocation,
				ConstU32<8>,
			>,
		),
	>,
>;

parameter_types! {
	pub AssetHubLocation: MultiLocation = MultiLocation::new(1, X1(Parachain(1000)));
	// ALWAYS ensure that the index in PalletInstance stays up-to-date with
	// AssetHub's Assets pallet index
	pub AssetHubAssetsPalletLocation: MultiLocation =
		MultiLocation::new(1, X2(Parachain(1000), PalletInstance(50)));
	pub RUsdPerSecond: (xcm::v3::AssetId, u128, u128) = (
		MultiLocation::new(1, X3(Parachain(1000), PalletInstance(50), GeneralIndex(1984))).into(),
		default_fee_per_second() * 10,
		0u128
	);
	/// Roc = 7 RUSD
	pub RocPerSecond: (xcm::v3::AssetId, u128,u128) = (MultiLocation::new(1,Here).into(), default_fee_per_second() * 70, 0u128);
}

parameter_types! {
	pub const TrappistNative: MultiAssetFilter = Wild(AllOf { fun: WildFungible, id: Concrete(MultiLocation::here()) });
	pub AssetHubTrustedTeleporter: (MultiAssetFilter, MultiLocation) = (TrappistNative::get(), AssetHubLocation::get());
}

pub struct ReserveAssetsFrom<T>(PhantomData<T>);
impl<T: Get<MultiLocation>> ContainsPair<MultiAsset, MultiLocation> for ReserveAssetsFrom<T> {
	fn contains(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		let prefix = T::get();
		log::trace!(target: "xcm::AssetsFrom", "prefix: {:?}, origin: {:?}, asset: {:?}", prefix, origin, asset);
		&prefix == origin
	}
}

pub struct OnlyTeleportNative;
impl Contains<(MultiLocation, Vec<MultiAsset>)> for OnlyTeleportNative {
	fn contains(t: &(MultiLocation, Vec<MultiAsset>)) -> bool {
		t.1.iter().any(|asset| {
			log::trace!(target: "xcm::OnlyTeleportNative", "Asset to be teleported: {:?}", asset);

			if let MultiAsset { id: xcm::latest::AssetId::Concrete(asset_loc), fun: Fungible(_a) } =
				asset
			{
				match asset_loc {
					MultiLocation { parents: 0, interior: Here } => true,
					_ => false,
				}
			} else {
				false
			}
		})
	}
}

pub type Traders = (
	// RUSD
	FixedRateOfFungible<RUsdPerSecond, ()>,
	// Roc
	FixedRateOfFungible<RocPerSecond, ()>,
	// Everything else
	UsingComponents<WeightToFee, SelfReserve, AccountId, Balances, ToAuthor<Runtime>>,
);

pub type Reserves = (NativeAsset, ReserveAssetsFrom<AssetHubLocation>);
pub type TrustedTeleporters = (xcm_builder::Case<AssetHubTrustedTeleporter>,);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = AssetTransactors;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = Reserves;
	type IsTeleporter = TrustedTeleporters;
	type Aliasers = ();
	type UniversalLocation = UniversalLocation;
	type Barrier = Barrier;
	type Weigher = WeightInfoBounds<
		crate::weights::xcm::TrappistXcmWeight<RuntimeCall>,
		RuntimeCall,
		MaxInstructions,
	>;
	type Trader = Traders;
	type ResponseHandler = PolkadotXcm;
	type AssetTrap = TrappistDropAssets<
		AssetIdForTrustBackedAssets,
		AssetRegistry,
		Assets,
		Balances,
		PolkadotXcm,
		AccountId,
		TrappistDropAssetsWeigher,
	>;
	type AssetLocker = ();
	type AssetExchanger = ();
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
	type PalletInstancesInfo = AllPalletsWithSystem;
	type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
	type FeeManager = ();
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
	type SafeCallFilter = Everything;
}

/// Converts a local signed origin into an XCM multilocation.
/// Forms the basis for local origins sending/executing XCMs.
pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm, ()>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
	pub ReachableDest: Option<MultiLocation> = Some(Parent.into());
}

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Everything;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	//Only teleport of HOP is allowed
	type XcmTeleportFilter = OnlyTeleportNative;
	type XcmReserveTransferFilter = Everything;
	type Weigher = WeightInfoBounds<
		crate::weights::xcm::TrappistXcmWeight<RuntimeCall>,
		RuntimeCall,
		MaxInstructions,
	>;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type MaxLockers = ConstU32<8>;
	type SovereignAccountOf = LocationToAccountId;
	type TrustedLockers = ();
	type UniversalLocation = UniversalLocation;
	// TODO: pallet-xcm weights
	type WeightInfo = pallet_xcm::TestWeightInfo;
	#[cfg(feature = "runtime-benchmarks")]
	type ReachableDest = ReachableDest;
	type AdminOrigin = EnsureRoot<AccountId>;
	type MaxRemoteLockConsumers = ConstU32<0>;
	type RemoteLockConsumerIdentifier = ();
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ChannelInfo = ParachainSystem;
	type VersionWrapper = PolkadotXcm;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type ControllerOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		EnsureXcm<IsMajorityOfBody<RelayLocation, ExecutiveBody>>,
	>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = weights::cumulus_pallet_xcmp_queue::WeightInfo<Runtime>;
	type PriceForSiblingDelivery = NoPriceForMessageDelivery<ParaId>;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}
