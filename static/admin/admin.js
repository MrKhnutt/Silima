// const listeners = {
//     a: pressA,
//     b: pressB,
//     x: pressX,
//     y: pressY,
//     start: pressStart,
//     select: pressSelect,
//     l: pressL,
//     r: pressR,
//     up: pressUp,
//     left: pressLeft,
//     down: pressDown,
//     right: pressRight,
// };

const wsProtocol = window.location.protocol === "https:" ? "wss" : "ws";
const socket = new WebSocket(`${wsProtocol}://${window.location.host}/ws`);

console.log("Page:", window.location.href);

/* =========================
   DOM REFERENCES
========================= */

const serverStatusLight = document.getElementById("server-status-light");
const serverStatusText = document.getElementById("server-status-text");

const studentCount = document.getElementById("student-count");

const retroarchStatusLight = document.getElementById("retroarch-status-light");
const retroarchStatus = document.getElementById("retroarch-status");

const websocketStatusLight = document.getElementById("websocket-status-light");
const websocketStatus = document.getElementById("websocket-status");

const votingStatusLight = document.getElementById("voting-status-light");
const votingStatus = document.getElementById("voting-status");

const currentPeriod = document.getElementById("current-period");
const currentWinner = document.getElementById("current-winner");

const toggleVoting = document.getElementById("toggle-voting");
const setAnarchy = document.getElementById("set-anarchy");
const setDemocracy = document.getElementById("set-democracy");
const clearVotes = document.getElementById("clear-votes");

const votingPeriod = document.getElementById("voting-period");
const applyPeriod = document.getElementById("apply-period");

const disconnectClients = document.getElementById("disconnect-clients");
const shutdownServer = document.getElementById("shutdown-server");

const passwordDialog = document.getElementById("password-dialog");
const passwordForm   = document.getElementById("password-form");
const passwordInput  = document.getElementById("admin-password");
const passwordError  = document.getElementById("password-error");

/* =========================
   PAGE LOADER
========================= */

window.addEventListener("load", () => {

    passwordDialog.showModal();
});

/* =========================
   WEBSOCKET
========================= */

socket.addEventListener("open", () => {
    setStatus(
        serverStatusLight,
        serverStatusText,
        true,
        "Server Online"
    );

    setStatus(
        websocketStatusLight,
        websocketStatus,
        true,
        "Connected"
    );

    sendMessage({
        type: "admin_connected"
    });
});

socket.addEventListener("close", () => {
    setStatus(
        serverStatusLight,
        serverStatusText,
        false,
        "Server Offline"
    );

    setStatus(
        websocketStatusLight,
        websocketStatus,
        false,
        "Disconnected"
    );
});

socket.addEventListener("error", (error) => {
    console.error("WebSocket error:", error);
    setStatus(
        websocketStatusLight,
        websocketStatus,
        false,
        "Error"
    );
});

socket.addEventListener("message", (event) => {
    const message = JSON.parse(event.data);
    handleServerMessage(message);
});


/* =========================
   SERVER MESSAGE HANDLER
========================= */

function handleServerMessage(message) {

    switch (message.type) {
        case "password-response":
            if (Boolean(message.value)) {
                passwordDialog.close();
                passwordInput.value = "";
                passwordError.textContent = "";
            }
            else {
                passwordError.textContent = "Incorrect password.";
                passwordInput.select();
                console.log("incorrect password")
            }
            break;
        default:
            console.log("invalid message received: ", message.type);
        }
}

/* =========================
   FULL STATE UPDATE
========================= */

// function updateFullState(state) {

//     if (state.clientCount !== undefined) {
//         studentCount.textContent = state.clientCount;
//     }
//     if (state.retroarchConnected !== undefined) {
//         setStatus(
//             retroarchStatusLight,
//             retroarchStatus,
//             state.retroarchConnected,
//             state.retroarchConnected
//                 ? "Connected"
//                 : "Disconnected"
//         );
//     }

//     if (state.votingActive !== undefined) {
//         updateVotingStatus(state.votingActive);
//     }
//     if (state.period !== undefined) {
//         currentPeriod.textContent = state.period;
//         votingPeriod.value = state.period;
//     }
//     if (state.votes !== undefined) {
//         updateVotes(state.votes);
//     }
//     if (state.winner !== undefined) {
//         currentWinner.textContent = state.winner ?? "None";
//     }
// }


/* =========================
   VOTING DISPLAY
========================= */

// function updateVotes(votes) {

//     const voteRows = document.querySelectorAll(".vote-row");
//     let highestVote = 0;


//     for (const row of voteRows) {

//         const input = row.dataset.input;
//         const count = votes[input] ?? 0;

//         if (count > highestVote) {
//             highestVote = count;
//         }

//         row.querySelector(".vote-count").textContent = count;
//     }


//     for (const row of voteRows) {

//         const input = row.dataset.input;
//         const count = votes[input] ?? 0;
//         const bar = row.querySelector(".vote-bar");

//         let percentage = 0;

//         if (highestVote > 0) {
//             percentage = (count / highestVote) * 100;
//         }

//         bar.style.width = `${percentage}%`;
//     }
// }


/* =========================
   STATUS HELPERS
========================= */

function setStatus(light, text, active, label) {

    light.classList.remove(
        "online",
        "warning",
        "offline"
    );

    if (active) {
        light.classList.add("online");
    }
    else {
        light.classList.add("offline");
    }

    text.textContent = label;
}


// function updateVotingStatus(active) {

//     setStatus(
//         votingStatusLight,
//         votingStatus,
//         active,
//         active ? "Active" : "Paused"
//     );

//     toggleVoting.textContent =
//         active
//             ? "Pause Voting"
//             : "Resume Voting";
// }

/* =========================
   SEND MESSAGE
========================= */

function sendMessage(message) {

    if (socket.readyState !== WebSocket.OPEN) {
        console.warn("Cannot send: WebSocket is not open.");
        return;
    }

    socket.send(JSON.stringify(message));
}

// /* =========================
//    ADMIN CONTROLS
// ========================= */

setAnarchy.addEventListener("click", () => {
    sendMessage({
        type: "vote-state",
        value: 0
    });
    document.getElementById("pause-state").setAttribute("hidden", "true");
    document.getElementById("status-value").removeAttribute("hidden");
    document.getElementById("status-value").innerHTML = "Anarchy";
});
setDemocracy.addEventListener("click", () => {
    sendMessage({
        type: "vote-state",
        value: 1
    });
    document.getElementById("pause-state").setAttribute("hidden", "true");
    document.getElementById("status-value").removeAttribute("hidden");
    document.getElementById("status-value").innerHTML = "Democracy";
});
toggleVoting.addEventListener("click", () => {
    sendMessage({
        type: "vote-state",
        value: 2
    });
    if (document.getElementById("pause-state").getAttribute("hidden") == "true") {
        document.getElementById("status-value").setAttribute("hidden", "true");
        document.getElementById("pause-state").removeAttribute("hidden");
    } else {
        document.getElementById("pause-state").setAttribute("hidden", "true");
        document.getElementById("status-value").removeAttribute("hidden");
    };
});

// clearVotes.addEventListener("click", () => {
//     sendMessage({
//         type: "clear_votes"
//     });
// });

applyPeriod.addEventListener("click", () => {

    const period = Number(votingPeriod.value);

    if (!Number.isFinite(period) || period < 100) {
        console.warn("Invalid voting period.");
        return;
    }

    sendMessage({
        type: "set-voting-period",
        value: period
    });
});

// disconnectClients.addEventListener("click", () => {

//     sendMessage({
//         type: "disconnect_clients"
//     });
// });

shutdownServer.addEventListener("click", () => {

    const confirmed =
        window.confirm("Shut down the Silima server?");
    if (!confirmed) {
        return;
    }
    sendMessage({
        type: "shutdown"
    });
});


/* =========================
   MANUAL CONTROLLER
========================= */

// const controllerButtons =
//     document.querySelectorAll(".controller-button");

// controllerButtons.forEach((button) => {
//     button.addEventListener("click", () => {
//         const input = button.dataset.input;
//         sendMessage({
//             type: "manual_input",
//             input: input
//         });
//     });
// });

/* =========================
    PASSWORD
========================= */

passwordForm.addEventListener("submit", async (event) => {
    event.preventDefault();

    const password = await hashPassword(passwordInput.value);

    console.log(passwordInput.value);
    console.log(password);

    sendMessage({
        type: "password-request",
        value: String(password)
    });
    //     if (response.ok) {
    //         passwordDialog.close();
    //         passwordInput.value = "";
    //         passwordError.textContent = "";
    //     }
    //     else {
    //         passwordError.textContent = "Incorrect password.";
    //         passwordInput.select();
    //     }
    // }
    // catch (err) {
    //     passwordError.textContent = "Unable to contact Silima.";
    //     console.error(err);
    // }
});

