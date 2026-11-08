# Contributing

Thanks for helping build Relay. The project is currently an offline dashboard and must not be represented as a working remote desktop application.

## Before contributing

- Read `README.md`, `BUILD.md`, and `SECURITY.md`.
- Keep the standalone interface self-contained: no mandatory CDN, server, or internet dependency.
- Do not add pretend connection success, fake device status, fabricated history, or controls that imply unavailable native functionality works.
- Do not add remote-control listeners or transport until the authentication, consent, threat-model, and protocol requirements in `docs/architecture.md` are addressed.
- Never commit secrets, credentials, signing keys, or user data.

## Changes

Keep changes focused and document platform limitations. For dashboard changes, test direct `file://` loading, narrow layouts, persistence, keyboard-accessible controls, and visible error states. For future native components, include reproducible test instructions and report exactly which OS versions and architectures were actually tested.

## License

By contributing, you agree that your contributions are offered under the repository's MIT license.
