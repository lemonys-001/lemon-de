# Publishing a release

The `Build release binaries` workflow is the single release pipeline.

## Cut a version

1. Update `[package].version` in `Cargo.toml` and regenerate/update `Cargo.lock` as needed. Keep the two versions consistent.
2. Merge the version change to `main` and wait for CI.
3. Create and push a version tag pointing at that commit, for example:

   ```sh
   git tag v0.4.2
   git push origin v0.4.2
   ```

4. The tag push starts `.github/workflows/release-binaries.yml`. It checks that `vX.Y.Z` matches `Cargo.toml`, builds each target, packages `lemon-vX.Y.Z-TARGET.tar.gz` plus a `.sha256` sidecar, and automatically creates/updates the GitHub Release with all the successful packages.

A failed architecture build fails the build job and prevents the release publish job from running, so users are not shown a release with a partial target set. Fix or remove a failing target before retrying the tag workflow.

## Manual builds

Use **Actions → Build release binaries → Run workflow** to build from a selected branch. A manual dispatch creates downloadable Actions artifacts only; it does not create or modify a GitHub Release. Extract each artifact ZIP and upload the `.tar.gz` and matching `.sha256` file if adding those binaries to a release manually.

The installer expects release asset names exactly as produced by the workflow. If no matching binary is attached for a machine, it falls back to building the tagged source locally.
