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
const clearVotes = document.getElementById("clear-votes");

const votingPeriod = document.getElementById("voting-period");
const applyPeriod = document.getElementById("apply-period");

const disconnectClients = document.getElementById("disconnect-clients");
const shutdownServer = document.getElementById("shutdown-server");


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
    try {
        const message = JSON.parse(event.data);
        handleServerMessage(message);
    }
    catch (error) {
        console.error("Invalid server message:", event.data);
    }
});


/* =========================
   SERVER MESSAGE HANDLER
========================= */

// function handleServerMessage(message) {

//     switch (message.type) {

//         case "client_count":
//             studentCount.textContent = message.count;
//             break;


//         case "retroarch_status":
//             setStatus(
//                 retroarchStatusLight,
//                 retroarchStatus,
//                 message.connected,
//                 message.connected ? "Connected" : "Disconnected"
//             );
//             break;


//         case "voting_status":
//             updateVotingStatus(message.active);
//             break;


//         case "voting_period":
//             currentPeriod.textContent = message.period;

//             // Only update the input if the teacher isn't currently editing it.
//             if (document.activeElement !== votingPeriod) {
//                 votingPeriod.value = message.period;
//             }

//             break;


//         case "vote_update":
//             updateVotes(message.votes);
//             break;


//         case "winner":
//             currentWinner.textContent = message.input ?? "None";
//             break;


//         case "state":
//             updateFullState(message);
//             break;


//         default:
//             console.warn("Unknown server message:", message);
//     }
// }

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

// toggleVoting.addEventListener("click", () => {
//     sendMessage({
//         type: "toggle_voting"
//     });
// });

// clearVotes.addEventListener("click", () => {
//     sendMessage({
//         type: "clear_votes"
//     });
// });

// applyPeriod.addEventListener("click", () => {

//     const period = Number(votingPeriod.value);

//     if (!Number.isFinite(period) || period < 100) {
//         console.warn("Invalid voting period.");
//         return;
//     }

//     sendMessage({
//         type: "set_voting_period",
//         period: period
//     });
// });

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
