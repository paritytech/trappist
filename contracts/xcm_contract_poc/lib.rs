#![cfg_attr(not(feature = "std"), no_std)]
use ink::env::chain_extension::FromStatusCode;
use ink::env::Environment;
use scale::Decode;
pub use xcm::{VersionedMultiAsset, VersionedMultiLocation, VersionedResponse, VersionedXcm};
use frame_support::{weights::Weight};

#[derive(Decode)]
pub enum Error {
    NoResponse = 1,
}

impl FromStatusCode for Error {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::NoResponse),
            _ => panic!("Unknown status code"),
        }
    }
}

#[ink::chain_extension]
pub trait XCMExtension {
    type ErrorCode = Error;

    #[ink(extension = 0x00010000, handle_status = false, returns_result = false)]
    fn prepare_execute(xcm: VersionedXcm<()>) -> Weight;

    #[ink(extension = 0x00010001, handle_status = false, returns_result = false)]
    fn execute();

    #[ink(extension = 0x00010002, handle_status = false, returns_result = false)]
    fn prepare_send(dest: VersionedMultiLocation, xcm: VersionedXcm<()>) -> VersionedMultiAsset;

    #[ink(extension = 0x00010003, handle_status = false, returns_result = false)]
    fn send();

    #[ink(extension = 0x00010004, handle_status = false, returns_result = false)]
    fn new_query() -> u64;

    #[ink(extension = 0x00010005, handle_status = true, returns_result = false)]
    fn take_response(query_id: u64) -> Result<VersionedResponse, Error>;
}

pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize = <ink::env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink::env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink::env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink::env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink::env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink::env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = XCMExtension;
}

#[ink::contract(env = crate::CustomEnvironment)]
mod xcm_contract_poc {

    pub use xcm::opaque::latest::prelude::{
        Junction, Junctions, MultiLocation,
        Transact, OriginKind
    };
    //pub use xcm::opaque::latest::prelude::*;
    pub use xcm::{VersionedMultiAsset, VersionedMultiLocation, VersionedResponse, VersionedXcm, v3::{Xcm,Transact,WeightLimit,Fungibility,AssetId,Parent,WildMultiAsset,MultiAsset,MultiAssets,MultiAssetFilter,Instruction::{DepositReserveAsset,InitiateReserveWithdraw,BuyExecution,DepositAsset,WithdrawAsset}}};
    use ink_prelude::vec::Vec;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct XcmContractPoC {
        value: bool
    }

    impl XcmContractPoC {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(value: bool) -> Self {
            Self {value}
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn send_message(&mut self, paraId: u32, call: Vec<u8>, weight: u64) {
            let multi_location = VersionedMultiLocation::V3(MultiLocation {
                parents: 1,
                interior: X1(Junction::Parachain(paraId)),
            });
            let versioned_xcm = VersionedXcm::from(Xcm([Transact {
                origin_type: OriginKind::Native,
                require_weight_at_most: weight as u64,
                call: call.into()
            }]
            .to_vec()));

            self
                .env()
                .extension()
                .prepare_send(multi_location, versioned_xcm);
            self.env().extension().send();
        }
    }
}
