#!/usr/bin/env python3
"""Discover all available skills and display their metadata.

This script scans the .claude/skills/ directory and extracts
frontmatter from each SKILL.md file to provide a dynamic,
maintenance-free skill discovery system.
"""

import os
import re
import sys
from pathlib import Path


def find_skills_dir() -> Path:
    """Find the .claude/skills directory relative to repo root."""
    # Start from script location and walk up to find .claude
    current = Path(__file__).resolve()

    # Walk up to find repo root (where .claude exists)
    for parent in current.parents:
        skills_dir = parent / ".claude" / "skills"
        if skills_dir.is_dir():
            return skills_dir

    # Fallback: check common locations
    for path in [
        Path.cwd() / ".claude" / "skills",
        Path.home() / "contextium" / ".claude" / "skills",
    ]:
        if path.is_dir():
            return path

    print("Error: Could not find .claude/skills directory", file=sys.stderr)
    sys.exit(1)


def parse_frontmatter(content: str) -> dict:
    """Extract YAML frontmatter from SKILL.md content."""
    # Match content between --- markers
    match = re.match(r'^---\s*\n(.*?)\n---', content, re.DOTALL)
    if not match:
        return {}

    frontmatter = {}
    for line in match.group(1).split('\n'):
        if ':' in line:
            key, value = line.split(':', 1)
            frontmatter[key.strip()] = value.strip()

    return frontmatter


def discover_skills(skills_dir: Path) -> list[dict]:
    """Scan skills directory and extract metadata from each skill."""
    skills = []

    for item in sorted(skills_dir.iterdir()):
        if not item.is_dir():
            continue

        skill_md = item / "SKILL.md"
        if not skill_md.exists():
            continue

        try:
            content = skill_md.read_text()
            frontmatter = parse_frontmatter(content)

            if frontmatter.get('name'):
                skills.append({
                    'name': frontmatter.get('name', item.name),
                    'description': frontmatter.get('description', 'No description'),
                    'tools': frontmatter.get('allowed-tools', ''),
                    'path': str(item.relative_to(skills_dir.parent.parent)),
                })
        except Exception as e:
            print(f"Warning: Could not parse {skill_md}: {e}", file=sys.stderr)

    return skills


def format_output(skills: list[dict]) -> str:
    """Format skills for display."""
    lines = []
    lines.append("=" * 60)
    lines.append("AVAILABLE SKILLS")
    lines.append("=" * 60)
    lines.append("")

    for skill in skills:
        name = skill['name']
        desc = skill['description']

        # Truncate long descriptions
        if len(desc) > 100:
            # Find trigger info if present
            trigger_match = re.search(r'Auto-triggers? on:.*$', desc, re.IGNORECASE)
            if trigger_match:
                main_desc = desc[:desc.find('Auto-trigger')].strip()
                triggers = trigger_match.group(0)
                desc = f"{main_desc[:60]}... {triggers}"
            else:
                desc = desc[:97] + "..."

        lines.append(f"/{name}")
        lines.append(f"  {desc}")
        lines.append("")

    lines.append("=" * 60)
    lines.append(f"Total: {len(skills)} skills")
    lines.append("")
    lines.append("Use /<skill-name> to invoke a skill")
    lines.append("Skills auto-trigger based on keywords in their description")

    return '\n'.join(lines)


def main():
    skills_dir = find_skills_dir()
    skills = discover_skills(skills_dir)

    if not skills:
        print("No skills found.")
        sys.exit(0)

    print(format_output(skills))


if __name__ == "__main__":
    main()
