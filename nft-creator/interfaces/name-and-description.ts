import { NftAttribute } from "./metadata-interface"

export interface NameGenerator {
    generateName(attributes: NftAttribute[], id: number): string;
}

export interface DescriptionGenerator {
    generateDescription(attributes: NftAttribute[], id: number): string;
}