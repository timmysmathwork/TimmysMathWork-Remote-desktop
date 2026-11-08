# Relay — Open-Source Remote Desktop (staged project)

Relay is an early-stage cross-platform remote desktop project. The current checked-in deliverable is an **offline, standalone dashboard**—not a remote-control product. It can be opened directly from disk, including with `file://`, without a server or external assets.

## Current status (be precise)

Implemented now:
- Responsive dark/light dashboard, with local theme preference.
- Local device-label/ID records, favorites, and removal using browser local storage.
- Networking, WISP, security, and capability-status screens that clearly distinguish saved preferences from working transports.
- No CDN, remote assets, server, or required internet access for the interface.

Not implemented and **not advertised as working**:
- Native Tauri/Rust application, screen capture, remote viewing, mouse/keyboard injection, clipboard, monitors, or fullscreen viewer.
- LAN discovery, WebRTC, STUN/TURN, relays, WISP transport, connection testing, or reconnection.
- Authentication/pairing backend, encryption protocol, device authorization, update system, installers, or platform permission handling.
- Windows/macOS builds or the requested interoperability tests.

The Connect and WISP test actions report that the capability is unavailable and do not make a network request. No sessions are fabricated. Do not use this dashboard to enter secrets.

## Run the dashboard

Open `frontend/index.html` directly in a modern browser. All styles and scripts are local. Data remains in that browser's local storage; clearing browser site data removes it. No build step is needed for this dashboard.

## Development stages

The project must be developed and tested in explicit stages before claiming the requested product is complete:

1. Establish threat model, protocol specifications, platform support matrix, and native application shell.
2. Implement opt-in local capture and input control with visible consent, OS permission checks, and tests on Windows and macOS.
3. Add authenticated pairing and cryptographic identity/key lifecycle before any remote-control transport.
4. Implement a documented transport, then direct LAN and WebRTC with explicitly configured STUN/TURN; test failure and reconnection behavior.
5. Add WISP only after selecting and documenting the exact compatible implementation and protocol. WISP remains optional.
6. Add signed platform packaging, permission/signing/notarization documentation, accessibility checks, and end-to-end testing.

No network service should accept unauthenticated control. Do not expose an unauthenticated desktop-control port to the public internet. Any future remote session must require explicit authorization on the controlled computer.

## Repository map

- `frontend/` — standalone dashboard, CSS, and JavaScript.
- `docs/architecture.md` — proposed boundaries and security requirements, not a claim of implemented functionality.
- `BUILD.md` — current build/run limitations.
- `SECURITY.md` — reporting and security scope.
- `CONTRIBUTING.md` — contribution expectations.

## License

MIT; see `LICENSE`.
