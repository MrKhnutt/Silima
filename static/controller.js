const listeners = {
    a: pressA,
    b: pressB,
    x: pressX,
    y: pressY,
    start: pressStart,
    select: pressSelect,
    l: pressL,
    r: pressR,
    up: pressUp,
    left: pressLeft,
    down: pressDown,
    right: pressRight,
};

const wsProtocol = window.location.protocol === "https:" ? "wss:" : "ws:";
const wsAddress = `${wsProtocol}//${window.location.host}/ws`;
const ws = new WebSocket(wsAddress);

let userID = 0;

const statusLight = document.getElementById("status-light");
const statusText = document.getElementById("status-text");
const serverAddress = document.getElementById("server-address");

serverAddress.textContent = window.location.host;
console.log("Silima Version 0.1b")

ws.addEventListener("open", () => {
    statusText.textContent = "Connected";
    statusLight.classList.remove("disconnected");
    statusLight.classList.add("connected");
    console.log(`Connected to ${wsAddress}`);
});

ws.addEventListener("close", () => {
    statusText.textContent = "Disconnected";
    statusLight.classList.remove("connected");
    statusLight.classList.add("disconnected");
    console.log("WebSocket disconnected");
});

ws.addEventListener("error", (event) => {
    statusText.textContent = "Connection error";
    statusLight.classList.remove("connected");
    statusLight.classList.add("disconnected");
    console.error("WebSocket error:", event);
});

ws.addEventListener("message", (event) => {
    let data = JSON.parse(event.data);
    if (data.msg != "") { console.log(data.msg); }
    switch(data.type) {
        case "pollUpdate":
            let votes = data.value;
            let line = "";
            for (var i = 0; i < votes.length; i++) {
                line += `${getButtonFromID(votes[i].id)}:${votes[i].count} `
            }
            console.log(line);
            break;
        case "userID":
            if (userID == 0) {
                userID = Number(data.value);
                console.log("User ID", userID, "Recieved from Server");
                break;
            } else {
                // We should panic here, something fishy is going on
            }

        default:
            console.log("JSON not recognized:", data);
    }
});

function getButtonFromID(id) {
    switch (id) {
        case 8: return "A";
        case 0: return "B";
        case 9: return "X";
        case 1: return "Y";

        case 3: return "START";
        case 2: return "SELECT";

        case 4: return "UP";
        case 5: return "DOWN";
        case 6: return "LEFT";
        case 7: return "RIGHT";

        case 10: return "L";
        case 11: return "R";

        case -1: throw Error("Invalid Button Recieved from pollUpdate J01");
    }
}

function sendSocket(input) {
    // console.log(input);

    if (userID == 0) { return; };
    if (ws.readyState === WebSocket.OPEN) {
        ws.send(input);
    } else {
        console.warn(`Cannot send ${input}: WebSocket is not connected`);
    }
}

function pressA() { sendSocket("A"); }
function pressB() { sendSocket("B"); }
function pressX() { sendSocket("X"); }
function pressY() { sendSocket("Y"); }
function pressStart() { sendSocket("START"); }
function pressSelect() { sendSocket("SELECT"); }
function pressL() { sendSocket("L"); }
function pressR() { sendSocket("R"); }
function pressUp() { sendSocket("UP"); }
function pressLeft() { sendSocket("LEFT"); }
function pressDown() { sendSocket("DOWN"); }
function pressRight() { sendSocket("RIGHT"); }

function setup() {
    const buttons = document.querySelectorAll("[data-input]");

    for (let i = 0; i < buttons.length; i++) {
        buttons[i].onclick = listeners[buttons[i].dataset.input];
    }
}

setup();
