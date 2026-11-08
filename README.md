# Relay — staged open-source remote desktop project

Relay is being built in verified stages. The repository currently contains a standalone dashboard and a native desktop milestone for **user-requested local still-image screen capture**.

## What works in this milestone

- `frontend/index.html`: standalone offline dashboard (file://) for local device labels and preferences.
- `src-tauri/` + `desktop-ui/`: Tauri 2 / Rust desktop shell. The native backend enumerates displays and captures one still image only when the user explicitly presses Capture. The image is rendered in the same local app window.
- OSI-approved MIT license and security/development documentation.

The native capture is not transmitted. The native UI reports browser mode when opened outside Tauri; it does not pretend browser capture works.

## Not implemented

This is **not yet a remote desktop product**. There is no remote screen streaming, mouse/keyboard control, authenticated pairing, cryptographic protocol, clipboard synchronization, LAN discovery, WebRTC/STUN/TURN, WISP, reconnection, connection history, or remote viewer. No Windows/macOS artifacts have been built or tested here. Do not expose this app as a remote service or advertise those features as available.

WISP remains out of scope until an exact, versioned protocol and server implementation are selected. A generic WISP setting is not sufficient for compatibility.

## Run

- Dashboard: open `frontend/index.html` directly in a browser.
- Native local capture: see `BUILD.md`; requires Rust and target native build prerequisites. macOS screen capture requires user-granted Screen Recording permission.

## Architecture and security

The native IPC surface is limited to listing monitors and on-demand local capture. It has no network listener or input injection commands. Future remote capability requires threat modeling, authenticated pairing and explicit local approval, secure key lifecycle, reviewed encryption, platform permission UX, and tested transports before adding any remote-control path. See `docs/architecture.md` and `SECURITY.md`.

## License

MIT; see `LICENSE`.
