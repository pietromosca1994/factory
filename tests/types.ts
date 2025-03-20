export enum SolanaNetwork {
    MainnetBeta = "https://api.mainnet-beta.solana.com",
    Devnet = "https://api.devnet.solana.com",
    Localnet = "http://127.0.0.1:8899",
}

export interface TokenConfigs {
    solanaNetwork: string,
    tokenStandard: string,
    tokenMeta: NonFungibleTokenMeta // | ecc.
}

export class Attribute {
    trait_type: string;
    value: string;

    constructor(trait_type: string, value: string) {
        this.trait_type = trait_type;
        this.value = value;
    }
}

export class Property {
    key: string;
    value: string;

    constructor(key: string, value: string) {
        this.key = key;
        this.value = value;
    }
}

export class NonFungibleTokenMeta {
    name: string;
    symbol: string;
    uri: string;
    description: string;
    attributes: Attribute[];
    properties: Property[];

    constructor(props: {
        name: string;
        symbol: string;
        uri: string;
        description: string;
        attributes: Attribute[];
        properties: Property[];
    }) {
        this.name = props.name;
        this.symbol = props.symbol;
        this.uri = props.uri;
        this.description = props.description;
        this.attributes = props.attributes;
        this.properties = props.properties;
    }
}