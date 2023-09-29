import beer from "beer-names";
import superb from "superb";
import fs from "fs";

import { NftMetadata } from "./interfaces/metadata-interface";
import { IpfsManager } from "./interfaces/ipfs-manager";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { NftCreator } from "./nft-creator";
import { createDirSync } from "./utils";

import { CID } from "multiformats/cid";
import * as json from 'multiformats/codecs/json'
import { sha256 } from 'multiformats/hashes/sha2'

const config = require("./mock-nft-config.json");

// Color trait to randomly choose for mock nfts
const colors = [
    "Red",
    "Blue",
    "Green",
    "Yellow",
    "Orange",
    "Purple",
    "Pink",
    "Brown",
    "Gray",
    "White",
    "Black",
    "Cyan",
    "Magenta",
    "Turquoise"
];

// Generates mock NFT metadata files and saves them to `outputDir`
function generateMockNfts(amount: number, outputDir: string, cleanDir: boolean = true) {
    // remove trailing slashes
    outputDir = outputDir.replace(/\/$/, "");
    createDirSync(outputDir, cleanDir);

    for (let i = 1; i <= amount; i++) {
        const randomName = beer.random();
        // get the last word, which is the beer type (ale, lager, etc.)
        const beerType: string = randomName.split(" ").slice(-1).join("");
        // add superb word + beer type
        let randomDescription = superb.random() + " " + beerType;
        const randomColor = colors[Math.floor(Math.random() * colors.length)];

        let metadata: NftMetadata = {
            attributes: [
                {
                    trait_type: "type",
                    value: beerType
                },
                {
                    trait_type: "color",
                    value: randomColor
                }
            ],
            description: randomDescription,
            image: "",
            name: randomName,
            itemId: i
        };

        // pad id with 0's for file name
        const fileId = i.toString().padStart(amount.toString().length, "0");
        const fileName = outputDir + "/beer_nft_" + fileId + ".json";
        fs.writeFileSync(fileName, JSON.stringify(metadata, null, 2));
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

    // generate metadata for `numNfts`. Save the metadata files to `metadataDir`
    generateMockNfts(config.numNfts, config.out.metadataDir);

    let nftCreator = new NftCreator(config, dotApi, signer, new MockIpfs());

    console.log("Creating NFT collection");
    await nftCreator.createNftCollection();
    console.log("Creating NFTs...");
    await nftCreator.bulkCreateNfts(config.numNfts);
    console.log("Done!");
}

main();