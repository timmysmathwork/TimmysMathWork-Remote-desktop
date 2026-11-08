# Architecture and implementation plan

## Current implementation boundary

`frontend/index.html`, `style.css`, and `app.js` are a file://-compatible dashboard. Browser storage persists local labels/preferences. The static page intentionally cannot capture the operating-system screen or inject global mouse/keyboard input; it also does not establish remote sessions. UI state and saved endpoint text must never be treated as proof of authentication, device availability, or secure transport.

The native Tauri app can enumerate displays and perform a still capture at the user's request. It also has an in-process fail-closed approval broker. There is no network listener, peer authentication, transport, session streaming, or input injection, so the approval event is not currently produced by any real connection source.

## Native authorization gate

A future native transport module must authenticate a peer first, then call the internal (non-Tauri-command) `request_connection_approval` API with the authenticated device's name, stable ID, OS, and requested access. The API emits a request to the local UI and blocks until a decision, timeout, or UI failure. A random request ID maps the decision back to its waiting task. The UI offers only explicit approve and deny actions, presents the peer details, and denies on timeout/closed UI/errors. The Tauri command can respond only to a currently pending request ID; it cannot create one. Never interpret approval as authentication or start a session before the awaited result is `true`.

Do not put passwords, tokens, private keys, or other secrets in request metadata. Validate identity claims cryptographically in the transport; UI display fields alone can be spoofed. Avoid using device name as identity.

## Proposed native boundary

- **UI:** local HTML/CSS/JS presentation and input validation.
- **Native service:** narrow, typed IPC commands for permission state, device identity, session lifecycle, and stream/control messages. Enforce authorization in Rust, never in the frontend alone.
- **Platform adapters:** explicit capture/input/clipboard APIs with visible permission handling for Windows and macOS. Default to no control until local user approval; do not suppress OS prompts or hide active sessions.
- **Identity/security:** device key lifecycle, authenticated pairing, revocation, secure local key storage, protocol versioning, replay protection, and authenticated encryption. Obtain independent review before release.
- **Transport:** separately tested LAN and documented WebRTC signaling/STUN/TURN paths. Select transports only when configured and authenticated. A relay must not grant control by virtue of connectivity.
- **Optional WISP adapter:** implement only against a named, versioned protocol specification and compatible server. Require explicit configuration/consent, authenticated WSS, bounded timeouts, redacted diagnostics, and fallback only after configured methods fail. No assumption of universal WISP interoperability.

## Security and consent invariants

1. No unauthenticated remote control or public unauthenticated listener.
2. A new controlling device requires authenticated pairing and explicit approval at the controlled computer.
3. Display the requesting device's authenticated identity, OS, and requested permissions before approval. A new session starts only after an explicit allow; deny, timeout, app closure, and errors fail closed.
4. Show a persistent local session indicator and provide immediate disconnect/revoke controls.
5. Respect OS screen-recording and accessibility/input permissions. Explain how users grant them and report actionable failures; never claim permission is available based on frontend state.
6. Make network endpoints and relay use visible. Avoid collecting clipboard contents or credentials unless explicitly consented to and necessary.
7. Treat retries, resume, device names, signaling data, and diagnostics as untrusted input; avoid logging secrets.

## Release gates

Before claiming a feature works, add platform-specific implementation and automated/manual test evidence. A native release requires Windows 10/11 x64 and macOS Intel/Apple Silicon builds, signing and packaging instructions, permission testing, and end-to-end tests across the combinations listed in the product request. Current repository code does not meet those release gates.
