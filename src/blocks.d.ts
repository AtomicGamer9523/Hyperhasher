export class Block {
    previos_hash : string ;
    data : string ;
    constructor ( data : string , chain: Chain ) ;
}

export class Chain {
    hash : string ;
    constructor ( ) ;

    push ( block : Block ) : void ;

    clean ( ) : void ;
}