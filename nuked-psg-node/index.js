const addon = require("./build/nuked-psg-node.node")

class PSGChip {
    constructor() {
        this.inner = addon.new()
    }

    init() {
        addon.init(this.inner)
    }

    write(data) {
        addon.write(this.inner, data)
    }

    read() {
        return addon.read(this.inner)
    }

    setIC(ic) {
        addon.setIC(this.inner, ic)
    }

    clock() {
        addon.clock(this.inner)
    }

    getOuput() {
        return addon.getOuput(this.inner)
    }

    test(testValue) {
        addon.test(this.inner, testValue)
    }

    generate() {
        return addon.generate(this.inner)
    }

    writeBuffered(data) {
        addon.writeBuffered(this.inner, data)
    }
}

const newPSGChip = () => new PSGChip()

exports = module.exports = {
    newPSGChip,
    PSGChip
}
