# Architecture and implementation plan

## Current implementation boundary

`frontend/index.html`, `style.css`, and `app.js` are a file://-compatible dashboard. Browser storage persists local labels/preferences. The static page intentionally cannot capture the operating-system screen or inject global mouse/keyboard input; it also does not establish remote sessions. UI state and saved endpoint text must never be treated as proof of authentication, device availability, or secure transport.

## Proposed native boundary (not implemented)

A future native application should separate:

- **UI:** local HTML/CSS/JS presentation and input validation.
- **Native service:** narrow, typed IPC commands for permission state, device identity, session lifecycle, and stream/control messages. Enforce authorization in Rust, never in the frontend alone.
- **Platform adapters:** explicit capture/input/clipboard APIs with visible permission handling for Windows and macOS. Default to no control until local user approval; do not suppress OS prompts or hide active sessions.
- **Identity/security:** device key lifecycle, authenticated pairing, revocation, secure local key storage, protocol versioning, replay protection, and authenticated encryption. Obtain independent review before release.
- **Transport:** separately tested LAN and documented WebRTC signaling/STUN/TURN paths. Select transports only when configured and authenticated. A relay must not grant control by virtue of connectivity.
- **Optional WISP adapter:** implement only against a named, versioned protocol specification and compatible server. Require explicit configuration/consent, authenticated WSS, bounded timeouts, redacted diagnostics, and fallback only after configured methods fail. No assumption of universal WISP interoperability.

## Security and consent invariants

1. No unauthenticated remote control or public unauthenticated listener.
2. A new controlling device requires authenticated pairing and explicit approval at the controlled computer.
3. Show a persistent local session indicator and provide immediate disconnect/revoke controls.
4. Respect OS screen-recording and accessibility/input permissions. Explain how users grant them and report actionable failures; never claim permission is available based on frontend state.
5. Make network endpoints and relay use visible. Avoid collecting clipboard contents or credentials unless explicitly consented to and necessary.
6. Treat retries, resume, device names, signaling data, and diagnostics as untrusted input; avoid logging secrets.

## Release gates

Before claiming a feature works, add platform-specific implementation and automated/manual test evidence. A native release requires Windows 10/11 x64 and macOS Intel/Apple Silicon builds, signing and packaging instructions, dependency/license inventory, permission testing, and end-to-end tests across the combinations listed in the product request. Current repository code does not meet those release gates.
