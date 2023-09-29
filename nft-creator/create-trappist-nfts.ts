import beer from "beer-names";
import superb from "superb";
import fs from "fs";

import { IpfsManager } from "./interfaces/ipfs-manager";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { NftCreator } from "./nft-creator";
import { NftGenerator } from "./nft-generator";
import { NameGenerator, DescriptionGenerator } from "./interfaces/name-and-description";

import { CID } from "multiformats/cid";
import * as json from 'multiformats/codecs/json'
import { sha256 } from 'multiformats/hashes/sha2'

const config = require("./trappist-nft-config.json");


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

class MockIpfs implements IpfsManager {
    // Mock implementation. There is no actual uploading
    async uploadContent(content: string): Promise<CID> {
        const bytes = json.encode(content);

        const hash = await sha256.digest(bytes);
        return CID.create(1, json.code, hash);
    }

    async uploadFile(filePath: string): Promise<CID> {
        // simple implementation, just read the file and get CID
        return this.uploadContent(fs.readFileSync(filePath, 'utf8'));
    }
}

async function main() {
    const wsProvider = new WsProvider(config.substrateEndpoint);
    const dotApi = await ApiPromise.create({ provider: wsProvider });

    const keyring = new Keyring({ type: 'sr25519' });
    const signer = keyring.addFromUri('//Alice');

    const generator = new NftGenerator(config, new TrappistNftNameGenerator(), new TrappistDescriptionGenerator());
    await generator.generateNfts(10);

    let nftCreator = new NftCreator(config, dotApi, signer, new MockIpfs());

    console.log("Creating NFT collection");
    await nftCreator.createNftCollection();
    console.log("Creating NFTs...");
    await nftCreator.bulkCreateNfts(config.numNfts);
    console.log("Done!");
}

main();