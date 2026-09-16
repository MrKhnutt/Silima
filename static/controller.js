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

const statusLight = document.getElementById("status-light");
const statusText = document.getElementById("status-text");
const serverAddress = document.getElementById("server-address");

serverAddress.textContent = window.location.host;

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
    console.log("Server:", event.data);
});

function sendSocket(input) {
    console.log(input);

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
