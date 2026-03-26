"""Smoke tests for tree-sitter-language-pack published package."""

from __future__ import annotations

import json
from pathlib import Path

import pytest
import tree_sitter_language_pack as tslp

FIXTURES_DIR = Path(__file__).parent.parent / "fixtures"


@pytest.fixture(scope="session", autouse=True)
def _download_languages() -> None:
    """Download required languages before running tests."""
    tslp.download(
        [
            "python",
            "javascript",
            "rust",
            "go",
            "ruby",
            "java",
            "c",
            "cpp",
            "csharp",
            "vb",
            "embeddedtemplate",
        ]
    )


def load_fixtures(name: str) -> list[dict]:
    return json.loads((FIXTURES_DIR / name).read_text())


class TestBasic:
    """Validate basic language discovery API."""

    @pytest.fixture(autouse=True)
    def _load_fixtures(self) -> None:
        self.fixtures = load_fixtures("basic.json")

    def test_package_imports(self) -> None:
        assert hasattr(tslp, "available_languages")
        assert hasattr(tslp, "has_language")
        assert hasattr(tslp, "language_count")

    @pytest.mark.parametrize(
        "fixture",
        load_fixtures("basic.json"),
        ids=lambda f: f["name"],
    )
    def test_basic_fixture(self, fixture: dict) -> None:
        match fixture["test"]:
            case "language_count":
                count = tslp.language_count()
                assert count >= fixture["expected_min"], (
                    f"language_count {count} < expected min {fixture['expected_min']}"
                )
            case "has_language":
                result = tslp.has_language(fixture["language"])
                assert result == fixture["expected"], (
                    f"has_language({fixture['language']!r}) = {result}, expected {fixture['expected']}"
                )
            case "available_languages":
                langs = tslp.available_languages()
                for lang in fixture["expected_contains"]:
                    assert lang in langs, f"available_languages missing {lang!r}"
            case other:
                pytest.fail(f"Unknown test type: {other}")


class TestErrorHandling:
    """Validate error handling for invalid inputs."""

    def test_invalid_language_process(self) -> None:
        config = tslp.ProcessConfig(language="nonexistent_xyz_123")
        with pytest.raises(Exception):
            tslp.process("some code", config)

    def test_has_language_returns_false_for_invalid(self) -> None:
        assert tslp.has_language("nonexistent_xyz_123") is False


class TestGetLanguage:
    """Validate get_language() for languages with c_symbol overrides (#80).

    These languages were broken in <=1.3.1 due to a dynamic library naming
    mismatch. Fixed in 1.3.2. Tests are marked xfail for 1.3.1.
    """

    @pytest.mark.parametrize(
        "language",
        ["csharp", "vb", "embeddedtemplate"],
        ids=["csharp", "vb", "embeddedtemplate"],
    )
    def test_get_language_returns_non_none(self, language: str) -> None:
        """get_language() should return a valid language object, not None."""
        result = tslp.get_language(language)
        assert result is not None, f"get_language({language!r}) returned None"

    @pytest.mark.parametrize(
        "language",
        ["csharp", "vb", "embeddedtemplate"],
        ids=["csharp", "vb", "embeddedtemplate"],
    )
    def test_get_parser_for_previously_broken_languages(self, language: str) -> None:
        """get_parser() should return a usable parser for previously broken languages."""
        parser = tslp.get_parser(language)
        assert parser is not None, f"get_parser({language!r}) returned None"

    @pytest.mark.parametrize(
        "language",
        ["csharp", "vb", "embeddedtemplate"],
        ids=["csharp", "vb", "embeddedtemplate"],
    )
    def test_has_language_for_previously_broken(self, language: str) -> None:
        """has_language() should return True for previously broken languages."""
        assert tslp.has_language(language), f"has_language({language!r}) returned False"


class TestDownloadAPI:
    """Validate download and configuration API."""

    def test_api_surface(self) -> None:
        """Verify all download-related functions are exposed."""
        for fn_name in [
            "init",
            "download",
            "download_all",
            "configure",
            "manifest_languages",
            "downloaded_languages",
            "clean_cache",
            "cache_dir",
            "DownloadError",
        ]:
            assert hasattr(tslp, fn_name), f"Missing API: {fn_name}"

    def test_downloaded_languages_returns_list(self) -> None:
        """Verify downloaded_languages() returns a list."""
        result = tslp.downloaded_languages()
        assert isinstance(result, list)

    def test_cache_dir_returns_string(self) -> None:
        """Verify cache_dir() returns a non-empty string."""
        result = tslp.cache_dir()
        assert isinstance(result, str)
        assert len(result) > 0

    def test_manifest_languages_returns_list(self) -> None:
        """Verify manifest_languages() returns a list."""
        result = tslp.manifest_languages()
        assert isinstance(result, list)
        # The manifest should contain a reasonable number of languages
        assert len(result) > 50
