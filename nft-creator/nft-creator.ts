import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { NftAttribute, NftMetadata } from "./metadata-interface";
import { KeyringPair } from "@polkadot/keyring/types";

import * as fs from 'fs';
import { CID } from "multiformats/cid";
import * as json from 'multiformats/codecs/json'
import { sha256 } from 'multiformats/hashes/sha2'

export class NftCreator {
    private dotApi: ApiPromise;
    private keyring: Keyring;
    private signer: KeyringPair;

    constructor(dotApi: ApiPromise, signer: KeyringPair) {
        this.dotApi = dotApi;
        this.signer = signer;
    }

    async createNftCollection(id: number) {
        const nftCall = this.dotApi.tx.uniques.create(id, this.signer.address);

        const txHash = await nftCall.signAndSend(this.signer, { nonce: -1 });
    }

    async setItemAttributes(id: number, itemId: number, attributes: NftAttribute[]) {
        for (let attribute of attributes) {
            const nftCall = this.dotApi.tx.uniques.setAttribute(id, itemId, attribute.trait_type, attribute.value);

            const txHash = await nftCall.signAndSend(this.signer, { nonce: -1 });
        }
    }

    async setItemMetadata(id: number, itemId: number, data: string, isFrozen: boolean = false) {
        const nftCall = this.dotApi.tx.uniques.setMetadata(id, itemId, data, isFrozen);

        const txHash = await nftCall.signAndSend(this.signer, { nonce: -1 });
    }

    async bulkCreateNfts(id: number, dir: string, max: number) {
        let count = 0;

        const files = fs.readdirSync(dir)
        for (let fileName of files) {
            // limit number of NFTs created
            if (count++ >= max) {
                break;
            }

            const content = fs.readFileSync(dir + "/" + fileName, 'utf8');
            const metadata: NftMetadata = JSON.parse(content);

            const cid = await generateContentHash(metadata);

            await this.setItemMetadata(id, metadata.itemId, cid.toString());
            await this.setItemAttributes(id, metadata.itemId, metadata.attributes);
        };
    }
}

async function generateContentHash(metadata: NftMetadata): Promise<CID> {
    const bytes = json.encode(metadata);

    const hash = await sha256.digest(bytes);
    return CID.create(1, json.code, hash);
}
