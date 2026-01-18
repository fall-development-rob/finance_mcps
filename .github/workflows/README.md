# GitHub Actions Workflows

This directory contains automated workflows for the Corp Finance MCP project.

## Workflows

### CI (`ci.yml`)
Runs on every push and pull request to main or claude/* branches.

**Jobs:**
1. **test-rust**: Runs Rust tests, formatting checks, and clippy lints
2. **test-typescript**: Type-checks TypeScript code
3. **build**: Builds all packages and uploads artifacts

**Triggers:**
- Push to `main` or `claude/**` branches
- Pull requests to `main`

### Release (`release.yml`)
Creates version PRs and publishes packages when changesets are merged.

**Jobs:**
1. Builds all packages (Rust + TypeScript)
2. Runs `changeset version` to update versions
3. Creates "Version Packages" PR if changesets exist
4. Publishes to GitHub Package Registry when version PR is merged
5. Creates GitHub Release with changelog

**Triggers:**
- Push to `main` branch (after PR merge)

**Required Permissions:**
- `contents: write` - Create releases
- `pull-requests: write` - Create version PRs
- `packages: write` - Publish to registry

### Publish (`publish.yml`)
Publishes packages to GitHub Package Registry.

**Jobs:**
1. Builds Rust core
2. Builds NAPI bindings
3. Builds TypeScript MCP server
4. Publishes both packages to registry

**Triggers:**
- Release published
- Manual workflow dispatch

**Required Permissions:**
- `packages: write` - Publish to registry

## Release Process

### 1. Create a changeset
```bash
npm run changeset
```

Follow the prompts to:
- Select which packages changed
- Choose version bump type (major/minor/patch)
- Describe the changes

This creates a markdown file in `.changeset/`.

### 2. Commit and push
```bash
git add .changeset
git commit -m "chore: add changeset for feature X"
git push
```

### 3. Merge PR
CI will run tests and build checks.

### 4. Version PR created
When your PR is merged to main, the `release` workflow:
- Consumes changesets
- Updates package versions
- Updates CHANGELOG.md
- Creates a "Version Packages" PR

### 5. Publish
When the "Version Packages" PR is merged:
- Packages are published to GitHub Package Registry
- A GitHub Release is created
- Tags are created (e.g., `v0.2.0`)

## Installing from GitHub Packages

### Setup authentication
Create a `.npmrc` file:
```
@corp-finance:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=YOUR_GITHUB_TOKEN
```

### Install packages
```bash
npm install @corp-finance/mcp-server
```

## Manual Publishing

If needed, you can manually publish:

```bash
# Build everything
npm run build

# Publish with changesets
npm run release
```

This requires:
1. GitHub token with `write:packages` permission
2. Proper `.npmrc` configuration

## Secrets Required

The workflows use these GitHub secrets:
- `GITHUB_TOKEN` - Automatically provided by GitHub Actions
  - Used for: creating PRs, publishing packages, creating releases

No manual secret configuration needed! GitHub Actions provides `GITHUB_TOKEN` automatically.

## Troubleshooting

### Build fails on bindings
- Ensure Rust toolchain is installed in CI
- Check that NAPI-RS dependencies are cached

### Publish fails
- Verify `publishConfig` in package.json
- Check that `GITHUB_TOKEN` has `write:packages` permission
- Ensure `.npmrc` is correctly configured

### Version PR not created
- Check that changesets exist in `.changeset/`
- Verify workflow has `pull-requests: write` permission
- Review workflow logs in GitHub Actions tab
