# Security policy

## Current scope

The current release is a static local dashboard only. It has no remote-control backend, network listener, signaling server, authentication service, or capture/input functionality. Device labels and preferences are stored in browser local storage and are not encrypted; do not enter credentials, passwords, private keys, or other secrets.

## Reporting a vulnerability

Please use GitHub's private vulnerability reporting feature for this repository if it is enabled. Otherwise contact the repository maintainers privately through GitHub. Include affected revision, impact, and safe reproduction details. Avoid public disclosure until a fix or coordinated disclosure plan is available.

## Future security requirements

Any future native release must include a threat model and protocol review. Remote control must require authenticated device pairing, explicit local approval, encrypted and authenticated transport, revocable credentials, safe key storage, and clear session indicators. Do not ship unauthenticated remote control, expose a public control port, or silently bypass operating-system permissions. WISP compatibility must name and follow a specific protocol; no generic WISP compatibility is assumed.
