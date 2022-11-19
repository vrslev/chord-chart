"""
Check the version in Cargo.toml matches the version from `GITHUB_REF` environment variable.
Based on script from pydantic-core: https://github.com/pydantic/pydantic-core/blob/e146483e2becb6151e76539861db0053465bb22a/.github/check_version.py
"""
import json
import os
import re
from pathlib import Path
from typing import Any

import tomllib  # type: ignore


def check_version(tag: str, file: str, file_path: Path):
    assert (
        file == tag
    ), f'GITHUB_REF version "{tag}" does not match {file_path} version "{file}"'
    print(f'✓ GITHUB_REF version matches {file_path} version "{file}"')


def main():
    cargo_toml = Path("Cargo.toml")
    with cargo_toml.open("rb") as f:
        content: Any = tomllib.load(f)  # type: ignore
        py = content["workspace"]["package"]["version"]
    package_json = Path("chord-chart-js/package.json")
    with package_json.open() as f:
        js = json.load(f)["version"]

    assert (ref := os.getenv("GITHUB_REF")), "no GITHUB_REF"  # type: ignore
    tag = re.sub(f"^refs/tags/v*", "", ref.lower())

    check_version(tag=tag, file=py, file_path=cargo_toml)
    check_version(tag=tag, file=js, file_path=package_json)


if __name__ == "__main__":
    main()
