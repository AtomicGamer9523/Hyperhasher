import * as hasher from './macros';
import * as fs from 'fs';

export class Block {
    previos_hash : string ;
    data : string ;
    constructor ( data : string , chain: Chain ) {
        this.data = data;
        this.previos_hash = chain.hash;
    }    
}

export class Chain {
    hash : string ;
    constructor ( ) {
        this.hash = hasher(
            new Block(
                "INITIAL_BLOCK",
                new Chain()
            )
        )
    }

    push ( block : Block ) : void {
        let hash = hasher(block);
        this.hash = hash;
        return hash;
    }

    clean ( ) : void {
        fs.writeFileSync(
            __dirname + "hash",
            this.hash,
            {
                encoding:"utf-8"
            }
        );
    }
}