# Build and test

## Standalone dashboard

Open `frontend/index.html` directly in a modern browser. All assets are local; no build step or network is needed.

## Native local-capture and approval-gate milestone

The Tauri 2 / Rust desktop shell enumerates displays and captures a still image only after the user presses **Capture screen**. A fail-closed approval broker is ready for a future native networking module: that module can authenticate a peer, submit the peer identity and requested access, and await an explicit local approval decision. The UI shows the device name, ID, operating system, requested access, and a 60-second expiration; closing, timeout, UI failure, or denial does not authorize access.

**There is currently no network listener or transport module.** This gate does not receive external connections by itself. Approving a future request only returns consent to its caller; it does not start a session or authorize unauthenticated access.

### Prerequisites

- Rust stable with the target platform's native toolchain.
- Tauri 2 prerequisites: WebView2 and Microsoft C++ build tools on Windows; Xcode command-line tools on macOS.
- For macOS, grant Relay Screen Recording permission in System Settings > Privacy & Security > Screen Recording and restart the app if capture is denied.

### Build

From the repository root, install the Tauri CLI if needed (`cargo install tauri-cli --version '^2'`), then run:

```sh
cargo tauri dev --manifest-path src-tauri/Cargo.toml
cargo tauri build --manifest-path src-tauri/Cargo.toml
```

Build on Windows for Windows installers and on macOS for Mac app/DMG artifacts. macOS Intel and Apple Silicon need their respective native targets/toolchains, signing identities, and notarization for distribution. No release artifact is checked in or claimed built/tested by this change.

## Validation

No build was run in the hosted repository-editing environment. Build and manually test on Windows 10/11 x64 and macOS Intel/Apple Silicon before distributing. Verify monitor enumeration, explicit still-image capture, permission-denied messaging, approval prompt data and approve/deny/timeout behavior using an authenticated test transport, and that no session starts without approval. This milestone does not satisfy remote session, authentication, transport, or control tests.
