export declare class PSGChip {
    constructor();
    init(): void;
    write(data: number): void;
    read(): number;
    setIC(ic: number): void;
    clock(): void;
    getOutput(): number;
    test(testValue: number): void;
    generate(): number;
    writeBuffered(data: number): void;
}

export function newPSGChip(): PSGChip;
