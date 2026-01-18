# Changesets

This directory contains [changesets](https://github.com/changesets/changesets) - descriptions of changes that should trigger version bumps and changelog entries.

## Creating a changeset

When you make changes that should be released, run:

```bash
pnpm changeset
```

This will:
1. Ask which packages have changed
2. Ask whether it's a major, minor, or patch change
3. Ask for a description of the changes
4. Create a changeset file in this directory

## Releasing

Changesets are automatically consumed by our GitHub Actions workflow when merged to main:

1. The workflow creates a "Version Packages" PR
2. When that PR is merged, packages are published to GitHub Package Registry
3. A GitHub Release is created with the changelog

## Manual release

To manually version and publish:

```bash
# Consume changesets and bump versions
pnpm changeset version

# Build packages
pnpm build

# Publish to registry
pnpm changeset publish
```
