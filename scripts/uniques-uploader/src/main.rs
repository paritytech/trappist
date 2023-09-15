// subxt codegen --url http://localhost:9944 | rustfmt > ./src/interface.rs
mod interface;
use crate::interface::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use core::default::Default;
use std::{env, fs::write, time::Duration};
use subxt::{ext::sp_runtime::Saturating, utils::MultiAddress::Id, OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

//const NONCE_FILE: &str = "nonce.txt";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut url = String::new();
	let mut from_collection = 1;
	let mut to_collection = 50;
	let mut items = 50;
	let block_limit = 500;

	for arg in env::args().skip(1) {
		let parts: Vec<&str> = arg.split('=').collect();
		match parts[0] {
			"--from-collection" =>
				from_collection = parts[1].parse().expect("Invalid from_collection value"),
			"--to-collection" =>
				to_collection = parts[1].parse().expect("Invalid to_collection value"),
			"--items" => items = parts[1].parse().expect("Invalid items value"),
			_ => url = arg,
		}
	}

	let api = OnlineClient::<PolkadotConfig>::from_url(&url).await?;
	let signer = &dev::alice();
	let mut nonce = api.tx().account_nonce(&signer.public_key().to_account_id()).await?;

	/*     // Read the last stored nonce value from the file.
	if Path::new(NONCE_FILE).exists() {
		let stored_nonce: u64 = read_to_string(NONCE_FILE)?.trim().parse()?;
		println!("Stored nonce: {}", stored_nonce);
		println!("Current nonce: {}", nonce);
		if stored_nonce > nonce {
			nonce.saturating_inc();
		}
	} */

	for collection_id in from_collection..=to_collection {
		// Check if collection already exists
		let start =
			match api
				.storage()
				.at_latest()
				.await?
				.fetch(&interface::api::storage().uniques().class(&collection_id))
				.await?
			{
				None => {
					// issue a collection
					let issue = interface::api::tx()
						.uniques()
						.create(collection_id, Id(signer.public_key().to_account_id()));
					api.tx()
						.create_signed_with_nonce(&issue, signer, nonce, Default::default())?
						.submit()
						.await?;

					nonce.saturating_inc();

					let data: Vec<u8> = "arbitrary data to store".as_bytes().to_vec();
					let bounded_data: BoundedVec<u8> = BoundedVec(data);

					let collection_metadata = interface::api::tx()
						.uniques()
						.set_collection_metadata(collection_id, bounded_data, false);
					api.tx()
						.create_signed_with_nonce(
							&collection_metadata,
							signer,
							nonce,
							Default::default(),
						)?
						.submit()
						.await?;

					nonce.saturating_inc();
					0
				},
				Some(c) => c.items,
			};

		let mut uploaded_items = 0;
		for item_id in start..items {
			if uploaded_items == block_limit {
				println!("block limit reached, waiting for next block...");
				std::thread::sleep(Duration::from_secs(12));
				uploaded_items = 0
			}

			let mint = interface::api::tx().uniques().mint(
				collection_id,
				item_id,
				Id(signer.public_key().to_account_id()),
			);

			api.tx()
				.create_signed_with_nonce(&mint, signer, nonce, Default::default())?
				.submit()
				.await?;

			nonce.saturating_inc();

			let data = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890abcdefg".to_vec();

			let item_metadata = interface::api::tx().uniques().set_metadata(
				collection_id,
				item_id,
				BoundedVec(data),
				false,
			);

			api.tx()
				.create_signed_with_nonce(&item_metadata, signer, nonce, Default::default())?
				.submit()
				.await?;

			nonce.saturating_inc();
			println!("minted collection: {} / item {}", collection_id, item_id);
			uploaded_items.saturating_inc();
		}
	}

	// Store the new nonce value to the file.
	//write(NONCE_FILE, nonce.to_string())?;

	Ok(())
}
