use super::*;

#[cfg(test)]
mod test_xcm_use_cases {
	use super::*;
	use crate::relay_chain::mock_paras_sudo_wrapper;
	use codec::Encode;
	use frame_support::{
		assert_ok,
		pallet_prelude::{DispatchResult, DispatchResultWithPostInfo},
		traits::PalletInfoAccess,
	};
	use std::sync::Once;
	use thousands::Separable;
	use xcm::prelude::*;
	use xcm_simulator::{TestExt, Weight};

	static INIT: Once = Once::new();
	pub fn init_tracing() {
		INIT.call_once(|| {
			// Add test tracing (from sp_tracing::init_for_tests()) but filtering for xcm logs only
			let _ = tracing_subscriber::fmt()
				.with_max_level(tracing::Level::TRACE)
				.with_env_filter("xcm=trace") // Comment out this line to see all traces
				.with_test_writer()
				.init();
		});
	}

	#[allow(non_upper_case_globals)]
	const xUSD: u32 = 1;
	#[allow(non_upper_case_globals)]
	const txUSD: u32 = 10;
	#[allow(non_upper_case_globals)]
	const pxUSD: u32 = xUSD; // Must match asset reserve identifier as no asset registry available in base runtime

	// Teleports some amount of the native asset of the relay chain to the asset reserve parachain
	// (DMP)
	#[test]
	fn teleport_native_asset_from_relay_chain_to_asset_reserve_parachain() {
		init_tracing();

		MockNet::reset();

		let mut beneficiary_balance = 0;
		let mut total_issuance = 0;

		AssetReserve::execute_with(|| {
			// Check beneficiary balance and total issuance on asset reserve before teleport
			beneficiary_balance = asset_reserve::Balances::free_balance(&ALICE);
			total_issuance = asset_reserve::Balances::total_issuance();
		});

		const AMOUNT: u128 = 1_000_000_000;

		Relay::execute_with(|| {
			// Teleport, ensuring relay chain total issuance remains constant
			let total_issuance = relay_chain::Balances::total_issuance();
			assert_ok!(relay_chain::XcmPallet::teleport_assets(
				relay_chain::RuntimeOrigin::signed(ALICE),
				Box::new(Parachain(ASSET_RESERVE_PARA_ID).into().into()),
				Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
				Box::new((Here, AMOUNT).into()),
				0
			));
			assert_eq!(relay_chain::Balances::total_issuance(), total_issuance);

			// Ensure teleport amount 'checked out' to check account
			assert_eq!(relay_chain::Balances::free_balance(&relay_chain::check_account()), AMOUNT);
			// Ensure sender balance decreased by teleport amount
			assert_eq!(relay_chain::Balances::free_balance(&ALICE), INITIAL_BALANCE - AMOUNT);
		});

		const EST_FEES: u128 = 4_000_000;
		AssetReserve::execute_with(|| {
			// Ensure receiver balance and total issuance increased by teleport amount
			let current_balance = asset_reserve::Balances::free_balance(&ALICE);
			assert_balance(current_balance, beneficiary_balance + AMOUNT, EST_FEES);
			assert_eq!(asset_reserve::Balances::total_issuance(), total_issuance + AMOUNT);

			println!(
				"Teleport: initial balance {} teleport amount {} current balance {} estimated fees {} actual fees {}",
				beneficiary_balance.separate_with_commas(),
				AMOUNT.separate_with_commas(),
				current_balance.separate_with_commas(),
				EST_FEES.separate_with_commas(),
				(beneficiary_balance + AMOUNT - current_balance).separate_with_commas()
			);
		});
	}

	// Teleports some amount of the (shared) native asset of the asset reserve parachain back to the
	// relay chain (UMP)
	#[test]
	fn teleport_native_asset_from_asset_reserve_parachain_to_relay_chain() {
		init_tracing();

		MockNet::reset();

		const AMOUNT: u128 = 1_000_000_000;
		let mut beneficiary_balance = 0;

		Relay::execute_with(|| {
			// Teleport some amount to asset reserve so there are tokens to teleport back
			assert_ok!(relay_chain::XcmPallet::teleport_assets(
				relay_chain::RuntimeOrigin::signed(ALICE),
				Box::new(Parachain(ASSET_RESERVE_PARA_ID).into().into()),
				Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
				Box::new((Here, AMOUNT).into()),
				0
			));

			// Check beneficiary balance
			beneficiary_balance = relay_chain::Balances::free_balance(&ALICE);
		});

		AssetReserve::execute_with(|| {
			// Check sender balance & total issuance of native asset on asset reserve before
			// teleporting
			let sender_balance = asset_reserve::Balances::free_balance(&ALICE);
			let total_issuance = asset_reserve::Balances::total_issuance();
			assert_ok!(asset_reserve::PolkadotXcm::teleport_assets(
				asset_reserve::RuntimeOrigin::signed(ALICE),
				Box::new(Parent.into()),
				Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
				Box::new((Parent, AMOUNT).into()),
				0
			));

			// Ensure sender balance and total issuance (of native asset on asset reserve) decreased
			// by teleport amount
			assert_eq!(asset_reserve::Balances::free_balance(&ALICE), sender_balance - AMOUNT);
			assert_eq!(asset_reserve::Balances::total_issuance(), total_issuance - AMOUNT)
		});

		const EST_FEES: u128 = 2_500_000;
		Relay::execute_with(|| {
			// Ensure receiver balance increased by teleport amount
			let current_balance = relay_chain::Balances::free_balance(&ALICE);
			assert_balance(current_balance, beneficiary_balance + AMOUNT, EST_FEES);
			println!(
				"Teleport: initial balance {} teleport amount {} current balance {} estimated fees {} actual fees {}",
				beneficiary_balance.separate_with_commas(),
				AMOUNT.separate_with_commas(),
				current_balance.separate_with_commas(),
				EST_FEES.separate_with_commas(),
				(beneficiary_balance + AMOUNT - current_balance).separate_with_commas()
			);
		});
	}

	// Initiates a reserve-transfer of some asset on the asset reserve parachain to the trappist
	// parachain (HRMP)
	#[test]
	fn reserve_transfer_asset_from_asset_reserve_parachain_to_trappist_parachain() {
		init_tracing();

		MockNet::reset();

		const ASSET_MIN_BALANCE: asset_reserve::Balance = 1_000_000_000;
		const MINT_AMOUNT: u128 = 1_000_000_000_000_000_000;

		AssetReserve::execute_with(|| {
			// Create fungible asset on Reserve Parachain
			assert_ok!(create_asset_on_asset_reserve(xUSD, ALICE, ASSET_MIN_BALANCE));

			// Mint fungible asset
			assert_ok!(mint_asset_on_asset_reserve(xUSD, ALICE, MINT_AMOUNT));
			assert_eq!(asset_reserve::Assets::balance(xUSD, &ALICE), MINT_AMOUNT);
		});

		Relay::execute_with(|| {
			// Declare xUSD (on Reserve Parachain) as self-sufficient via Relay Chain
			paras_sudo_wrapper_sudo_queue_downward_xcm(asset_reserve::RuntimeCall::Assets(
				pallet_assets::Call::<asset_reserve::Runtime>::force_asset_status {
					id: xUSD,
					owner: ALICE.into(),
					issuer: ALICE.into(),
					admin: ALICE.into(),
					freezer: ALICE.into(),
					min_balance: ASSET_MIN_BALANCE,
					is_sufficient: true,
					is_frozen: false,
				},
			));
		});

		let mut beneficiary_balance = 0;
		Trappist::execute_with(|| {
			// Create derivative asset on Trappist Parachain
			assert_ok!(create_derivative_asset_on_trappist(txUSD, ALICE.into(), ASSET_MIN_BALANCE));

			// Map derivative asset (txUSD) to multi-location (xUSD within Assets pallet on Reserve
			// Parachain) via Asset Registry
			assert_ok!(register_reserve_asset_on_trappist(ALICE, txUSD));
			assert!(trappist::AssetRegistry::asset_id_multilocation(txUSD).is_some());

			// Check beneficiary balance
			beneficiary_balance = trappist::Assets::balance(txUSD, &ALICE);
		});

		const AMOUNT: u128 = 10_000_000_000;

		AssetReserve::execute_with(|| {
			// Reserve parachain should be able to reserve-transfer an asset to Trappist Parachain
			assert_ok!(asset_reserve::PolkadotXcm::limited_reserve_transfer_assets(
				asset_reserve::RuntimeOrigin::signed(ALICE),
				Box::new((Parent, Parachain(TRAPPIST_PARA_ID)).into()),
				Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
				Box::new(
					(
						X2(
							PalletInstance(asset_reserve::Assets::index() as u8),
							GeneralIndex(xUSD as u128)
						),
						AMOUNT
					)
						.into()
				),
				0,
				WeightLimit::Unlimited,
			));

			// Ensure send amount moved to sovereign account
			let sovereign_account = asset_reserve::sovereign_account(TRAPPIST_PARA_ID);
			assert_eq!(asset_reserve::Assets::balance(xUSD, &sovereign_account), AMOUNT);
		});

		const EST_FEES: u128 = 1_600_000_000;
		Trappist::execute_with(|| {
			// Ensure beneficiary account balance increased
			let current_balance = trappist::Assets::balance(txUSD, &ALICE);
			assert_balance(current_balance, beneficiary_balance + AMOUNT, EST_FEES);
			println!(
				"Reserve-transfer: initial balance {} transfer amount {} current balance {} estimated fees {} actual fees {}",
				beneficiary_balance.separate_with_commas(),
				AMOUNT.separate_with_commas(),
				current_balance.separate_with_commas(),
				EST_FEES.separate_with_commas(),
				(beneficiary_balance + AMOUNT - current_balance).separate_with_commas()
			);
		});
	}

	// Initiates a send of a XCM message from trappist to the asset reserve parachain, instructing
	// it to transfer some amount of a fungible asset to some tertiary (base) parachain (HRMP)
	#[test]
	fn two_hop_reserve_transfer_from_trappist_parachain_to_tertiary_parachain() {
		init_tracing();

		MockNet::reset();

		const ASSET_MIN_BALANCE: asset_reserve::Balance = 1_000_000_000;
		const AMOUNT: u128 = 100_000_000_000;

		AssetReserve::execute_with(|| {
			// Create and mint fungible asset on Reserve Parachain
			assert_ok!(create_asset_on_asset_reserve(xUSD, ALICE, ASSET_MIN_BALANCE));
			assert_ok!(mint_asset_on_asset_reserve(xUSD, ALICE, AMOUNT * 2));

			// Touch parachain account
			assert_ok!(asset_reserve::Assets::transfer(
				asset_reserve::RuntimeOrigin::signed(ALICE),
				xUSD,
				asset_reserve::sovereign_account(TRAPPIST_PARA_ID).into(),
				AMOUNT
			));
		});

		Relay::execute_with(|| {
			// Declare xUSD (on Reserve Parachain) as self-sufficient via Relay Chain
			paras_sudo_wrapper_sudo_queue_downward_xcm(asset_reserve::RuntimeCall::Assets(
				pallet_assets::Call::<asset_reserve::Runtime>::force_asset_status {
					id: xUSD,
					owner: ALICE.into(),
					issuer: ALICE.into(),
					admin: ALICE.into(),
					freezer: ALICE.into(),
					min_balance: ASSET_MIN_BALANCE,
					is_sufficient: true,
					is_frozen: false,
				},
			));
		});

		let mut beneficiary_balance = 0;
		Base::execute_with(|| {
			// Create fungible asset on tertiary parachain
			assert_ok!(create_derivative_asset_on_tertiary_parachain(
				pxUSD,
				ALICE,
				ASSET_MIN_BALANCE
			));
			beneficiary_balance = base::Assets::balance(pxUSD, &ALICE);
		});

		const MAX_WEIGHT: u128 = 1_000_000_000 * 2; // 1,000,000,000 per instruction
		const EXECUTION_COST: u128 = 65_000_000_000;

		Trappist::execute_with(|| {
			// Create derivative asset on Trappist Parachain
			assert_ok!(create_derivative_asset_on_trappist(txUSD, ALICE.into(), ASSET_MIN_BALANCE));

			// Mint derivative asset on Trappist Parachain
			assert_ok!(trappist::Assets::mint(
				trappist::RuntimeOrigin::signed(ALICE),
				txUSD,
				ALICE.into(),
				AMOUNT * 2
			));
			assert_eq!(trappist::Assets::balance(txUSD, &ALICE), AMOUNT * 2);

			// Map derivative asset (txUSD) to multi-location (xUSD within Assets pallet on Reserve
			// Parachain) via Asset Registry
			assert_ok!(register_reserve_asset_on_trappist(ALICE, txUSD));
			assert!(trappist::AssetRegistry::asset_id_multilocation(txUSD).is_some());

			// Trappist parachain should be able to reserve-transfer an asset to Tertiary Parachain
			assert_ok!(trappist::PolkadotXcm::execute(
				trappist::RuntimeOrigin::signed(ALICE),
				Box::new(VersionedXcm::from(Xcm(vec![
					WithdrawAsset(
						(
							(
								Parent,
								X3(
									Parachain(ASSET_RESERVE_PARA_ID),
									PalletInstance(asset_reserve::Assets::index() as u8),
									GeneralIndex(xUSD as u128)
								)
							),
							AMOUNT
						)
							.into()
					),
					InitiateReserveWithdraw {
						assets: Wild(All),
						reserve: (Parent, Parachain(ASSET_RESERVE_PARA_ID)).into(),
						xcm: Xcm(vec![
							BuyExecution {
								fees: (
									X2(
										PalletInstance(asset_reserve::Assets::index() as u8),
										GeneralIndex(xUSD as u128)
									),
									EXECUTION_COST
								)
									.into(),
								weight_limit: Unlimited
							},
							DepositReserveAsset {
								assets: Wild(All),
								max_assets: 1,
								dest: (Parent, Parachain(BASE_PARA_ID)).into(),
								xcm: Xcm(vec![DepositAsset {
									assets: Wild(All),
									max_assets: 1,
									beneficiary: X1(AccountId32 { network: Any, id: ALICE.into() })
										.into()
								}])
							}
						])
					},
				]))),
				Weight::from_ref_time(MAX_WEIGHT as u64)
			));

			// // Check send amount moved to sovereign account
			// let sovereign_account = asset_reserve::sovereign_account(TRAPPIST_PARA_ID);
			// assert_eq!(asset_reserve::Assets::balance(xUSD, &sovereign_account), AMOUNT);
		});

		Base::execute_with(|| {
			// Ensure beneficiary received amount, less fees
			let current_balance = base::Assets::balance(pxUSD, &ALICE);
			assert_balance(current_balance, beneficiary_balance + AMOUNT, EXECUTION_COST);
			println!(
				"Two-hop Reserve-transfer: initial balance {} transfer amount {} current balance {} estimated fees {} actual fees {}",
				beneficiary_balance.separate_with_commas(),
				AMOUNT.separate_with_commas(),
				current_balance.separate_with_commas(),
				EXECUTION_COST.separate_with_commas(),
				(beneficiary_balance + AMOUNT - current_balance).separate_with_commas()
			);
		});
	}

	fn assert_balance(actual: u128, expected: u128, fees: u128) {
		assert!(
			actual >= (expected - fees) && actual <= expected,
			"expected: {expected}, actual: {actual} fees: {fees}"
		)
	}

	fn create_asset_on_asset_reserve(
		id: asset_reserve::AssetId,
		admin: asset_reserve::AccountId,
		min_balance: asset_reserve::Balance,
	) -> DispatchResult {
		asset_reserve::Assets::create(
			asset_reserve::RuntimeOrigin::signed(ALICE),
			id,
			admin.into(),
			min_balance,
		)
	}

	fn create_derivative_asset_on_trappist(
		id: trappist::AssetId,
		admin: trappist::AccountId,
		min_balance: trappist::Balance,
	) -> DispatchResult {
		trappist::Assets::create(
			trappist::RuntimeOrigin::signed(ALICE),
			id,
			admin.into(),
			min_balance,
		)
	}

	fn create_derivative_asset_on_tertiary_parachain(
		id: base::AssetId,
		admin: base::AccountId,
		min_balance: base::Balance,
	) -> DispatchResult {
		base::Assets::create(base::RuntimeOrigin::signed(ALICE), id, admin.into(), min_balance)
	}

	fn mint_asset_on_asset_reserve(
		asset_id: asset_reserve::AssetId,
		origin: asset_reserve::AccountId,
		mint_amount: asset_reserve::Balance,
	) -> DispatchResult {
		asset_reserve::Assets::mint(
			asset_reserve::RuntimeOrigin::signed(origin),
			asset_id,
			ALICE.into(),
			mint_amount,
		)
	}

	fn paras_sudo_wrapper_sudo_queue_downward_xcm<RuntimeCall: Encode>(call: RuntimeCall) {
		let sudo_queue_downward_xcm =
			relay_chain::RuntimeCall::ParasSudoWrapper(mock_paras_sudo_wrapper::Call::<
				relay_chain::Runtime,
			>::sudo_queue_downward_xcm {
				id: ParaId::new(ASSET_RESERVE_PARA_ID),
				xcm: Box::new(VersionedXcm::V2(Xcm(vec![Transact {
					origin_type: OriginKind::Superuser,
					require_weight_at_most: 10000000000u64,
					call: call.encode().into(),
				}]))),
			});

		assert_ok!(relay_chain::Sudo::sudo(
			relay_chain::RuntimeOrigin::signed(ALICE),
			Box::new(sudo_queue_downward_xcm),
		));
	}

	fn register_reserve_asset_on_trappist(
		origin: trappist::AccountId,
		asset_id: trappist::AssetId,
	) -> DispatchResultWithPostInfo {
		trappist::Sudo::sudo(
			trappist::RuntimeOrigin::signed(origin),
			Box::new(trappist::RuntimeCall::AssetRegistry(pallet_asset_registry::Call::<
				trappist::Runtime,
			>::register_reserve_asset {
				asset_id,
				asset_multi_location: (
					Parent,
					X3(
						Parachain(ASSET_RESERVE_PARA_ID),
						PalletInstance(asset_reserve::Assets::index() as u8),
						GeneralIndex(xUSD as u128),
					),
				)
					.into(),
			})),
		)
	}
}
