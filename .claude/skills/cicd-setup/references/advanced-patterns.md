# Advanced CI/CD Patterns

## Supply Chain Security

### Artifact Attestations (SLSA Level 3)

Generate tamper-proof provenance for builds:

```yaml
permissions:
  id-token: write
  contents: read
  attestations: write

steps:
  - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2

  - name: Build
    run: npm run build

  - uses: actions/attest-build-provenance@6149ea5740be74af77f260b9db67e633f6b0a9a1 # v1.4.2
    with:
      subject-path: 'dist/**'
```

### Container Signing with Cosign

```yaml
- uses: sigstore/cosign-installer@dc72c7d5c4d10cd6bcb8cf6e3fd625a9e5e537da # v3.7.0

- name: Sign container image
  env:
    COSIGN_EXPERIMENTAL: "true"  # Keyless signing via OIDC
  run: |
    cosign sign --yes ${{ env.REGISTRY }}/${{ env.IMAGE }}@${{ steps.build.outputs.digest }}
```

### SBOM Generation

```yaml
- uses: anchore/sbom-action@f5d3f78c0a0478ee4c9f00f1e63d76b2e5ba6ee6 # v0.17.8
  with:
    artifact-name: sbom.spdx.json
    output-file: sbom.spdx.json
```

## Reusable Workflows

Define in `.github/workflows/reusable-ci.yml`:

```yaml
name: Reusable CI

on:
  workflow_call:
    inputs:
      node-version:
        type: string
        default: '20'

permissions:
  contents: read

jobs:
  test:
    runs-on: ubuntu-latest
    timeout-minutes: 15
    steps:
      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2
      - uses: actions/setup-node@39370e3970a6d050c480ffad4ff0ed4d3fdee5af # v4.1.0
        with:
          node-version: ${{ inputs.node-version }}
          cache: 'npm'
      - run: npm ci
      - run: npm test
```

Call from another workflow:

```yaml
jobs:
  ci:
    uses: ./.github/workflows/reusable-ci.yml
    with:
      node-version: '22'
```

## Matrix Builds (Use Sparingly)

Only matrix what you actually need - each cell costs runner time:

```yaml
jobs:
  test:
    strategy:
      fail-fast: true  # Stop on first failure
      matrix:
        os: [ubuntu-latest]  # Only add more if you deploy to them
        node: [20, 22]       # LTS versions only
    runs-on: ${{ matrix.os }}
    timeout-minutes: 15
    steps:
      - uses: actions/setup-node@39370e3970a6d050c480ffad4ff0ed4d3fdee5af # v4.1.0
        with:
          node-version: ${{ matrix.node }}
```

## Path Filters

Skip CI for docs-only changes:

```yaml
on:
  push:
    branches: [main]
    paths-ignore:
      - '**.md'
      - 'docs/**'
      - '.github/ISSUE_TEMPLATE/**'
  pull_request:
    branches: [main]
    paths-ignore:
      - '**.md'
      - 'docs/**'
```

## Release Automation

### Tag-Based Release

```yaml
name: Release

on:
  push:
    tags: ['v*']

permissions:
  contents: write

jobs:
  release:
    runs-on: ubuntu-latest
    timeout-minutes: 15
    steps:
      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2

      - name: Build
        run: npm run build

      - uses: softprops/action-gh-release@e7a8f85e1c67a31e6ed99a94b41bd0b71bbee6b8 # v2.0.9
        with:
          generate_release_notes: true
          files: dist/*
```

## Environment Protection

```yaml
jobs:
  deploy-staging:
    runs-on: ubuntu-latest
    environment: staging
    steps:
      - run: ./deploy.sh staging

  deploy-production:
    needs: deploy-staging
    runs-on: ubuntu-latest
    environment:
      name: production
      url: https://example.com
    steps:
      - run: ./deploy.sh production
```

Configure in GitHub: Settings → Environments → Add required reviewers, wait timers, branch restrictions.

## Docker Build with Caching

```yaml
- uses: docker/setup-buildx-action@b5ca514318bd6ebac0fb2aedd5d36ec1b5c232a2 # v3.7.1

- uses: docker/build-push-action@471d1dc4e07e5cdedd4c2171150001c434f0b7a4 # v6.15.0
  with:
    context: .
    push: ${{ github.event_name != 'pull_request' }}
    tags: ${{ env.REGISTRY }}/${{ env.IMAGE }}:${{ github.sha }}
    cache-from: type=gha
    cache-to: type=gha,mode=max
```

## Caching Dependencies

Use built-in caching in setup actions when available:

```yaml
# Node.js - built-in cache
- uses: actions/setup-node@39370e3970a6d050c480ffad4ff0ed4d3fdee5af # v4.1.0
  with:
    cache: 'npm'  # Automatically caches based on lockfile

# Manual cache for other cases
- uses: actions/cache@1bd1e32a3bdc45362d1e726936510720a7c30a57 # v4.2.0
  with:
    path: |
      ~/.cache/pip
      ~/.local/share/virtualenvs
    key: ${{ runner.os }}-pip-${{ hashFiles('**/requirements*.txt') }}
    restore-keys: |
      ${{ runner.os }}-pip-
```

## Security Scanning

### CodeQL (for supported languages)

```yaml
name: CodeQL

on:
  push:
    branches: [main]
  schedule:
    - cron: '0 0 * * 1'  # Weekly

permissions:
  security-events: write
  contents: read

jobs:
  analyze:
    runs-on: ubuntu-latest
    timeout-minutes: 30
    steps:
      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2

      - uses: github/codeql-action/init@48ab28a6f5dbc2a99bf1e0131198dd8f1df78169 # v3.28.0
        with:
          languages: javascript  # or python, go, java, etc.

      - uses: github/codeql-action/analyze@48ab28a6f5dbc2a99bf1e0131198dd8f1df78169 # v3.28.0
```

## Monorepo Patterns

Run only affected packages:

```yaml
jobs:
  changes:
    runs-on: ubuntu-latest
    outputs:
      packages: ${{ steps.filter.outputs.changes }}
    steps:
      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2
      - uses: dorny/paths-filter@de90cc6fb38fc0963ad72b210f1f284cd68cea36 # v3.0.2
        id: filter
        with:
          filters: |
            api:
              - 'packages/api/**'
            web:
              - 'packages/web/**'

  test:
    needs: changes
    if: needs.changes.outputs.packages != '[]'
    strategy:
      matrix:
        package: ${{ fromJSON(needs.changes.outputs.packages) }}
    runs-on: ubuntu-latest
    steps:
      - run: npm test --workspace=${{ matrix.package }}
```
