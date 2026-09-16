const listeners = {
    a       : pressA,
    b       : pressB,
    x       : pressX,
    y       : pressY,

    start   : pressStart,
    select  : pressSelect,

    l       : pressL,
    r       : pressR,

    up      : pressUp,
    left    : pressLeft,
    down    : pressDown,
    right   : pressRight,
}
const ws = new WebSocket("ws://127.0.0.1:42699/ws");
let uid = 0;

function sendSocket(input) {
    console.log(input);
    ws.send(input);
}

function pressA() {
    sendSocket("A");
}
function pressB() {
    sendSocket("B");
}
function pressX() {
    sendSocket("X");
}
function pressY() {
    sendSocket("Y");
}
function pressStart() {
    sendSocket("START");
}
function pressSelect() {
    sendSocket("SELECT");
}
function pressL() {
    sendSocket("L");
}
function pressR() {
    sendSocket("R");
}
function pressUp() {
    sendSocket("UP");
}
function pressLeft() {
    sendSocket("LEFT");
}
function pressDown() {
    sendSocket("DOWN");
}
function pressRight() {
    sendSocket("RIGHT");
}

function setup() {
    let buttons = document.querySelectorAll("[data-input]");
    console.log(buttons);

    for (let i = 0; i < buttons.length; i++) {
        buttons[i].onclick = listeners[buttons[i].dataset.input];
        console.log(i);
    }
}

setup();
