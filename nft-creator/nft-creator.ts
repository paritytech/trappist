import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { NftAttribute, NftMetadata } from "./interfaces/metadata-interface";
import { IpfsManager } from "./interfaces/ipfs-manager";
import { KeyringPair } from "@polkadot/keyring/types";

import * as fs from 'fs';
import { CID } from "multiformats/cid";
import * as json from 'multiformats/codecs/json'
import { sha256 } from 'multiformats/hashes/sha2'

export class NftCreator {
    private dotApi: ApiPromise;
    private signer: KeyringPair;
    private ipfsManager: IpfsManager;

    constructor(dotApi: ApiPromise, signer: KeyringPair, ipfsManager: IpfsManager) {
        this.dotApi = dotApi;
        this.signer = signer;
        this.ipfsManager = ipfsManager;
    }

    async createNftCollection(id: number): Promise<any> {
        const nftCall = this.dotApi.tx.uniques.create(id, this.signer.address);

        // return txHash
        return await nftCall.signAndSend(this.signer, { nonce: -1 });
    }

    async setItemAttributes(id: number, itemId: number, attributes: NftAttribute[]): Promise<any> {
        for (let attribute of attributes) {
            const nftCall = this.dotApi.tx.uniques.setAttribute(id, itemId, attribute.trait_type, attribute.value);

            await nftCall.signAndSend(this.signer, { nonce: -1 });
        }
    }

    async setItemMetadata(id: number, itemId: number, data: string, isFrozen: boolean = false): Promise<any> {
        const nftCall = this.dotApi.tx.uniques.setMetadata(id, itemId, data, isFrozen);

        return await nftCall.signAndSend(this.signer, { nonce: -1 });
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

            const metadataCid = await this.ipfsManager.uploadContent(content);
            // TODO: handle images

            await this.setItemMetadata(id, metadata.itemId, metadataCid.toString());
            await this.setItemAttributes(id, metadata.itemId, metadata.attributes);
        };
    }
}