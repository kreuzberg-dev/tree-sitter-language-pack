"""Allowlist validation for values from ``sources/language_definitions.json``.

Every field validated here is handed to ``git`` — as a clone URL, an
``ls-remote`` positional, or a checkout target. Git treats a leading ``-`` as an
option, so an entry such as ``--upload-pack=sh -c '...'`` or an ``ext::`` URL is
remote code execution at vendor-sync time rather than a bad URL. Validate with
an allowlist before any value reaches a git invocation. ~keep

Shared by ``clone_vendors.py``, ``pin_vendors.py`` and ``check_grammar_updates.py``.
"""

from __future__ import annotations

import re
from typing import Any

ALLOWED_REPO_HOSTS: tuple[str, ...] = ("github.com", "gitlab.com")
"""Forges grammar sources may be fetched from. Add a host here to allow it."""

_REPO_URL_PATTERN = re.compile(
    r"^https://(?:" + "|".join(re.escape(host) for host in ALLOWED_REPO_HOSTS) + r")/[\w.\-]+/[\w.\-]+$"
)
_REV_PATTERN = re.compile(r"^[0-9a-f]{40}$")
_BRANCH_PATTERN = re.compile(r"^[\w.\-/]+$")


class InvalidLanguageSourceError(ValueError):
    """Raised when a language definition carries a value git must not receive."""


def validate_repo_url(language_name: str, repo_url: Any) -> str:
    """Validate a grammar repository URL against the forge allowlist.

    Args:
        language_name: Language the URL belongs to, used in the error message.
        repo_url: The candidate URL.

    Returns:
        The validated URL.

    Raises:
        InvalidLanguageSourceError: If the URL is not an ``https`` URL on an
            allowed forge, or contains a ``..`` path segment.
    """
    if not isinstance(repo_url, str) or not _REPO_URL_PATTERN.match(repo_url) or ".." in repo_url:
        allowed = ", ".join(ALLOWED_REPO_HOSTS)
        raise InvalidLanguageSourceError(
            f"{language_name}: invalid repo URL {repo_url!r}. "
            f"Expected https://<host>/<owner>/<name> where <host> is one of: {allowed}. "
            f"Values are passed to git verbatim, so anything else (a leading '-', an 'ext::' "
            f"or 'ssh://' URL, a '..' segment) is rejected."
        )
    return repo_url


def validate_rev(language_name: str, rev: Any) -> str:
    """Validate a pinned revision is a full 40-character lowercase hex SHA.

    Args:
        language_name: Language the revision belongs to, used in the error message.
        rev: The candidate revision.

    Returns:
        The validated revision.

    Raises:
        InvalidLanguageSourceError: If the revision is not a 40-hex SHA.
    """
    if not isinstance(rev, str) or not _REV_PATTERN.match(rev):
        raise InvalidLanguageSourceError(
            f"{language_name}: invalid rev {rev!r}. Expected a full 40-character lowercase hex commit SHA; "
            f"revisions are passed to `git checkout` verbatim."
        )
    return rev


def validate_branch(language_name: str, branch: Any) -> str:
    """Validate a branch name contains only ref-safe characters.

    Args:
        language_name: Language the branch belongs to, used in the error message.
        branch: The candidate branch name.

    Returns:
        The validated branch name.

    Raises:
        InvalidLanguageSourceError: If the branch name is empty, starts with ``-``,
            or contains characters outside ``[A-Za-z0-9_.\\-/]``.
    """
    if not isinstance(branch, str) or not _BRANCH_PATTERN.match(branch) or branch.startswith("-") or ".." in branch:
        raise InvalidLanguageSourceError(
            f"{language_name}: invalid branch {branch!r}. Expected a plain ref name matching [A-Za-z0-9_.-/]+."
        )
    return branch


def validate_language_definition(language_name: str, definition: dict[str, Any]) -> None:
    """Validate every git-bound field of a single language definition.

    In-repo (``local``) grammars have no upstream fields and are skipped.

    Args:
        language_name: The language key.
        definition: The language definition mapping.

    Raises:
        InvalidLanguageSourceError: If any git-bound field fails validation.
    """
    if definition.get("local"):
        return
    validate_repo_url(language_name, definition.get("repo"))
    if "rev" in definition:
        validate_rev(language_name, definition["rev"])
    if "branch" in definition:
        validate_branch(language_name, definition["branch"])


def validate_language_definitions(definitions: dict[str, Any]) -> None:
    """Validate every remote language definition, reporting all offenders at once.

    Args:
        definitions: The parsed ``language_definitions.json`` mapping.

    Raises:
        InvalidLanguageSourceError: If one or more definitions fail validation.
    """
    failures: list[str] = []
    for language_name, definition in definitions.items():
        try:
            validate_language_definition(language_name, definition)
        except InvalidLanguageSourceError as error:  # noqa: PERF203
            failures.append(str(error))
    if failures:
        joined = "\n  ".join(failures)
        raise InvalidLanguageSourceError(f"rejected {len(failures)} language definition(s):\n  {joined}")
