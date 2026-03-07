#!/usr/bin/env python3
"""README generation script for tree-sitter-language-pack.

Generates READMEs from Jinja2 templates and YAML configuration.
Supports validation mode for CI, dry-run for previewing, and
per-language filtering.
"""

import argparse
import logging
import re
import sys
from pathlib import Path
from typing import Any

try:
    import yaml
except ImportError:
    print("Error: PyYAML is required. Install with: pip install pyyaml")
    sys.exit(1)

try:
    from jinja2 import Environment, FileSystemLoader, TemplateNotFound
except ImportError:
    print("Error: Jinja2 is required. Install with: pip install jinja2")
    sys.exit(1)


logging.basicConfig(level=logging.INFO, format="%(levelname)s: %(message)s")
logger = logging.getLogger(__name__)


class ReadmeGenerator:
    """Generates README files from Jinja2 templates and YAML config."""

    def __init__(self, project_root: Path) -> None:
        """Initialize generator with project root path.

        Args:
            project_root: Absolute path to the repository root.
        """
        self.project_root = project_root
        self.scripts_dir = project_root / "scripts"
        self.templates_dir = self.scripts_dir / "readme_templates"
        self.config: dict[str, Any] = {}
        self.jinja_env: Environment | None = None

    def load_config(self) -> dict[str, Any]:
        """Load and parse readme_config.yaml.

        Returns:
            Parsed configuration dictionary.

        Raises:
            FileNotFoundError: If config file is missing.
            ValueError: If config file is empty or invalid.
        """
        config_path = self.scripts_dir / "readme_config.yaml"

        if not config_path.exists():
            msg = f"Configuration file not found: {config_path}"
            raise FileNotFoundError(msg)

        with config_path.open(encoding="utf-8") as f:
            self.config = yaml.safe_load(f)

        if not self.config:
            msg = "Configuration file is empty"
            raise ValueError(msg)

        lang_count = len(self.config.get("languages", {}))
        logger.info("Loaded configuration with %d language targets", lang_count)
        return self.config

    def setup_jinja_env(self) -> Environment:
        """Configure Jinja2 environment.

        Returns:
            Configured Jinja2 Environment.

        Raises:
            FileNotFoundError: If templates directory is missing.
        """
        if not self.templates_dir.exists():
            msg = f"Templates directory not found: {self.templates_dir}"
            raise FileNotFoundError(msg)

        self.jinja_env = Environment(
            loader=FileSystemLoader(str(self.templates_dir)),
            keep_trailing_newline=True,
            trim_blocks=True,
            lstrip_blocks=False,
        )

        logger.debug("Jinja2 environment configured")
        return self.jinja_env

    @staticmethod
    def _normalize_markdown(text: str) -> str:
        """Post-process rendered markdown to fix common lint issues.

        Uses a line-by-line approach to properly handle fenced code blocks:
        - MD012: Collapses multiple consecutive blank lines into one.
        - MD031: Ensures blank lines before and after fenced code block fences.
        - Removes spurious blank lines immediately inside code fences.

        Args:
            text: Raw rendered markdown content.

        Returns:
            Normalized markdown string.
        """
        # Fix concatenated headings (e.g., "## Foo### Bar" -> "## Foo\n\n### Bar")
        text = re.sub(r"(#{1,6}\s+[^\n]+?)(#{1,6}\s+)", r"\1\n\n\2", text)

        # Ensure blank line before headings (MD022)
        text = re.sub(r"([^\n])\n(#{1,6}\s+)", r"\1\n\n\2", text)

        lines = text.split("\n")
        result: list[str] = []
        in_code_block = False

        for _i, line in enumerate(lines):
            stripped = line.strip()
            is_fence = stripped.startswith("```")

            if is_fence:
                if not in_code_block:
                    # Opening fence
                    # MD031: ensure blank line before opening fence
                    if result and result[-1].strip() != "" and not result[-1].strip().startswith("<!--"):
                        result.append("")
                    result.append(line)
                    in_code_block = True
                else:
                    # Closing fence
                    # Remove trailing blank lines inside code block
                    while result and result[-1].strip() == "":
                        result.pop()
                    result.append(line)
                    in_code_block = False
                continue

            if in_code_block:
                # Inside code block: skip leading blank lines right after opening fence
                if stripped == "" and result and result[-1].strip().startswith("```"):
                    continue
                result.append(line)
            else:
                # Outside code block
                # MD012: skip consecutive blank lines (keep at most one)
                if stripped == "" and result and result[-1].strip() == "":
                    continue
                # MD031: ensure blank line after a closing fence
                if stripped != "" and result and result[-1].strip().startswith("```"):
                    result.append("")
                result.append(line)

        return "\n".join(result)

    def _build_context(self, lang_code: str, lang_config: dict[str, Any]) -> dict[str, Any]:
        """Build template rendering context.

        Args:
            lang_code: Language identifier key from config.
            lang_config: Language-specific configuration dict.

        Returns:
            Merged context dictionary for template rendering.
        """
        return {
            "language": lang_code,
            "project": self.config.get("project", {}),
            "badges": self.config.get("badges", {}),
            **lang_config,
        }

    def generate_readme(
        self,
        lang_code: str,
        lang_config: dict[str, Any],
        output_path: Path,
        *,
        dry_run: bool = False,
    ) -> str:
        """Render a README from its template.

        Args:
            lang_code: Language identifier.
            lang_config: Language-specific configuration.
            output_path: Destination file path.
            dry_run: If True, skip writing to disk.

        Returns:
            Generated README content as a string.

        Raises:
            TemplateNotFound: If the specified template does not exist.
            RuntimeError: If rendering fails.
        """
        if self.jinja_env is None:
            msg = "Jinja2 environment not initialized"
            raise RuntimeError(msg)

        template_name = lang_config.get("template", f"{lang_code}.md.jinja")

        try:
            template = self.jinja_env.get_template(template_name)
        except TemplateNotFound as err:
            msg = f"Template not found: {template_name} (expected at {self.templates_dir / template_name})"
            raise TemplateNotFound(msg) from err

        context = self._build_context(lang_code, lang_config)

        try:
            content = template.render(**context)
        except Exception as exc:
            msg = f"Failed to render template {template_name}: {exc}"
            raise RuntimeError(msg) from exc

        content = self._normalize_markdown(content)

        if not dry_run:
            output_path.parent.mkdir(parents=True, exist_ok=True)
            output_path.write_text(content, encoding="utf-8")
            logger.info("Generated: %s", output_path.relative_to(self.project_root))
        else:
            logger.info("[DRY-RUN] Would generate: %s", output_path.relative_to(self.project_root))

        return content

    def generate_root_readme(self, *, dry_run: bool = False) -> str:
        """Generate the root README.md.

        Args:
            dry_run: If True, skip writing to disk.

        Returns:
            Generated root README content.
        """
        if self.jinja_env is None:
            msg = "Jinja2 environment not initialized"
            raise RuntimeError(msg)

        template = self.jinja_env.get_template("root.md.jinja")
        context = {
            "project": self.config.get("project", {}),
            "badges": self.config.get("badges", {}),
            "languages": self.config.get("languages", {}),
        }

        content = template.render(**context)
        content = self._normalize_markdown(content)
        output_path = self.project_root / "README.md"

        if not dry_run:
            output_path.write_text(content, encoding="utf-8")
            logger.info("Generated: README.md")
        else:
            logger.info("[DRY-RUN] Would generate: README.md")

        return content

    def validate_file(self, generated: str, file_path: Path) -> bool:
        """Check whether an existing file matches the generated content.

        Args:
            generated: Expected file content.
            file_path: Path to the existing file.

        Returns:
            True if the file matches, False otherwise.
        """
        if not file_path.exists():
            logger.warning("File not found: %s", file_path)
            return False

        existing = file_path.read_text(encoding="utf-8")
        if generated == existing:
            logger.info("Valid: %s", file_path.relative_to(self.project_root))
            return True

        logger.warning("Out of date: %s", file_path.relative_to(self.project_root))
        return False

    def process_all(
        self,
        *,
        language_filter: str | None = None,
        dry_run: bool = False,
        validate_only: bool = False,
    ) -> bool:
        """Process all configured README targets.

        Args:
            language_filter: If set, only process this language key.
            dry_run: Preview without writing.
            validate_only: Only validate existing files.

        Returns:
            True if all operations succeeded, False otherwise.
        """
        if not self.config:
            logger.error("Configuration not loaded")
            return False

        languages = self.config.get("languages", {})
        all_ok = True

        # Root README (skip if filtering to a specific language)
        if not language_filter:
            try:
                root_content = self.generate_root_readme(dry_run=dry_run or validate_only)
                if validate_only:
                    root_path = self.project_root / "README.md"
                    if not self.validate_file(root_content, root_path):
                        all_ok = False
            except Exception:
                logger.exception("Failed to process root README")
                all_ok = False

        # Filter languages if requested
        if language_filter:
            if language_filter not in languages:
                logger.error("Unknown language: %s", language_filter)
                logger.info("Available: %s", ", ".join(languages.keys()))
                return False
            languages = {language_filter: languages[language_filter]}

        # Per-package READMEs
        for lang_code, lang_config in languages.items():
            if not self._process_language(lang_code, lang_config, dry_run=dry_run, validate_only=validate_only):
                all_ok = False

        return all_ok

    def _process_language(
        self,
        lang_code: str,
        lang_config: dict,
        *,
        dry_run: bool = False,
        validate_only: bool = False,
    ) -> bool:
        """Process a single language README. Returns True on success."""
        try:
            if "output_path" in lang_config:
                readme_path = self.project_root / lang_config["output_path"]
            else:
                readme_path = self.project_root / "crates" / lang_code / "README.md"

            content = self.generate_readme(
                lang_code,
                lang_config,
                readme_path,
                dry_run=dry_run or validate_only,
            )

            if validate_only and not self.validate_file(content, readme_path):
                return False
        except Exception:
            logger.exception("Failed to process %s", lang_code)
            return False
        return True

    def run(self, args: argparse.Namespace) -> int:
        """Main entry point.

        Args:
            args: Parsed CLI arguments.

        Returns:
            Exit code: 0 for success, 1 for failure.
        """
        try:
            self.load_config()
            self.setup_jinja_env()

            success = self.process_all(
                language_filter=args.language,
                dry_run=args.dry_run,
                validate_only=args.validate,
            )

            if args.validate:
                if success:
                    logger.info("All READMEs are up-to-date")
                else:
                    logger.error("Some READMEs are out of date. Run: python scripts/generate_readme.py")
            elif success:
                logger.info("README generation completed successfully")
            else:
                logger.error("README generation completed with errors")

            return 0 if success else 1

        except Exception:
            logger.exception("Fatal error")
            return 1


def parse_args() -> argparse.Namespace:
    """Parse command-line arguments.

    Returns:
        Parsed argument namespace.
    """
    parser = argparse.ArgumentParser(
        description="Generate READMEs for tree-sitter-language-pack from templates",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Generate all READMEs
  python scripts/generate_readme.py

  # Generate only the Python package README
  python scripts/generate_readme.py --language python

  # Preview changes without writing
  python scripts/generate_readme.py --dry-run

  # Check if READMEs are up-to-date (for CI)
  python scripts/generate_readme.py --validate
        """,
    )

    parser.add_argument(
        "--language",
        help="Generate README for a specific language target only",
        metavar="LANG",
    )

    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Preview generation without writing to disk",
    )

    parser.add_argument(
        "--validate",
        action="store_true",
        help="Validate existing READMEs match generated output (exit 1 if not)",
    )

    parser.add_argument(
        "-v",
        "--verbose",
        action="store_true",
        help="Enable verbose output",
    )

    return parser.parse_args()


def main() -> int:
    """Entry point for the script.

    Returns:
        Exit code.
    """
    args = parse_args()

    if args.verbose:
        logger.setLevel(logging.DEBUG)

    project_root = Path(__file__).resolve().parent.parent
    generator = ReadmeGenerator(project_root)
    return generator.run(args)


if __name__ == "__main__":
    sys.exit(main())
