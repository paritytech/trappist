import { NftAttribute, NftMetadata } from "./interfaces/metadata-interface";
import mergeImg from "merge-img";
import fs from "fs";
import { DescriptionGenerator, NameGenerator } from "./interfaces/name-and-description";

export class NftGenerator {
    traitsDir: string;
    imageDir: string;
    metadataDir: string;
    imageWidth: number;
    nameGenerator: NameGenerator;
    descriptionGenerator: DescriptionGenerator;

    constructor(traitsDir: string, imageDir: string, metadataDir: string, imageWidth: number, nameGenerator: NameGenerator, descriptionGenerator: DescriptionGenerator) {
        this.traitsDir = traitsDir;
        this.imageDir = imageDir;
        this.metadataDir = metadataDir;
        this.imageWidth = imageWidth;
        this.nameGenerator = nameGenerator;
        this.descriptionGenerator = descriptionGenerator;
    }

    // inspiration from https://github.com/UniqueNetwork/nft-workshop/blob/master/step2-image-generator.js#L9
    // Set x offset for each image to overlay them using merge-img
    getImgsData(images: string[]) {
        let imgs = [];
        for (let i = 0; i < images.length; i++) {
            imgs.push(
                {
                    src: images[i],
                    offsetX: (i == 0) ? 0 : -this.imageWidth,
                    offsetY: 0,
                }
            )
        }

        return imgs;
    }

    generateMetadata(traits: [string, string[]][], currentTraitIndexes: number[], id: number): NftMetadata {
        let attributes: NftAttribute[] = [];
        for (let i in currentTraitIndexes) {
            attributes.push({
                // remove the number prefix (00- ,01-, etc.)
                trait_type: traits[i][0].replace(/^\d+-/, ''),
                // remove file extension
                value: traits[i][1][currentTraitIndexes[i]].replace(/\.\w+$/, '')
            })
        }

        return {
            attributes: attributes,
            description: this.descriptionGenerator.generateDescription(attributes, id),
            image: "",
            name: this.nameGenerator.generateName(attributes, id),
            itemId: id
        } as NftMetadata;
    }


    async generateNfts(numNfts: number) {
        const traitCategories = fs.readdirSync(this.traitsDir);
        // [traitCategory1, [trait1, ...], ...]
        let traits: [string, string[]][] = [];
        let maxNftCombos = 0;
        // to keep track of already generated NFTs
        let seenCombinations: Set<string> = new Set();

        // collect traits for each category
        for (const i in traitCategories) {
            let traitFiles = fs.readdirSync(this.traitsDir + "/" + traitCategories[i]);
            // remove any files that aren't .png
            traitFiles = traitFiles.filter((file) => file.endsWith(".png"));
            const newEntry: [string, string[]] = [traitCategories[i], traitFiles];
            traits[i] = newEntry;
        }

        // max combinations is the product of the number of traits in each category
        maxNftCombos = traits.reduce((acc, cur) => acc * cur[1].length, 1);

        for (let i = 0; i < numNfts && seenCombinations.size < maxNftCombos; i++) {
            console.log("generating NFT " + i);
            let currentTraits: string[] = [];
            // keep track of the index of the trait used for each category
            let currentTraitIndexes: number[] = [];
            for (let traitIndex in traits) {
                const traitName = traits[traitIndex][0];
                const traitFiles = traits[traitIndex][1];
                const randTrait = Math.floor(Math.random() * traitFiles.length);
                const randomTrait = traitFiles[randTrait];

                currentTraits.push(this.traitsDir + "/" + traitName + "/" + randomTrait);
                currentTraitIndexes.push(randTrait);
            }

            let combination = currentTraitIndexes.toString();
            if (seenCombinations.has(combination)) {
                console.log("Duplicate combination, skipping");
                i--;
                continue;
            }

            seenCombinations.add(combination);

            const metadata = this.generateMetadata(traits, currentTraitIndexes, i);

            let images = this.getImgsData(currentTraits);
            const img = await mergeImg(images);

            // pad id with 0's for file name
            const nftId = i.toString().padStart(numNfts.toString().length, "0");
            const nftName = nftId + "_" + metadata.name;
            const metadataFileName = this.metadataDir + "/" + nftName + ".json";
            fs.writeFileSync(metadataFileName, JSON.stringify(metadata, null, 2));
            await img.write(this.imageDir + "/" + nftName + ".png");
        }
    }
}