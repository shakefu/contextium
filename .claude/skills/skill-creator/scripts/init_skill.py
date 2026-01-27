#!/usr/bin/env python3
"""Initialize a new Claude skill with proper structure."""

import argparse
import os
import sys
from pathlib import Path

SKILL_TEMPLATE = '''---
name: {name}
description: TODO: What this skill does AND when to trigger it. Include specific triggers, keywords, and use cases.
---

# {title}

TODO: Brief description of the skill's purpose.

## Usage

TODO: Core workflow or commands.

## Guidelines

TODO: Key guidelines for using this skill effectively.
'''

GITKEEP = ''


def create_skill(name: str, path: Path) -> None:
    """Create a new skill directory with template files."""
    skill_dir = path / name

    if skill_dir.exists():
        print(f"Error: Directory already exists: {skill_dir}", file=sys.stderr)
        sys.exit(1)

    # Create directory structure
    skill_dir.mkdir(parents=True)
    (skill_dir / "scripts").mkdir()
    (skill_dir / "references").mkdir()
    (skill_dir / "assets").mkdir()

    # Create SKILL.md
    title = name.replace("-", " ").title()
    skill_md = SKILL_TEMPLATE.format(name=name, title=title)
    (skill_dir / "SKILL.md").write_text(skill_md)

    # Create .gitkeep files for empty directories
    (skill_dir / "scripts" / ".gitkeep").write_text(GITKEEP)
    (skill_dir / "references" / ".gitkeep").write_text(GITKEEP)
    (skill_dir / "assets" / ".gitkeep").write_text(GITKEEP)

    print(f"Created skill: {skill_dir}")
    print(f"  - SKILL.md (edit the TODO sections)")
    print(f"  - scripts/ (add executable scripts)")
    print(f"  - references/ (add documentation)")
    print(f"  - assets/ (add output files)")
    print()
    print("Next steps:")
    print(f"  1. Edit {skill_dir}/SKILL.md")
    print("  2. Add resources as needed")
    print(f"  3. Validate with: validate_skill.py {skill_dir}")


def main():
    parser = argparse.ArgumentParser(
        description="Initialize a new Claude skill"
    )
    parser.add_argument(
        "name",
        help="Skill name (lowercase, hyphenated)"
    )
    parser.add_argument(
        "--path",
        type=Path,
        default=Path.cwd(),
        help="Parent directory for the skill (default: current directory)"
    )

    args = parser.parse_args()

    # Validate name
    if not args.name.replace("-", "").isalnum():
        print("Error: Skill name must be lowercase alphanumeric with hyphens", file=sys.stderr)
        sys.exit(1)

    if args.name != args.name.lower():
        print("Error: Skill name must be lowercase", file=sys.stderr)
        sys.exit(1)

    create_skill(args.name, args.path)


if __name__ == "__main__":
    main()
