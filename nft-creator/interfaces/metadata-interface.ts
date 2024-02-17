
export interface NftAttribute {
    trait_type: string;
    value: string;
}

export interface NftMetadata {
    attributes: NftAttribute[];
    description: string;
    image: string;
    name: string;
    itemId: number;
}