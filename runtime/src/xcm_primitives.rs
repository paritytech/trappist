use sp_std::{marker::PhantomData, prelude::*};
use xcm::latest::prelude::*;
use xcm_executor::traits::FilterAssetLocation;

// From orml-traits
// See https://github.com/open-web3-stack/open-runtime-module-library/blob/c439a50e01944aedeef33231e0824a17ed1813bc/traits/src/location.rs
pub trait Parse {
	/// Returns the "chain" location part. It could be parent, sibling
	/// parachain, or child parachain.
	fn chain_part(&self) -> Option<MultiLocation>;
	/// Returns "non-chain" location part.
	fn non_chain_part(&self) -> Option<MultiLocation>;
}

fn is_chain_junction(junction: Option<&Junction>) -> bool {
	matches!(junction, Some(Parachain(_)))
}

impl Parse for MultiLocation {
	fn chain_part(&self) -> Option<MultiLocation> {
		match (self.parents, self.first_interior()) {
			// sibling parachain
			(1, Some(Parachain(id))) => Some(MultiLocation::new(1, X1(Parachain(*id)))),
			// parent
			(1, _) => Some(MultiLocation::parent()),
			// children parachain
			(0, Some(Parachain(id))) => Some(MultiLocation::new(0, X1(Parachain(*id)))),
			_ => None,
		}
	}

	fn non_chain_part(&self) -> Option<MultiLocation> {
		let mut junctions = self.interior().clone();
		while is_chain_junction(junctions.first()) {
			let _ = junctions.take_first();
		}

		if junctions != Here {
			Some(MultiLocation::new(0, junctions))
		} else {
			None
		}
	}
}

pub trait Reserve {
	/// Returns assets reserve location.
	fn reserve(asset: &MultiAsset) -> Option<MultiLocation>;
}

// Provide reserve in absolute path view
pub struct AbsoluteReserveProvider;

impl Reserve for AbsoluteReserveProvider {
	fn reserve(asset: &MultiAsset) -> Option<MultiLocation> {
		if let Concrete(location) = &asset.id {
			location.chain_part()
		} else {
			None
		}
	}
}

// From orml-support
// See https://github.com/open-web3-stack/open-runtime-module-library/blob/c439a50e01944aedeef33231e0824a17ed1813bc/xcm-support/src/lib.rs
pub struct MultiNativeAsset<ReserveProvider>(PhantomData<ReserveProvider>);
impl<ReserveProvider> FilterAssetLocation for MultiNativeAsset<ReserveProvider>
where
	ReserveProvider: Reserve,
{
	fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		if let Some(ref reserve) = ReserveProvider::reserve(asset) {
			if reserve == origin {
				return true;
			}
		}
		false
	}
}
