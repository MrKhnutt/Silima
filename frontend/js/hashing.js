async function hashFiles() {
    let data;
    const hash = sha256.create();
    let encoder = new TextEncoder();

    try {
        const directory  = await fetch("/manifest.json")
                            .then(response => response.json())
                            .catch(error => {
                                console.error(error);
                                return [];
                            });
        data             = await directory.files;
    } catch (err) {
        console.log("Unable to find /client.dir\n", err);
        return null;
    };

    for (let fileName of data.toSorted()) {
        try {
            let bytes = new Uint8Array(await (await fetch(fileName)).arrayBuffer());
            fileName = fileName.replace(/^\.?\//, "");
            // console.log(fileName);
            hash.update(encoder.encode(fileName));
            hash.update(bytes);
        } catch (err) {
            console.log("Unable to find ", fileName, "\n", err);
            return null;
        }
    };
    
    return hash.hex().slice(0,4);
};

const CLIENT_HASH_PROMISE = hashFiles();