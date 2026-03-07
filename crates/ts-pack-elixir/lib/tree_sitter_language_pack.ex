defmodule TreeSitterLanguagePack do
  @moduledoc """
  Elixir bindings for tree-sitter-language-pack.

  Provides access to 165+ tree-sitter language parsers through
  Rustler NIFs backed by `ts-pack-core`.

  ## Usage

      # List all available languages
      TreeSitterLanguagePack.available_languages()
      #=> ["python", "rust", "javascript", ...]

      # Check if a language is available
      TreeSitterLanguagePack.has_language("python")
      #=> true

      # Get the number of available languages
      TreeSitterLanguagePack.language_count()
      #=> 165

      # Get the raw TSLanguage pointer (for interop with tree-sitter bindings)
      TreeSitterLanguagePack.get_language_ptr("python")
      #=> 140234567890

  """

  use Rustler, otp_app: :tree_sitter_language_pack, crate: "ts_pack_elixir"

  @doc """
  Returns a list of all available language names.
  """
  @spec available_languages() :: [String.t()]
  def available_languages(), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Checks whether a language with the given name is available.
  """
  @spec has_language(String.t()) :: boolean()
  def has_language(_name), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Returns the total number of available languages.
  """
  @spec language_count() :: non_neg_integer()
  def language_count(), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Returns the raw `TSLanguage` pointer as an integer.

  This is useful for interop with Elixir tree-sitter bindings that
  accept a language pointer.

  Raises on error if the language is not found.
  """
  @spec get_language_ptr(String.t()) :: non_neg_integer()
  def get_language_ptr(_name), do: :erlang.nif_error(:nif_not_loaded)
end
