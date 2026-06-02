import argparse
import os
import re
import subprocess
import sys


class VersionBumper:
    """Orchestrates version bumping across project files with robust error handling."""

    def __init__(self, root_dir: str, dry_run: bool = False, no_push: bool = False):
        self.root_dir = root_dir
        self.dry_run = dry_run
        self.no_push = no_push
        self.pyproject_path = os.path.join(root_dir, "pyproject.toml")
        self.skill_path = os.path.join(root_dir, "SKILL.md")
        self.prd_path = os.path.join(root_dir, "PRD.md")

    def bump(self, bump_type: str = "patch", set_version: str | None = None) -> None:
        """Execute the bump process."""
        if not os.path.exists(self.pyproject_path):
            print("❌ Error: pyproject.toml not found")
            sys.exit(1)

        with open(self.pyproject_path, 'r', encoding="utf-8") as f:
            content = f.read()

        match = re.search(r'^version\s*=\s*"(.*?)"', content, re.MULTILINE)
        if not match:
            print("❌ Error: Could not find version in pyproject.toml")
            sys.exit(1)

        old_version = match.group(1)
        if set_version:
            new_version = set_version
        else:
            new_version = self._calculate_next_version(old_version, bump_type)

        print(f"🔄 Bumping version: {old_version} -> {new_version} {'[DRY RUN]' if self.dry_run else ''}")

        # 1. Update pyproject.toml
        self._update_file(self.pyproject_path, [
            (r'(?m)^version\s*=\s*".*?"', f'version = "{new_version}"')
        ])

        # 2. Update SKILL.md
        self._update_file(self.skill_path, [
            (r'(?m)^version:\s*.*', f'version: {new_version}'),
            (r'Show current version \(.*?\)', f'Show current version ({new_version})')
        ])

        # 3. Update PRD.md
        self._update_file(self.prd_path, [
            (r'## Auto Linter MCP Server v.*', f'## Auto Linter MCP Server v{new_version}'),
            (r'\*\*Version\*\*:\s*.*', f'**Version**: {new_version}')
        ])

        if self.dry_run:
            print("✨ Dry run completed. No files modified, no git operations performed.")
            return

        # 4. Sync uv.lock
        print("⚡ Syncing uv.lock...")
        self._run_command(["uv", "lock"], critical=False)  # Don't crash if uv is not installed

        # Git operations
        print("🐙 Performing Git operations...")
        # Check if we are inside a Git repository
        git_check = self._run_command(["git", "rev-parse", "--is-inside-work-tree"], critical=False)
        if git_check.returncode != 0:
            print("⚠️ Warning: Not a git repository or git command failed. Skipping git commit.")
            return

        self._run_command(["git", "add", self.pyproject_path, self.skill_path, self.prd_path, os.path.join(self.root_dir, "uv.lock")], critical=True)
        status = self._run_command(["git", "status", "--porcelain"], critical=True)
        if status.stdout.strip():
            self._run_command(["git", "commit", "--no-verify", "-m", f"chore: bump version to {new_version}"], critical=True)
            if not self.no_push:
                self._run_command(["git", "tag", f"v{new_version}"], critical=True)
                self._run_command(["git", "push"], critical=True)
                self._run_command(["git", "push", "origin", f"v{new_version}"], critical=True)
                print(f"🚀 Successfully bumped version to {new_version}, committed, tagged v{new_version}, and pushed.")
            else:
                print(f"✅ Successfully bumped version to {new_version} and committed locally.")
        else:
            print("ℹ️ No changes to commit")

    def _calculate_next_version(self, current_version: str, bump_type: str) -> str:
        parts = current_version.split('.')
        if len(parts) == 3:
            try:
                major, minor, patch = int(parts[0]), int(parts[1]), int(parts[2])
                if bump_type == "major":
                    return f"{major + 1}.0.0"
                elif bump_type == "minor":
                    return f"{major}.{minor + 1}.0"
                else:  # patch
                    return f"{major}.{minor}.{patch + 1}"
            except ValueError:
                pass
        return current_version + ".1"

    def _run_command(self, cmd: list[str], critical: bool = False) -> subprocess.CompletedProcess:
        print(f"💻 Running: {' '.join(cmd)}")
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            print(f"⚠️ Warning: Command failed with exit code {result.returncode}")
            if result.stderr:
                print(f"Error output: {result.stderr.strip()}")
            if critical:
                print("❌ Critical error encountered. Halting version bump.")
                sys.exit(result.returncode)
        return result

    def _update_file(self, path: str, patterns: list[tuple[str, str]]) -> bool:
        if not os.path.exists(path):
            print(f"⚠️ Warning: {path} not found. Skipping.")
            return False

        with open(path, 'r', encoding="utf-8") as f:
            content = f.read()

        new_content = content
        for pattern, replacement in patterns:
            new_content = re.sub(pattern, replacement, new_content)

        if new_content == content:
            print(f"ℹ️ Info: No changes made to {path}")
            return False

        if not self.dry_run:
            with open(path, 'w', encoding="utf-8") as f:
                f.write(new_content)
        print(f"📝 Updated {path}")
        return True


def main():
    parser = argparse.ArgumentParser(description="Orchestrates version bumping across project files.")
    parser.add_argument(
        "--type",
        choices=["major", "minor", "patch"],
        default="patch",
        help="Type of version increment (default: patch)"
    )
    parser.add_argument(
        "--set-version",
        help="Explicitly set a specific version string"
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Perform all updates in memory and print messages, without modifying files or Git"
    )
    parser.add_argument(
        "--no-push",
        action="store_true",
        help="Skip pushing committed changes to remote repository"
    )

    args = parser.parse_args()

    root_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    bumper = VersionBumper(root_dir, dry_run=args.dry_run, no_push=args.no_push)
    bumper.bump(bump_type=args.type, set_version=args.set_version)


if __name__ == "__main__":
    main()
