# Automatic updates

Decode CAD's release pipeline already produces platform installers and a `SHA256SUMS.txt` file. This workflow adds a machine-readable `latest.json` to each **published** GitHub Release. It does not install software or silently replace files on a user's computer; the desktop updater still needs to consume and verify this manifest.

## Manifest contract

The workflow at `.github/workflows/update-manifest.yml` runs when a GitHub Release is published and attaches `latest.json` to that same release. The manifest includes the product name, version/tag, release URL, prerelease flag, and downloadable assets with their sizes and SHA-256 hashes. Assets without a hash are omitted rather than advertised as verifiable downloads.

The stable manifest URL is:

`https://github.com/Kaiqueaurelio/decodecad/releases/latest/download/latest.json`

That URL follows GitHub's latest **published non-prerelease** release. A release that is still a draft does not trigger the workflow; publish it when it is ready for users.

## Safe updater rollout

1. **Release metadata (this change):** publish checksummed artifact metadata, without changing how installed apps behave.
2. **In-app update notice:** the desktop app periodically checks the stable manifest and offers the version, release notes and a link to the appropriate package. Network errors must never prevent the CAD app from starting.
3. **Platform-specific installation:** use the Windows installer upgrade path for MSI installations; use AppImageUpdate-compatible deltas for AppImage; let the native package manager update .deb/.rpm/Flatpak installations. Do not overwrite files owned by a package manager.
4. **Verification and tests:** validate the manifest schema, reject untrusted hosts, compare downloaded bytes with SHA-256 before opening an installer, handle offline mode and prereleases deliberately, and test upgrade/downgrade and user-data preservation on each supported OS.

**Security note:** SHA-256 detects accidental or in-transit changes only when the manifest itself is trusted. A production in-app installer must also rely on HTTPS, strict host allowlists and platform code-signing/signature validation. Never execute a downloaded file solely because it appears in the manifest.

## How to publish

Use the existing release process in [releasing.md](releasing.md). Once the draft release is published, this workflow attaches `latest.json`. Check the Actions run and confirm the manifest is present before announcing the release. The manifest is a foundation for an updater, not proof that the app already updates itself.
