# Relay — staged open-source remote desktop project

Relay is being built in verified stages. The repository currently contains a standalone dashboard and a native desktop milestone for user-requested local still-image screen capture, plus a fail-closed local authorization gate for a future authenticated transport.

## What works in this milestone

- `frontend/index.html`: standalone offline dashboard (file://) for local device labels and preferences.
- `src-tauri/` + `desktop-ui/`: Tauri 2 / Rust desktop shell. The native backend enumerates displays and captures one still image only when the user explicitly presses Capture. The image is rendered in the same local app window.
- Native authorization broker: future trusted transport code can submit an authenticated peer's identity and requested access, then must wait for the user's explicit approve/deny decision. The app displays device name, device ID, operating system, and requested access. Requests expire after 60 seconds; timeout, prompt failure, or closed UI denies access. Approval alone does not start a session.
- OSI-approved MIT license and security/development documentation.

## Not implemented

This is not yet a remote desktop product. There is no network listener or incoming connection service, authenticated pairing/identity protocol, remote screen streaming, mouse/keyboard control, clipboard synchronization, LAN discovery, WebRTC/STUN/TURN, WISP, reconnection, or remote session. Therefore no real external device can currently initiate a connection or trigger the approval dialog. No Windows/macOS artifacts have been built or tested here.

The approval gate is a native backend hook, not a way to accept arbitrary network requests. Future transport code must authenticate the peer before requesting approval and must not proceed without an explicit user allow decision. WISP remains out of scope until an exact, versioned protocol and server implementation are selected.

## Run

- Dashboard: open `frontend/index.html` directly in a browser.
- Native local capture and approval UI: see `BUILD.md`; requires Rust and target native build prerequisites. macOS screen capture requires user-granted Screen Recording permission.

## Security

The current app has no network listener and no input injection commands. The approval handler is not callable by the frontend to fabricate inbound requests; only native backend code can create pending requests. Treat an approval prompt as consent, not proof of identity—the future transport must authenticate the device first. See `docs/architecture.md` and `SECURITY.md`.

## License

MIT; see `LICENSE`.
