# NFT Creator

These files provide the building blocks to:
- Bulk create mock NFTs on a local network, no images required
- Generate NFT images from trait .pngs
- Upload files to IPFS (implementation in progress)
- And an e2e example of generating NFTs + metadata from traits, storing them in IPFS (in progres), and creating them on a parachain

## Running
Launch trappist parachain locally.
Instructions found [here](../README.md)

```
cd nft-creator
```

```
yarn
```

```
ts-node create-mock-nfts.ts
```


## Scripts
- `mock-nft-config.json` example config passed for mock NFTs. No images.
- `trappist-nft-config.json` config used for trappist NFTs with traits
- `create-mock-nfts.ts` will generate mock metadata, create collection, and populate each NFT on-chain with the mock metadata CID and attributes.
- `create-trappist-nfts.ts` will generate NFT images & metadata from the `trappist-nfts/traits` folder. Populates the NFT metadata on-chain

The creation scripts include examples on implementing custom interfaces to provide to the `NftGenerator` and `NftCreator` classes.

## Building Blocks
- `nft-generator.ts` is a reusable NFT generator class that will create NFT images from trait images and provide metadata
- `nft-creator.ts` is reusable class that will bulk populate NFT metadata and images on-chain. It is configurable to upload metadata & images on-chain based on the `IpfsManager` implementation
- `interfaces/config.ts` the interface that config files must follow
- `interfaces/ipfs-manager.ts` a simple interface for IPFS to allow configurability for IPFS connections & apis
- `interfaces/metadata-interface.ts` the metadata interface that the NFTs must use. Based on common metadata standards. Example [here](https://docs.opensea.io/docs/metadata-standards)
- `interfaces/name-and-description.ts` interfaces to implement custom NFT naming and description generation. Will be provided to the `NftGenerator` class.

