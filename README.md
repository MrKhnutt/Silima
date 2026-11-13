# ![Alt text](./media/logo/silimaLogoWhite.svg)

Silima is an open-source, self-hosted platform for collective networked gameplay. It lets groups of users connect through a browser, vote on or submit controller inputs, view shared gameplay, and interact with trivia and educational content while a host system runs and manages the played game.

> **Status:** Silima is currently being actively developed.

## ToDo List

### Currently Working On:

- Administration Tools
- Backend API
- Frontend Devolvement with API access

### Suspended:

- Thread sync and Alerts
- Server hashing

<details> 

<summary><strong>Full ToDo List</strong></summary>

### Server Side

- ✅ Atomic Client Identity
- ✅ Per User Voting
- ✅ Shared State Architecture
- ⏸️ Vote Resolution
    - ✅ Game Modes
        - ✅ Anarchy
        - ✅ Democracy
    - ⬜ Game Mode Voting
    - ⏸️ Spam Protection
    - ✅ Polling Behavior Broadcast
- ⏸️ Websocket Lifecycle
- 🔨 Vote Resolution
- ⬜ Networking Controller Binding
- ✅ Build Hashing and directory manifest
- ⏸️ Build Sync
- ⏸️ JS Hash Query Strings for Cache Updates
- ⏸️ Build.rs hash comparison for for manifest updates

### Client UI

- Controller Elements
    - ✅ Connection Status
    - ⏸️ Room/Game
    - ⬜ Selected Vote
    - ⬜ Polling Stats
    - ⬜ Gamemode
- ⬜ Controller Themes
    - ⬜ GBC Controller
    - ⬜ N64 Controller
    - ⬜ GBA 
- ⬜ Clean IP Retrieval
- ⬜ Map buttons to KeyPress

### UX

- ⬜ Randomly Generated Nicknames/Avatars
- ⬜ Limited Chat
- ⬜ Reactions & Explosions
- ⬜ Spectator Mode
- ⬜ Settings and Keybindings in cache?
- ⬜ Frame-per-Action Streaming

### Administration

- 🔨 Administration Page
- ⬜ Lockable Controls
- ✅ Game Mode Toggle
- ⬜ Theme Selection
- ⬜ Input Reporting
- ⬜ Kick-Screen
- ⬜ Refreshable expiring Join Key
- ✅ Pause/Resume
- ⬜ Game Mode Change Vote Threshold
- ⬜ Frame-per-Action Toggle
- 🔨 Config Work
- 🔨 Behavior Logging
    - ⬜ Admin Log Pane
    - ⬜ .log generation to directory

### Emulation Integration Layer

- ⬜ Retroarch Process Detection
- ⬜ Window and Frame capture & Transmission
- Client Interaction & Trivia Overlays
    - ⬜ Server Reaction (Explosion)
    - ⬜ Server Transmission/Client Caputre
    - ⬜ Democracy Trivia
    - ⬜ Anarchal Trivia
- ⬜ Emulation Setting Generation

### Misc

- ✅ Script to Normalize SVGs
- ⬜ Run script if icons have changed
- ⬜ Intergrate script into Rust
- ⬜ Manually invoke script from build command/admin menu
- ⬜ SHA256 Checksum generation on releases
- ⬜ Metadata generation for Windows
- ⬜ OSS Code signing through Signpath

</details> 

## Features

- Browser-based game controller
- Real-time communication using WebSockets
- Vote-based input aggregation
- RetroArch network controller integration
- Teacher keyboard control remains available

## Technologies Used

- Rust
- Axum
- Tokio
- HTML / CSS / JavaScript
- WebSockets
- UDP / RetroArch

## Security

Silima is designed to operate entirely within a local network without requiring internet access, administrator privileges, or the collection of student information. Communication currently uses unencrypted HTTP and WebSocket connections, and administrative authentication is still under development. Network administrators should restrict access to the administrative interface accordingly.

## Documentation

Documentation is currently in development. For now, see this README and the source comments for setup and implementation details.

For information about network activity, software dependencies, permissions, and known security limitations, see the Silima Information Security Reference. For information regarding game broadcasting, publisher policies, and copyright considerations, see the Game Content Usage and Copyright Reference.

## License

Silima is free and open-source software licensed under the GNU General Public License, Version 3 (GPLv3).

You are free to use, study, modify, and redistribute Silima. Any modified versions distributed to others must also comply with GPLv3, including its source-code availability requirements.

Silima is provided without warranty or guaranteed technical support.

See LICENSE for the complete license terms.

Note: This license applies to Silima itself. Third-party software, copyrighted games, and associated assets remain subject to their respective licenses.
