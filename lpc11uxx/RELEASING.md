# Releasing

Set variables:

    $ export CRATE=$(cat Cargo.toml | grep name | sed 's/.*"\([^"]*\)".*/\1/')
    $ export VERSION=X.Y.Z
    $ export GPG_KEY=F001234

Update version numbers:

    $ vim Cargo.toml

Update changelog:

    $ vim CHANGELOG.md

Commit & tag:

    $ git commit -S${GPG_KEY} -m "${CRATE}: Release v${VERSION}"
    $ git tag -s -u ${GPG_KEY} ${CRATE}-v${VERSION} -m "${CRATE}: Version ${VERSION}"

Publish:

    $ cargo publish
    $ git push && git push --tags
