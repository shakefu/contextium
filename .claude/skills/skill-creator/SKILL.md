---
name: skill-creator
description: Create Claude skills following best practices. Use when building new skills, packaging skill folders, or improving existing skills. Triggers on requests to create skills, make skills, build capabilities, or package skill directories.
allowed-tools: Bash, Read, Write, Edit, Task
---

# Skill Creator

Create effective, token-efficient Claude skills.

## Core Principles

1. **Concise** - Challenge every token. Only add what Claude doesn't already know.
2. **Appropriate freedom** - Match specificity to task fragility (see references/workflows.md)
3. **Progressive disclosure** - Metadata → SKILL.md → references (loaded as needed)
4. **Favor subagents** - Design skills to use Task tool for parallel work where beneficial

## Skill Structure

```
skill-name/
├── SKILL.md              # Required: frontmatter + instructions
├── scripts/              # Optional: executable code
├── references/           # Optional: documentation loaded as needed
└── assets/               # Optional: files used in output
```

## Creation Workflow

1. **Understand** - Gather concrete usage examples
2. **Plan** - Identify reusable resources (scripts/references/assets)
3. **Initialize** - Run `scripts/init_skill.py <name> --path <dir>`
4. **Implement** - Create resources, write SKILL.md
5. **Validate** - Run `scripts/validate_skill.py <skill-dir>`
6. **Register** - Add skill to `.claude/claude.md` under "Auto-Triggered Skills"
7. **Package** - Run `scripts/package_skill.py <skill-dir>` (creates .skill file)
8. **Iterate** - Improve based on real usage

Use subagents (Task tool) to parallelize research during the Understand phase.

## Writing SKILL.md

### Frontmatter (Critical)

```yaml
---
name: lowercase-hyphenated
description: What it does AND when to trigger. Include specific triggers, keywords, use cases. This is the PRIMARY triggering mechanism.
allowed-tools: Read, Edit, Write, Task  # Include Task for subagent support
---
```

The `description` must contain ALL "when to use" info - body is only loaded AFTER triggering.
Include `Task` in allowed-tools to enable subagent parallelization.

### Body Guidelines

- Keep under 500 lines; split to references if longer
- Use imperative form ("Run the script" not "You should run")
- For multi-step processes, give overview early
- For output formats, use templates or examples (see references/output-patterns.md)

## Resource Types

| Type | When to Use | Example |
|------|-------------|---------|
| `scripts/` | Same code rewritten repeatedly; deterministic reliability needed | `rotate_pdf.py` |
| `references/` | Documentation Claude should reference while working | `schema.md` |
| `assets/` | Files used in output, not loaded into context | `template.pptx` |

## Do NOT Include

- README.md, CHANGELOG.md, INSTALLATION_GUIDE.md
- Auxiliary documentation not needed by Claude
- Duplicate info (lives in SKILL.md OR references, not both)

## Validation Checklist

- [ ] `name` and `description` in frontmatter
- [ ] Description includes what + when to trigger
- [ ] SKILL.md under 500 lines
- [ ] Scripts tested and working
- [ ] No extraneous files
- [ ] References cited in SKILL.md

See `references/workflows.md` for workflow patterns and `references/output-patterns.md` for output formatting.
