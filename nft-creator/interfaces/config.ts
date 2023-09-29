export interface Config {
    substrateEndpoint: string;
    collectionId: number;
    numNfts: number;
    imageInfo: {
        traitsDir: string;
        width: number;
    }
    out: {
        metadataDir: string;
        imageDir: string;
    };
}