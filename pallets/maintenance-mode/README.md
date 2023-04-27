# Asset Registry Pallet

## Overview

Successful Reserve-based transfers rely on the Runtime having its `xcm_executor::Config` properly set.
More specifically, its `AssetTransactor` type needs a `FungiblesAdapter` with a `ConvertedConcreteAssetId` that is able to convert the foreign `MultiLocation` into a local `AssetId`.

The `asset-registry` pallet provides a solution to this problem by implementing a trait (`AssetMultiLocationGetter<AssetId>`) that converts between `AssetId` and `MultiLocation` (and vice-versa).

This trait is used by a struct (`AsAssetMultiLocation<AssetId, AssetIdInfoGetter>`) that is added to the runtime (as an extra XCM primitive) and used as the `xcm_executor::traits::Convert<MultiLocation, AssetId>` implementor needed by the `ConvertedConcreteAssetId` of `FungiblesAdapter`.

The pallet needs to be used in conjunction with the [`xcm-primitives` crate](https://github.com/paritytech/trappist/tree/master/primitives/xcm) or an equivalent implementation.

## Configuration

### Types
* `Event` – The overarching event type.
* `ReserveAssetModifierOrigin` – The origin that's allowed to register and unregister reserve assets.
* `Assets` – The assets type.

## Extrinsics

<details>
<summary><h3>register_reserve_asset</h3></summary>

Register a new Reserve Asset.

#### Parameters
* `origin` – Origin for the call. Must be signed.
* `asset_id` – ID of the Asset. Asset with this ID must exist on the local `Assets` pallet.
* `asset_multi_location` – `MultiLocation` of the Reserve Asset.

#### Errors
* `AssetDoesNotExist` – The Asset ID does not exist on the local `Assets` pallet.
* `AssetAlreadyRegistered` – The Asset ID is already registered.
* `WrongMultiLocation` – Provided Reserve Asset `MultiLocation` is invalid.

</details>

<details>
<summary><h3>unregister_reserve_asset</h3></summary>

Unregister a Reserve Asset.

#### Parameters
* `origin` – Origin for the call. Must be signed.
* `asset_id` – ID of the asset. Asset with this ID must exist on the local `Assets` pallet.

#### Errors
* `AssetIsNotRegistered` – The Asset ID is not registered, and therefore cannot be unregistered.

</details>

## How to add `pallet-asset-registry` to a runtime

### Runtime's `Cargo.toml`

Add `pallet-assets`, `pallet-asset-registry` and `xcm-primitives` to the dependencies:
```toml
[dependencies.pallet-assets]
version = "4.0.0-dev"
default-features = false
git = "https://github.com/paritytech/substrate.git"
branch = "polkadot-v0.9.37"

[dependencies.pallet-asset-registry]
version = "0.0.1"
default-features = false
git = "https://github.com/paritytech/trappist.git"
branch = "master"

[dependencies.xcm-primitives]
version = "0.1.0"
default-features = false
git = "https://github.com/paritytech/trappist.git"
branch = "master"
```

Update the runtime's `std` feature:
```toml
std = [
    # --snip--
    "pallet-assets/std",
    "pallet-asset-registry/std",
    # --snip--
]
```

### Runtime's `lib.rs`
Configure the `assets` pallet:
```rust
pub type AssetBalance = Balance;
pub type AssetId = u32;

impl pallet_assets::Config for Runtime {
    type Event = Event;
    type Balance = AssetBalance;
    type AssetId = AssetId;
    type AssetIdParameter = u32;
    type Currency = Balances;
    type ForceOrigin = EnsureRoot<AccountId>;
    type AssetDeposit = ConstU128<1>;
    type AssetAccountDeposit = ConstU128<10>;
    type MetadataDepositBase = ConstU128<1>;
    type MetadataDepositPerByte = ConstU128<1>;
    type ApprovalDeposit = ConstU128<1>;
    type StringLimit = ConstU32<50>;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = ();
    type RemoveItemsLimit = ConstU32<5>;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ();
}
```

Configure the `asset-registry` pallet:
```rust
impl pallet_asset_registry::Config for Runtime {
	type Event = Event;
	type ReserveAssetModifierOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type Assets = Assets;
}
```

Add the configured pallets to the `construct_runtime` macro call.
```rust
construct_runtime!(
    pub enum Runtime where
        // --snip--
    {
        // --snip---
        Assets: pallet_assets,
        AssetRegistry: pallet_asset_registry::{Pallet, Call, Storage, Event<T>},
        // --snip---
    }
);
```

### Runtime's `xcm_config.rs`
Add a new `FungiblesAdapter`:
```rust
pub type ReservedFungiblesTransactor = FungiblesAdapter<
	Assets,
	ConvertedConcreteAssetId<
		AssetId,
		Balance,
		AsAssetMultiLocation<AssetId, AssetRegistry>,
		JustTry,
	>,
	LocationToAccountId,
	AccountId,
	Nothing,
	CheckingAccount,
>;
```

Add the new `FungiblesAdapter` to the `AssetTransactors` tuple:
```rust
pub type AssetTransactors = (
    // snip
    ReservedFungiblesTransactor,
    // snip
);
```

Make sure the `AssetTransactors` tuple is set as `AssetTransactor` type for `XcmConfig`:
```rust
pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    // snip
	type AssetTransactor = AssetTransactors;
    // snip
}
```

### Node's `chain_spec.rs`
Add genesis configuration for assets pallet.
```rust
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        // --snip--
        assets: AssetsConfig {
            assets: vec![],
            accounts: vec![],
            metadata: vec![],
        },
    }
}
```
