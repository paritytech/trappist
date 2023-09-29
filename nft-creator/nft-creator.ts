import { ApiPromise} from "@polkadot/api";
import { NftAttribute, NftMetadata } from "./interfaces/metadata-interface";
import { IpfsManager } from "./interfaces/ipfs-manager";
import { Config } from "./interfaces/config";
import { KeyringPair } from "@polkadot/keyring/types";

import * as fs from 'fs';

export class NftCreator {
    private config : Config;
    private dotApi: ApiPromise;
    private signer: KeyringPair;
    private ipfsManager: IpfsManager;

    constructor(config: Config, dotApi: ApiPromise, signer: KeyringPair, ipfsManager: IpfsManager) {
        this.config = config;
        this.dotApi = dotApi;
        this.signer = signer;
        this.ipfsManager = ipfsManager;
    }

    async createNftCollection(): Promise<any> {
        const nftCall = this.dotApi.tx.uniques.create(this.config.collectionId, this.signer.address);

        // return txHash
        return await nftCall.signAndSend(this.signer, { nonce: -1 });
    }

    async setItemAttributes(itemId: number, attributes: NftAttribute[]) {
        for (let attribute of attributes) {
            const nftCall = this.dotApi.tx.uniques.setAttribute(this.config.collectionId, itemId, attribute.trait_type, attribute.value);

            await nftCall.signAndSend(this.signer, { nonce: -1 });
        }
    }

    async setItemMetadata(itemId: number, data: string, isFrozen: boolean = false): Promise<any> {
        const nftCall = this.dotApi.tx.uniques.setMetadata(this.config.collectionId, itemId, data, isFrozen);

        return await nftCall.signAndSend(this.signer, { nonce: -1 });
    }

    async bulkCreateNfts(max: number) {
        const dir = this.config.out.metadataDir;

        const metadataFiles = fs.readdirSync(dir);
        let count = 0;
        for (let fileName of metadataFiles) {
            // limit number of NFTs created
            if (count++ >= max) {
                break;
            }

            const content = fs.readFileSync(dir + "/" + fileName, 'utf8');
            let imageCid = null;
            if (this.config.imageInfo !== null) {
                const imagePath = this.config.out.imageDir + "/" + fileName.replace(".json", ".png");

                imageCid = await this.ipfsManager.uploadFile(imagePath);
            }

            const metadata: NftMetadata = JSON.parse(content);
            metadata.image = imageCid === null ? metadata.image : imageCid.toString();

            const metadataCid = await this.ipfsManager.uploadContent(JSON.stringify(metadata));

            await this.setItemMetadata(metadata.itemId, metadataCid.toString());
            await this.setItemAttributes(metadata.itemId, metadata.attributes);
        };
    }
}