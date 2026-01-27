# Output Patterns

## Template Pattern

Use for strict output requirements:

```markdown
## Report Structure

ALWAYS use this exact template:

# [Title]

## Executive Summary
[One paragraph overview]

## Key Findings
- Finding 1 with data
- Finding 2 with data

## Recommendations
1. Actionable item
2. Actionable item
```

## Examples Pattern

Use when style matters more than exact structure:

```markdown
## Commit Message Format

Follow these examples:

**Example 1:**
Input: Added user authentication
Output:
feat(auth): implement JWT authentication
Add login endpoint and token middleware

**Example 2:**
Input: Fixed date display bug
Output:
fix(reports): correct timezone handling
Use UTC timestamps consistently

Style: type(scope): brief description, then details.
```

## Schema Pattern

Use for structured data output:

```markdown
## Output Schema

Generate JSON matching this structure:

{
  "id": "string (uuid)",
  "status": "pending" | "active" | "complete",
  "items": [
    {
      "name": "string",
      "value": "number"
    }
  ],
  "metadata": {
    "created": "ISO 8601 timestamp",
    "version": "semver string"
  }
}
```

## Hybrid Pattern

Combine template + examples for complex outputs:

```markdown
## API Response Format

Structure:
{
  "success": boolean,
  "data": <varies by endpoint>,
  "error": string | null
}

Examples:

Success:
{"success": true, "data": {"user": "alice"}, "error": null}

Failure:
{"success": false, "data": null, "error": "User not found"}
```

## Progressive Detail Pattern

Start simple, add complexity as needed:

```markdown
## Quick Start

Basic usage:
run_tool input.txt

## Options

--format json|csv    Output format (default: json)
--verbose           Show detailed progress

## Advanced

For batch processing, see references/batch-processing.md
```
