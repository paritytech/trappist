import beer from "beer-names";
import superb from "superb";
import fs from "fs";

import { NftMetadata } from "./interfaces/metadata-interface";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { NftCreator } from "./nft-creator";
import { NftGenerator } from "./nft-generator";
import { NameGenerator, DescriptionGenerator } from "./interfaces/name-and-description";

const config = require("./mock-nft-config.json");


class TrappistNftNameGenerator implements NameGenerator {
    generateName(attributes: any, id: number): string {
        let label = attributes.find((attribute: any) => attribute.trait_type === "label").value;
        // Replace last word (beer type) with label (polkastout, kusamale, squinkist)
        return beer.random().replace(/\b\w+\b(?![^\s]*\s)/, label);
    }
}

class TrappistDescriptionGenerator implements DescriptionGenerator {
    generateDescription(attributes: any, id: number): string {
        let label = attributes.find((attribute: any) => attribute.trait_type === "label").value;
        // add superb word + beer type
        return superb.random() + " " + label;
    }
}

async function main() {
    const wsProvider = new WsProvider(config.substrateEndpoint);
    const dotApi = await ApiPromise.create({ provider: wsProvider });

    const keyring = new Keyring({ type: 'sr25519' });
    const signer = keyring.addFromUri('//Alice');

    const generator = new NftGenerator("trappist-nfts/traits", "trappist-nfts/images", "trappist-nfts/metadata", 1028, new TrappistNftNameGenerator(), new TrappistDescriptionGenerator());
    generator.generateNfts(10);

    let nftCreator = new NftCreator(dotApi, signer);

    console.log("Creating NFT collection");
    await nftCreator.createNftCollection(config.collectionId);
    console.log("Creating NFTs...");
    await nftCreator.bulkCreateNfts(config.collectionId, config.metadataDir, config.numNfts);
    console.log("Done!");
}

main();