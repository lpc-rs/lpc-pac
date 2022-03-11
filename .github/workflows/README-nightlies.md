# LPC Peripheral Access Crates - Nightly Builds

This repository contains automated builds of the [lpc-pac] crates, rebuilt
whenever a PR is merged to the master branch. Consult the [lpc-pac] README
for full details.

[lpc-pac]: https://github.com/lpc-rs/lpc-pac

## Using These Builds

Edit your `Cargo.toml`:

```toml
[dependencies.lpc546xx-pac]
git = "https://github.com/lpc-rs/lpc-pac-nightlies"
features = ["lpc54608", "rt"]
```

The nightlies should always build and be as stable as the latest release, but
typically with the latest patches and updates.


## Using These Builds With Cargo.lock

Since no commit history is stored in this repository (to keep download sizes
small), if you depend on a specific git commit (for example if one ends up
in your `Cargo.lock` file) it will eventually be removed from this repository,
breaking your build until you update.

For local development builds this shouldn't be a problem, but for CI systems
it might be annoying. The recommended solution is to fork this repository;
your fork will not update automatically and so will persist the commit you
use until you manually update the fork (either by pushing a new commit to it
or by deleting and re-forking it).
