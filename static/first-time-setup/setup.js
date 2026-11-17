const roomName     = document.getElementById("room-name");
const address      = document.getElementById("address-preview");
const saveLocation = document.getElementById("save-location");

const platform =
    navigator.userAgentData?.platform ??
    navigator.platform ??
    "";

if (platform.toLowerCase().includes("win")) {
    saveLocation.value = "%APPDATA%\\Silima";
}
else {
    saveLocation.value = "~/.config/Silima";
}

roomName.addEventListener("input", () => {

    let name = roomName.value
        .toLowerCase()
        .replace(/[^a-z0-9-]/g, "");

    address.textContent =
        name.length > 0
            ? `silima-${name}.local`
            : "silima-room-name.local";
});