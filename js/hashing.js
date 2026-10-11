async function hashFiles() {
    let data;
    const hash = sha256.create();
    let encoder = new TextEncoder();

    try {
        const directory  = await fetch("/client.dir");
        data             = await directory.text();
    } catch (err) {
        console.log("Unable to find /client.dir\n", err);
        return null;
    };

    for (let fileName of data.trim().split(/\r?\n/).toSorted()) {
        try {
            let bytes = new Uint8Array(await (await fetch(fileName)).arrayBuffer());
            fileName = fileName.replace(/^\.?\//, "");
            console.log(fileName);
            hash.update(encoder.encode(fileName));
            hash.update(bytes);
        } catch (err) {
            console.log("Unable to find ", fileName, "\n", err);
            return null;
        }
    };
    
    return hash.hex().slice(0,4);
};
