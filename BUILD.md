# Build and test

## Current dashboard

No compilation is required. Open `frontend/index.html` directly in a current desktop browser. It uses local CSS and JavaScript only and does not require a web server or internet connection. Test the dashboard manually by adding/removing a device record, toggling favorites and theme, refreshing the page, and checking that the Connect and WISP-test actions clearly report that no connection was attempted.

## Native builds

There is currently no Rust backend, Tauri shell, Windows installer/portable executable, macOS app bundle, or DMG in this repository. Consequently there are no native build commands, and no native artifact has been built or tested. Do not distribute this dashboard as if it provided remote desktop capabilities.

A native build pipeline must be added after the native backend and threat model are reviewed. Windows 10/11 x64 packaging should be built and tested on Windows; macOS Intel/Apple Silicon packaging, signing, notarization, screen-recording permission, and accessibility behavior require macOS build/test hosts. Cross-compilation does not replace target-platform permission and interaction testing.

## Required validation before native release

Document reproducible builds and test results for each supported OS and architecture. At minimum test authenticated pairing, explicit approval, capture and input permissions, monitor selection, network loss/recovery, LAN, configured WebRTC/STUN/TURN, clipboard consent, and revocation. Test only with devices and accounts you own or are authorized to administer. Never claim an installer exists unless the artifact was actually compiled and tested.
