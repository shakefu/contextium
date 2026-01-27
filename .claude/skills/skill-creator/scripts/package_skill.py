#!/usr/bin/env python3
"""Package a Claude skill into a distributable .skill file."""

import argparse
import subprocess
import sys
import zipfile
from pathlib import Path

# ANSI colors
GREEN = "\033[32m"
RED = "\033[31m"
RESET = "\033[0m"


def package_skill(skill_dir: Path, output_dir: Path) -> bool:
    """Package a skill directory into a .skill file."""

    # First validate the skill
    validate_script = Path(__file__).parent / "validate_skill.py"
    if validate_script.exists():
        print("Running validation...")
        result = subprocess.run(
            [sys.executable, str(validate_script), str(skill_dir)],
            capture_output=True,
            text=True
        )
        print(result.stdout)
        if result.returncode != 0:
            print(f"{RED}Validation failed. Fix errors before packaging.{RESET}")
            return False

    skill_name = skill_dir.name
    output_file = output_dir / f"{skill_name}.skill"

    print(f"\nPackaging {skill_name}...")

    # Create the .skill file (a zip archive)
    with zipfile.ZipFile(output_file, 'w', zipfile.ZIP_DEFLATED) as zf:
        for file_path in skill_dir.rglob('*'):
            if file_path.is_file():
                arcname = file_path.relative_to(skill_dir)

                # Skip hidden files and __pycache__ within the skill
                if any(part.startswith('.') or part == '__pycache__'
                       for part in arcname.parts):
                    continue

                zf.write(file_path, arcname)
                print(f"  Added: {arcname}")

    print(f"\n{GREEN}Created: {output_file}{RESET}")
    print(f"Size: {output_file.stat().st_size:,} bytes")

    return True


def main():
    parser = argparse.ArgumentParser(
        description="Package a Claude skill into a .skill file"
    )
    parser.add_argument(
        "skill_dir",
        type=Path,
        help="Path to the skill directory"
    )
    parser.add_argument(
        "output_dir",
        type=Path,
        nargs="?",
        default=Path.cwd(),
        help="Output directory for .skill file (default: current directory)"
    )

    args = parser.parse_args()

    if not args.skill_dir.is_dir():
        print(f"Error: Not a directory: {args.skill_dir}", file=sys.stderr)
        sys.exit(1)

    if not (args.skill_dir / "SKILL.md").exists():
        print(f"Error: No SKILL.md found in {args.skill_dir}", file=sys.stderr)
        sys.exit(1)

    args.output_dir.mkdir(parents=True, exist_ok=True)

    success = package_skill(args.skill_dir, args.output_dir)
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
