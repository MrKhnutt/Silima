async function hashFiles() {
    let data;
    const hash = sha256.create();

    try {
        const directory  = await fetch("/client.dir");
        data             = await directory.text();
    } catch (err) {
        console.log("Unable to find /client.dir\n", err);
        return null;
    };

    for (const fileName of data.trim().split("\n").toSorted()) {
        try {
            hash.update(fileName);
            hash.update(await (await fetch(fileName)).arrayBuffer());
        } catch (err) {
            console.log("Unable to find ", fileName, "\n", err);
            return null;
        }
    };
    
    return hash.hex();
};
