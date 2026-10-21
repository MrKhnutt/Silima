# ![Alt text](./media/logo/silimaLogoWhite.svg)

Silima is an open-source, self-hosted platform for collective networked gameplay. It lets groups of users connect through a browser, vote on or submit controller inputs, view shared gameplay, and interact with trivia and educational content while a host system runs and manages the played game.

> **Status:** Silima is currently being actively developed.

## ToDo List

### Currently Working On:

- Limited Video Streaming
- Backend API
- Frontend Devolvement with API access

### Suspended:

- Administration Tools
- Thread sync and Alerts

<details> 

<summary><strong>Full ToDo List</strong></summary>

### Server Side

- 🔨 Atomic Client Identity
- ✅ Per User Voting
- ✅ Shared State Architecture
- 🔨 Vote Resolution
    - ✅ Game Modes
        - ✅ Anarchy
        - ✅ Democracy
    - ⬜ Game Mode Voting
    - 🔨 Spam Protection
    - ✅ Polling Behavior Broadcast
- 🔨 Websocket Lifecycle
- ⬜ Anonymous IP Binding
- 🔨 Vote Resolution
- ⬜ Networking Controller Binding
- ✅ Build Hashing and directory manifest
- 🔨 Build Sync
- 🔨 JS Hash Query Strings for Cache Updates
- 🔨 Build.rs hash comparison for for manifest updates

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

- ⏸️ Administration Page
- ⬜ Lockable Controls
- ⬜ Game Mode Toggle
- ⬜ Theme Selection
- ⬜ Input Reporting
- ⬜ Kick-Screen
- ⬜ Refreshable expiring Join Key
- ⬜ Logging/Statistics
- ⬜ Pause/Resume
- ⬜ Game Mode Change Vote Threshold
- ⬜ Frame-per-Action Toggle
- ⬜ Config Work

### Misc

- ✅ Script to Normalize SVGs
- ⬜ Run script if icons have changed
- ⬜ Manually invoke script from build command/admin menu

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

## Documentation

Documentation is currently in development. For now, see this README and the source comments for setup and implementation details.

## License

No license has been selected yet.
