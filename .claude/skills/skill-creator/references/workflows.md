# Workflow Patterns

## Degrees of Freedom

Match specificity to task fragility:

| Freedom | Use When | Format |
|---------|----------|--------|
| **High** | Multiple valid approaches, context-dependent | Text instructions |
| **Medium** | Preferred pattern exists, some variation OK | Pseudocode with parameters |
| **Low** | Fragile operations, consistency critical | Specific scripts, few params |

## Sequential Workflow Pattern

For multi-step processes, give overview first:

```markdown
## Process Overview

Creating a report involves:
1. Validate input data (run validate.py)
2. Generate structure (run generate.py)
3. Apply formatting (run format.py)
4. Export final output

## Step 1: Validate Input
[details]

## Step 2: Generate Structure
[details]
```

## Conditional Workflow Pattern

When different paths apply:

```markdown
## Determine Approach

1. Identify the modification type:
   - **Creating new?** → Follow "Creation Workflow"
   - **Editing existing?** → Follow "Edit Workflow"

## Creation Workflow
1. [step]
2. [step]

## Edit Workflow
1. [step]
2. [step]
```

## Decision Tree Pattern

For complex branching:

```markdown
## Select Approach

Input type?
├── Structured data → Use schema validation
│   ├── JSON → scripts/validate_json.py
│   └── CSV → scripts/validate_csv.py
└── Unstructured text → Use heuristic analysis
    └── Run scripts/analyze_text.py
```

## Error Handling Pattern

```markdown
## Troubleshooting

If validation fails:
1. Check error message for specific field
2. Verify input matches schema in references/schema.md
3. Run `scripts/debug.py <input>` for detailed analysis

Common errors:
- `MISSING_FIELD`: Required field absent → Add field per schema
- `TYPE_MISMATCH`: Wrong data type → Convert to expected type
```
