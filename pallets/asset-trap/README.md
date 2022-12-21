# Asset Trap Pallet

## Overview

Note: the name of this pallet has **nothing** to do with the name of the project (Trappist). They just sound alike as a coincidence.

In the [third blogpost of his XCM series](https://medium.com/polkadot-network/xcm-part-iii-execution-and-error-management-ceb8155dd166), Dr. Gavin Wood describes his vision for an XCM **Asset Trap** & **Claim** system:

> ### 🪤 The Asset Trap
> When errors occur during programs that deal with assets (as most do since they will often need to pay for their execution with `BuyExecution`), then it can be very problematic. There may be instances where the `BuyExecution` instruction itself results in error, perhaps because the weight limit was incorrect or the assets used for payment were insufficient. Or perhaps an asset gets sent to a chain which cannot deal with it in a useful way. In these cases, any many others, the message’s XCVM execution finishes with assets remaining in the Holding Register, which like the other registers are transient and we would expect to be forgotten about.
>
> Teams and their users will be happy to know that Substrate’s XCM allows chains to avoid this loss entirely 🎉. The mechanism works in two steps. First, any assets in the Holding Register when it gets cleared do not get completely forgotten. If the Holding Register is not empty when the XCVM halts, then an event is emitted containing three pieces of information: the value of the Holding Register; the original value of the Origin Register; and the hash of these two pieces of information. Substrate’s XCM system then places this hash in storage. This part of the mechanism is called the Asset Trap.
> 
> ### 🎟 The Claim System
> 
> The second step to the mechanism is being able to claim some previous contents of the Holding Register. This actually happens not through anything specially designed for this purpose but rather through a general purpose instruction that we have not yet met called `ClaimAsset`.

The XCM Executor Config (`XcmConfig`) expects two specific types:
- `AssetTrap`: which must implement the [`DropAssets`](https://github.com/paritytech/polkadot/blob/1a034bd6de0e76721d19aed02a538bcef0787260/xcm/xcm-executor/src/traits/drop_assets.rs#L24) trait.
- `AssetClaims`: which must implement the [`ClaimAssets`](https://github.com/paritytech/polkadot/blob/1a034bd6de0e76721d19aed02a538bcef0787260/xcm/xcm-executor/src/traits/drop_assets.rs#L64) trait.

The [`pallet-xcm`](https://github.com/paritytech/polkadot/tree/master/xcm/pallet-xcm) available on the `polkadot` repository provides a default implementation for both of these traits. It calculates a hash of `(origin, versioned_assets)` and uses this hash as a key for a `StorageMap`, where the value is a counter of how many times these assets have been trapped. For the claiming part, it takes a `ticket` which helps it figure out the version, and if the calculated hash of `(origin, versioned_assets)` (claimer's origin and claimed assets) matches a key on the `StorageMap`, it decreases the counter (or cleans it up) while loading the assets back into the Holding Register.

This default implementation stores all leftover assets into a single trap. If the Holding Register has multiple assets when the XCVM halts, all of them are used in the hash calculation, and written into the same storage item. There's also no on-chain information about the trapped assets, aside from the hash.

The `pallet-asset-trap` provided here provides an alternative approach, arguably more refined. A few design decisions differ from the default implementation, namely:
- If the Holding Register has multiple assets, they are trapped separately into different storage items (and with different hashes).
- The trapped asset's `MultiLocation` is also written into storage.
- The assets to be trapped are checked. Non-compliant assets will not be trapped (they're lost forever). The asset will be trapped if (and only if):
    - It is Trappist's Native Token (`HOP`) OR some derivative asset previously registered in `pallet-asset-registry` (e.g.: `txUSD`).
    - The amount is bigger than the token's minimum balance (a.k.a. existential deposit).

## Configuration

### Types
* `RuntimeEvent` – The overarching event type.
* `Balances` – The balances type implemented by `pallet-balances`.
* `Assets` – The assets type implemented by `pallet-assets`.
* `AssetRegistry` – The asset registry type implemented by `pallet-asset-registry`.

## Extrinsics

This pallet does not provide any extrinsic.

## How to add `pallet-asset-trap` to a runtime

### Runtime's `Cargo.toml`

Add `pallet-asset-trap`, to the dependencies:
```toml
[dependencies.pallet-asset-trap]
version = "0.0.1"
default-features = false
git = "https://github.com/paritytech/trappist.git"
branch = "master"
```

Make sure `pallet-balances`, `pallet-assets`, `pallet-asset-registry` and `xcm-primitives` are also there.

Update the runtime's `std` feature:
```toml
std = [
    # --snip--
    "pallet-asset-trap/std",
    # --snip--
]
```

### Runtime's `lib.rs`

Here's how to configure `pallet-asset-trap`:
```
impl pallet_asset_trap::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Balances = Balances;
    type Assets = Assets;
    type AssetMultiLocationGetter = AssetRegistry;
}
```

Where `Balances` is `pallet_balances`, `Assets` is `pallet_assets`, and `AssetRegistry` is `pallet_asset_registry` on the `construct_runtime` macro call:

```rust
construct_runtime!(
    pub enum Runtime where
        // --snip--
    {
        // --snip---
        System: frame_system,
        Balances: pallet_balances,
        Assets: pallet_assets,
        AssetRegistry: pallet_asset_registry,
        AssetTrap: pallet_asset_trap,
        // --snip---
    }
);
```

### Runtime's `xcm_config.rs`

Make sure the `AssetTrap` and `AssetClaims` types are both set as `AssetTrap` type for `XcmConfig`:
```rust
pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    // snip
    type AssetTrap = AssetTrap;
    type AssetClaims = AssetTrap;
    // snip
}
```