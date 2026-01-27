---
name: cicd-setup
description: Create and configure CI/CD pipelines with best practices. Triggers on requests to set up CI/CD, add GitHub Actions, create pipelines, automate testing/deployment, configure workflows, add continuous integration, or automate releases.
allowed-tools: Read, Edit, Write, Task, Glob, Grep, Bash
---

# CI/CD Setup

Create production-ready CI/CD pipelines following industry best practices.

## Workflow

### 1. Analyze Project (Parallel)

Spawn two agents simultaneously:

**Platform Detector (Explore agent):**
```
Detect CI/CD platform for this project:
1. Check for existing workflow files (.github/workflows/, .gitlab-ci.yml, .circleci/)
2. Check for GitHub/GitLab remote
3. Return: platform type, existing workflows, recommendations
```

**Tech Stack Analyzer (Explore agent):**
```
Analyze project tech stack:
1. Identify languages (package.json, Cargo.toml, go.mod, pyproject.toml, etc.)
2. Identify test frameworks
3. Identify build tools
4. Check for Dockerfile
5. Return: languages, test commands, build commands, docker usage
```

### 2. Generate Workflows

Based on analysis, spawn generation agents (parallel if both needed):

**CI Workflow Generator (general-purpose):**
- Linting configuration
- Test execution with coverage
- Build verification
- Caching strategy (dependencies, build artifacts)
- Matrix testing (multiple versions where appropriate)

**CD Workflow Generator (general-purpose):**
- Release automation
- Deployment configuration
- Environment-specific workflows
- Required secrets documentation

### 3. Write and Document

- Write workflow files to appropriate location
- Document required secrets and setup steps
- Provide manual verification commands

## Best Practices Applied

### Security
- Never hardcode secrets (use repository secrets)
- Pin action versions with SHA (not `@v1`)
- Use minimal permissions (`permissions:` block)
- Add security scanning (dependabot, CodeQL where appropriate)

### Performance
- Cache dependencies aggressively
- Use matrix builds sparingly (cost vs coverage)
- Fail fast on critical checks
- Run expensive jobs only on relevant changes (path filters)

### Reliability
- Use `timeout-minutes` to prevent hung jobs
- Add retry logic for flaky network operations
- Use `concurrency` to prevent duplicate runs
- Test workflows in feature branches first

### Maintainability
- Keep workflows focused (separate CI from CD)
- Use reusable workflows for shared logic
- Document non-obvious steps with comments
- Use meaningful job and step names

## Platform Reference

### GitHub Actions

Location: `.github/workflows/*.yml`

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

permissions:
  contents: read

jobs:
  test:
    runs-on: ubuntu-latest
    timeout-minutes: 10
    steps:
      - uses: actions/checkout@v4
      - name: Setup and test
        run: |
          # Setup commands
          # Test commands
```

### GitLab CI

Location: `.gitlab-ci.yml`

```yaml
stages:
  - test
  - build
  - deploy

test:
  stage: test
  script:
    - # test commands
  cache:
    paths:
      - node_modules/  # or equivalent
```

## Common Patterns

### Node.js CI
```yaml
- uses: actions/setup-node@v4
  with:
    node-version-file: '.nvmrc'
    cache: 'npm'
- run: npm ci
- run: npm test
```

### Python CI
```yaml
- uses: actions/setup-python@v5
  with:
    python-version-file: '.python-version'
    cache: 'pip'
- run: pip install -e ".[dev]"
- run: pytest
```

### Rust CI
```yaml
- uses: dtolnay/rust-toolchain@stable
- uses: Swatinem/rust-cache@v2
- run: cargo test
```

### Go CI
```yaml
- uses: actions/setup-go@v5
  with:
    go-version-file: 'go.mod'
- run: go test ./...
```

### Docker Build
```yaml
- uses: docker/setup-buildx-action@v3
- uses: docker/build-push-action@v5
  with:
    context: .
    push: false
    cache-from: type=gha
    cache-to: type=gha,mode=max
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Workflow not triggering | Check branch names, path filters, event types |
| Cache not working | Verify cache key includes lockfile hash |
| Secrets not available | Check if job has `environment:` or is on fork |
| Permission denied | Add explicit `permissions:` block |

See `references/advanced-patterns.md` for reusable workflows, matrix builds, release automation, and security scanning patterns.

## Output Checklist

After setup, verify:
- [ ] Workflow triggers on correct events
- [ ] Tests pass locally first
- [ ] Required secrets documented
- [ ] Caching configured correctly
- [ ] Job timeouts set
- [ ] Permissions minimized
