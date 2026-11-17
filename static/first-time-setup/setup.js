const wsProtocol = window.location.protocol === "https:" ? "wss:" : "ws:";
const wsAddress = `${wsProtocol}//${window.location.host}/ws`;
const ws = new WebSocket(wsAddress);

const form         = document.getElementById("setup-form");

const roomName     = document.getElementById("room-name");
const address      = document.getElementById("address-preview");
const saveLocation = document.getElementById("save-location");
const password     = document.getElementById("password");

const platform =
    navigator.userAgentData?.platform ??
    navigator.platform ??
    "";

// OS based logic
if (platform.toLowerCase().includes("win")) {
    saveLocation.value = "%APPDATA%\\Silima";
}
else {
    saveLocation.value = "~/.config/Silima";
}

// console logging
ws.addEventListener("open", () => {
    console.log(`Connected to ${wsAddress}`);
});

ws.addEventListener("close", () => {
    console.log("WebSocket disconnected");
});

ws.addEventListener("error", (event) => {
    console.error("WebSocket error:", event);
});

roomName.addEventListener("input", () => {
    let name = roomName.value
        .toLowerCase()
        .replace(/[^a-z0-9-]/g, "");

    address.textContent =
        name.length > 0
            ? `silima-${name}.local`
            : "silima-feanor.local";
});

// form transmission
form.addEventListener("submit", async (event) => {
    event.preventDefault();
    const data = Object.fromEntries(new FormData(form));

    passHash = await hashPassword(data["password"]);

    ws.send(JSON.stringify({
        "id" : 0,
        "type" : "config-set",
        "room-name" : data["room-name"],
        "save-location" : data["save-location"],
        "password" : passHash
    }));
});

ws.addEventListener("message", (event) => {
    let data = JSON.parse(event.data);
    switch(data.type) {
        case "ok":
            window.location.href = "/done.html";
        case "err":
            alert(data.msg);
        default:
            console.log("JSON not recognized:", data);
    }
});