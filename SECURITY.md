# ![Alt text](./media/logo/silimaLogoWhite.svg)

## Application

Silima is an open-source, self-hosted platform for collective networked gameplay. It lets groups of users connect through a browser, vote on or submit controller inputs, view shared gameplay, and interact with trivia and educational content while a host system runs and manages the played game.

Source:
https://github.com/MrKhnutt/Silima

License:
TBD

## Network Activity

Silima operates primarily over the local network.

- HTTP/WebSocket: student and administrator interfaces
    - TCP 42699: Client Service
    - TCP 42614: Administrative Service
    - UDP 55400: RetroArch Network RetroPad
- No cloud service required
- No telemetry
- No student information is transmitted outside the local network
- No student information is requested or required for participation

## Permissions

Silima does not require administrator privileges.

Silima does not:
- install system services
- modify startup settings
- inject code into other applications
- install browser extensions

## External Software

RetroArch is currently used separately for game execution. An internet browser, such as Firefox or Google Chrome, is required to access the application.

## Release Verification

Version:
TODO

SHA-256:
TODO

Source commit:
TODO