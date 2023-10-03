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

    // return tx for creating uniques collection. No call is made
    formCreateNftCollectionTx(): any {
        return this.dotApi.tx.uniques.create(this.config.collectionId, this.signer.address);
    }

    // return tx for setting item attribute. No call is made
    formSetItemAttributeTx(itemId: number, attribute: NftAttribute): any {
        return this.dotApi.tx.uniques.setAttribute(this.config.collectionId, itemId, attribute.trait_type, attribute.value);
    }

    formBatchedSetItemAttributesTxs(itemId: number, attributes: NftAttribute[]): any {
        let txs = [];
        for (let attribute of attributes) {
            const nftCall = this.dotApi.tx.uniques.setAttribute(this.config.collectionId, itemId, attribute.trait_type, attribute.value);
            txs.push(nftCall);
        }

        return txs;
    }

    formSetItemMetadataTx(itemId: number, data: string, isFrozen: boolean = false): any {
        return this.dotApi.tx.uniques.setMetadata(this.config.collectionId, itemId, data, isFrozen);
    }

    async sendBatchedTxs(txs: any[]): Promise<any> {
        const batchTx = this.dotApi.tx.utility.batchAll(txs);

        return await batchTx.signAndSend(this.signer, { nonce: -1 });
    }

    async createNftCollection(): Promise<any> {
        const nftCall = this.formCreateNftCollectionTx();

        // return txHash
        return await nftCall.signAndSend(this.signer, { nonce: -1 });
    }

    async setItemAttributes(itemId: number, attributes: NftAttribute[]): Promise<any> {
        let txs = this.formBatchedSetItemAttributesTxs(itemId, attributes);
        // batch set attribute calls
        return await this.sendBatchedTxs(txs);
    }

    async setItemMetadata(itemId: number, data: string, isFrozen: boolean = false): Promise<any> {
        const nftCall = this.formSetItemMetadataTx(itemId, data, isFrozen);

        return await nftCall.signAndSend(this.signer, { nonce: -1 });
    }

    async bulkCreateNfts(max: number) {
        const dir = this.config.out.metadataDir;

        let metadataTxs = [];
        let attributeTxs = [];

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

            metadataTxs.push(this.formSetItemMetadataTx(metadata.itemId, metadataCid.toString()));
            attributeTxs = attributeTxs.concat(this.formBatchedSetItemAttributesTxs(metadata.itemId, metadata.attributes));
        };

        await this.sendBatchedTxs(attributeTxs);
        await this.sendBatchedTxs(metadataTxs);
    }
}