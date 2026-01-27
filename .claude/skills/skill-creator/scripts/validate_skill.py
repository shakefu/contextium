#!/usr/bin/env python3
"""Validate a Claude skill for correctness and best practices."""

import argparse
import re
import sys
from pathlib import Path

# ANSI colors
GREEN = "\033[32m"
RED = "\033[31m"
YELLOW = "\033[33m"
RESET = "\033[0m"


def ok(msg: str) -> None:
    print(f"{GREEN}✓{RESET} {msg}")


def error(msg: str) -> None:
    print(f"{RED}✗{RESET} {msg}")


def warn(msg: str) -> None:
    print(f"{YELLOW}!{RESET} {msg}")


def parse_frontmatter(content: str) -> dict | None:
    """Extract YAML frontmatter from SKILL.md content."""
    if not content.startswith("---"):
        return None

    parts = content.split("---", 2)
    if len(parts) < 3:
        return None

    frontmatter = {}
    for line in parts[1].strip().split("\n"):
        if ":" in line:
            key, value = line.split(":", 1)
            frontmatter[key.strip()] = value.strip()

    return frontmatter


def validate_skill(skill_dir: Path) -> bool:
    """Validate a skill directory. Returns True if valid."""
    errors = 0
    warnings = 0

    print(f"\nValidating skill: {skill_dir.name}\n")

    # Check SKILL.md exists
    skill_md = skill_dir / "SKILL.md"
    if not skill_md.exists():
        error("SKILL.md not found (required)")
        return False
    ok("SKILL.md exists")

    # Read and parse content
    content = skill_md.read_text()
    lines = content.split("\n")

    # Check frontmatter
    frontmatter = parse_frontmatter(content)
    if frontmatter is None:
        error("Missing or invalid YAML frontmatter")
        errors += 1
    else:
        ok("Valid YAML frontmatter")

        # Check required fields
        if "name" not in frontmatter:
            error("Missing 'name' field in frontmatter")
            errors += 1
        else:
            name = frontmatter["name"]
            if name != name.lower() or " " in name:
                error(f"Name must be lowercase with hyphens: '{name}'")
                errors += 1
            else:
                ok(f"Name field valid: {name}")

        if "description" not in frontmatter:
            error("Missing 'description' field in frontmatter")
            errors += 1
        else:
            desc = frontmatter["description"]
            if len(desc) < 50:
                warn(f"Description seems short ({len(desc)} chars). Include what + when to trigger.")
                warnings += 1
            elif "TODO" in desc:
                error("Description contains TODO placeholder")
                errors += 1
            else:
                ok(f"Description field present ({len(desc)} chars)")

    # Check line count
    if len(lines) > 500:
        warn(f"SKILL.md has {len(lines)} lines (recommended: <500). Consider splitting to references/")
        warnings += 1
    else:
        ok(f"SKILL.md length OK ({len(lines)} lines)")

    # Check for TODO markers
    todo_count = content.count("TODO")
    if todo_count > 0:
        error(f"Found {todo_count} TODO markers - complete before use")
        errors += 1

    # Check for forbidden files
    forbidden = ["README.md", "CHANGELOG.md", "INSTALLATION_GUIDE.md", "QUICK_REFERENCE.md"]
    for f in forbidden:
        if (skill_dir / f).exists():
            warn(f"Found {f} - skills should not include auxiliary documentation")
            warnings += 1

    # Check scripts are executable (if any exist)
    scripts_dir = skill_dir / "scripts"
    if scripts_dir.exists():
        for script in scripts_dir.glob("*.py"):
            if script.name == ".gitkeep":
                continue
            if not script.stat().st_mode & 0o111:
                warn(f"Script not executable: {script.name}")
                warnings += 1

        for script in scripts_dir.glob("*.sh"):
            if not script.stat().st_mode & 0o111:
                warn(f"Script not executable: {script.name}")
                warnings += 1

    # Check references are cited in SKILL.md
    refs_dir = skill_dir / "references"
    if refs_dir.exists():
        for ref in refs_dir.glob("*.md"):
            if ref.name == ".gitkeep":
                continue
            if ref.name not in content:
                warn(f"Reference not cited in SKILL.md: {ref.name}")
                warnings += 1

    # Summary
    print()
    if errors == 0 and warnings == 0:
        print(f"{GREEN}Skill is valid!{RESET}")
        return True
    elif errors == 0:
        print(f"{YELLOW}Skill has {warnings} warning(s) but is usable{RESET}")
        return True
    else:
        print(f"{RED}Skill has {errors} error(s) and {warnings} warning(s){RESET}")
        return False


def main():
    parser = argparse.ArgumentParser(
        description="Validate a Claude skill"
    )
    parser.add_argument(
        "skill_dir",
        type=Path,
        help="Path to the skill directory"
    )

    args = parser.parse_args()

    if not args.skill_dir.is_dir():
        print(f"Error: Not a directory: {args.skill_dir}", file=sys.stderr)
        sys.exit(1)

    valid = validate_skill(args.skill_dir)
    sys.exit(0 if valid else 1)


if __name__ == "__main__":
    main()
