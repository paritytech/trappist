use frame_support::{pallet_prelude::Weight, traits::GenesisBuild};
use sp_runtime::AccountId32;

pub const ALICE: AccountId32 = AccountId32::new([
    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
]);
pub const BOB: AccountId32 = AccountId32::new([
    142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201,
    18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
]);
pub const CHARLIE: AccountId32 = AccountId32::new([
    144, 181, 171, 32, 92, 105, 116, 201, 234, 132, 27, 230, 136, 134, 70, 51, 220, 156, 168, 163,
    87, 132, 62, 234, 207, 35, 20, 100, 153, 101, 254, 34,
]);
pub const DAVE: AccountId32 = AccountId32::new([
    48, 103, 33, 33, 29, 84, 4, 189, 157, 168, 142, 2, 4, 54, 10, 26, 154, 184, 184, 124, 102, 193,
    188, 47, 205, 211, 127, 60, 34, 34, 204, 32,
]);
const INITIAL_BALANCE: u128 = 1_000_000_000_000;

pub fn new_ext() -> sp_io::TestExternalities {
    use rococo_runtime::{Runtime, System};

    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Runtime>()
        .unwrap();

    // set initial balances
    pallet_balances::GenesisConfig::<Runtime> {
        balances: vec![(ALICE, INITIAL_BALANCE)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    // set parachain configuration
    polkadot_runtime_parachains::configuration::GenesisConfig::<Runtime> {
        config: default_parachains_host_configuration(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

	pallet_sudo::GenesisConfig::<Runtime> { key: Some(ALICE) }
		.assimilate_storage(&mut t)
		.unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

fn default_parachains_host_configuration(
) -> polkadot_runtime_parachains::configuration::HostConfiguration<
    polkadot_primitives::v2::BlockNumber,
> {
    use polkadot_primitives::v2::{MAX_CODE_SIZE, MAX_POV_SIZE};

    polkadot_runtime_parachains::configuration::HostConfiguration {
        minimum_validation_upgrade_delay: 5,
        validation_upgrade_cooldown: 10u32,
        validation_upgrade_delay: 10,
        code_retention_period: 1200,
        max_code_size: MAX_CODE_SIZE,
        max_pov_size: MAX_POV_SIZE,
        max_head_data_size: 32 * 1024,
        group_rotation_frequency: 20,
        chain_availability_period: 4,
        thread_availability_period: 4,
        max_upward_queue_count: 8,
        max_upward_queue_size: 1024 * 1024,
        max_downward_message_size: 1024,
        ump_service_total_weight: Weight::from_parts(4 * 1_000_000_000, 0),
        max_upward_message_size: 50 * 1024,
        max_upward_message_num_per_candidate: 5,
        hrmp_sender_deposit: 0,
        hrmp_recipient_deposit: 0,
        hrmp_channel_max_capacity: 8,
        hrmp_channel_max_total_size: 8 * 1024,
        hrmp_max_parachain_inbound_channels: 4,
        hrmp_max_parathread_inbound_channels: 4,
        hrmp_channel_max_message_size: 1024 * 1024,
        hrmp_max_parachain_outbound_channels: 4,
        hrmp_max_parathread_outbound_channels: 4,
        hrmp_max_message_num_per_candidate: 5,
        dispute_period: 6,
        no_show_slots: 2,
        n_delay_tranches: 25,
        needed_approvals: 2,
        relay_vrf_modulo_samples: 2,
        zeroth_delay_tranche_width: 0,
        ..Default::default()
    }
}
