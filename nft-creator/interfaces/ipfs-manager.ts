import { NftMetadata } from "./metadata-interface";
import { CID } from "multiformats/cid";


export interface IpfsManager {
    uploadFile(filePath: string): Promise<CID>;
    uploadContent(content: string): Promise<CID>;
}