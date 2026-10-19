async function versionUpdate(version, hash) {
    let hashStr = await hash;
    document.querySelector(".silima-version").textContent = "v" + version + ":" + hashStr.toUpperCase();
}
