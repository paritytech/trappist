use crate::Weight;

pub trait WeightInfo {
	fn drop_assets_fungible() -> Weight;
	fn drop_assets_native() -> Weight;
	fn drop_assets_default() -> Weight;
}
