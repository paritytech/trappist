import { NftMetadata } from "./metadata-interface";


interface IpfsManager {
    uploadMetadata(metadata: NftMetadata);
    uploadImage(file: any);
}