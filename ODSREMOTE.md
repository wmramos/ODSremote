# ODSremote

Remote support for OneDot Systems: https://help.onedotsystems.com.
This is an AGPL-3.0 RustDesk source fork, not a RustDesk-endorsed product.
The globe/circuit logo was supplied by OneDot Systems; its PNG transparency is
preserved. SVG placeholders are not used. The Windows ICO is a format conversion.
Upstream base: 1.5.0, commit fada664df7a294d1d1a9ca3e7cd3637069122f17.
The original LICENCE, copyright and About license notices are retained.
Source: https://github.com/wmramos/ODSremote.

## Windows Editions

- **Quick Support** (`ODSremote-1.5.0-windows-x64-qs.exe`): portable, incoming-only, requires local
  approval. Installation and account login are disabled. Uses its own
  `ODSremoteQS` configuration and IPC identity, separate from the Agent.
- **Agent** (`ODSremote-1.5.0-windows-x64-agent-install.exe`): shows an installation dialog.
  An authorized installation provides the normal Windows background service.
  Starts with click approval; unattended access must be explicitly configured by
  the device owner using the client's password and approval settings.

Portable means no persistent installed Agent. It still extracts files, retains
local configuration, and can request UAC approval for an elevated helper. It is
not zero-footprint and is not a browser-only host. Closing Quick Support ends its
availability. Windows restart, UAC, session cleanup and coexistence with Agent
must be tested before distributing it to customers.

Both profiles pin the established OneDot ID/relay/API server and public key.
No account tokens or shared unattended passwords are embedded. Automatic upstream
binary updates are disabled. The installed Agent registers `odsremote://`, used
by the help portal. Quick Support has the separate `odsremoteqs://` identity.
Renaming an executable cannot enable installation in the Quick Support profile.

## Build And Release

Release numbers follow the upstream RustDesk version in the root `Cargo.toml`,
read using the existing `build.py` version helper. Current release naming:

- Release title: **ODSremote 1.5.0**.
- Release tag: **odsremote-v1.5.0** (separate from inherited upstream tags).
- Quick Support: `ODSremote-1.5.0-windows-x64-qs.exe`.
- Agent: `ODSremote-1.5.0-windows-x64-agent-install.exe`.

The workflow derives artifact names, file names, wrapper metadata and its release
summary from this single version. Updating the upstream base updates these names
automatically; edition names do not introduce independent release versions.
The final `-qs.exe` and `-install.exe` suffixes preserve the native startup modes.

The `ODSremote Windows` workflow builds Windows x64 only, with separate compile
features for each edition. It uploads unsigned **test artifacts**, not public
releases. Run it manually or push to `codex/odsremote-windows`. Existing upstream
workflows are retained for provenance and should stay disabled in this fork.

Signing is not configured. Do not publish these artifacts as production downloads
until both editions have passed tests on clean Windows machines and are signed
with OneDot's certificate. The workflow deliberately does not enable the portal
downloads or publish a release automatically. Include the matching source commit,
LICENCE and notices with any release. Optional virtual-monitor and printer-driver
bundles are not included in this first build.

This source fork does not use or bypass RustDesk's signed custom-client generator.
The ODSremote feature initializes its own compiled profile; custom.txt is not
consumed by these builds. Feature-off builds retain upstream behavior.

## Regression Surface

Release naming changes only `.github/workflows/odsremote-windows.yml` (artifact
packaging and release metadata), `libs/portable/build.rs` (OriginalFilename),
and this document. Installed executable paths, service identity, client
configuration, networking and portal routing are unchanged by the naming update.

- `Cargo.toml` and `src/lib.rs`: additive feature gates and client-local module.
- `src/common.rs`: branded initialization at the existing custom-client entry
  point, covering GUI and service startup, plus refusal of upstream update checks;
  feature-off follows the stock path.
- `build.py`: optional edition selector for the existing Windows Flutter build.
- `libs/portable/build.rs`: branded wrapper metadata only when the edition env is
  set. Packer protocol, remote protocol and shared hbb_common are unchanged.
- Flutter About/footer website links: OneDot support link only for ODSremote names;
  upstream license, privacy statement and copyright notices remain visible.
- `branding/prepare.ps1`: stages original assets and product metadata only for
  branded CI builds. Stock assets in the checkout are not replaced in the commit.

Current status: source and build workflow prepared; no client binary is yet
certified for distribution. A successful CI compile is not a live-session test.
