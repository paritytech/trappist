export interface Config {
    substrateEndpoint: string;
    collectionId: string;
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