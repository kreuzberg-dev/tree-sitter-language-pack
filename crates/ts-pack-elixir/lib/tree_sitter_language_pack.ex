defmodule TreeSitterLanguagePack do
  @moduledoc """
  Elixir bindings for tree-sitter-language-pack.

  Provides access to 165+ tree-sitter language parsers through
  Rustler NIFs backed by `ts-pack-core`.

  Language names are plain strings such as `"python"`, `"rust"`, `"javascript"`,
  etc. Use `available_languages/0` to discover all supported names at runtime,
  or `has_language/1` to check for a specific language before loading it.

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

  ## Error Handling

  Functions that accept a language name return `{:error, {:language_not_found, name}}`
  if the language is not found (e.g. `get_language_ptr/1`). Use `has_language/1` to
  check availability before calling these functions if you want to avoid handling errors.
  """

  use Rustler,
    otp_app: :tree_sitter_language_pack,
    crate: "ts-pack-elixir",
    path: ".",
    skip_compilation?: true,
    load_from: {:tree_sitter_language_pack, "priv/native/ts_pack_elixir"}

  @typedoc """
  A language name string such as `"python"`, `"rust"`, or `"javascript"`.

  Use `available_languages/0` to discover all valid language names at runtime.
  """
  @type language_name :: String.t()

  @doc """
  Returns a list of all available language names.

  ## Examples

      iex> languages = TreeSitterLanguagePack.available_languages()
      iex> is_list(languages) and length(languages) > 0
      true
  """
  @spec available_languages() :: [language_name()]
  def available_languages(), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Checks whether a language with the given name is available.

  Returns `true` if the language is supported, `false` otherwise.

  ## Examples

      TreeSitterLanguagePack.has_language("python")
      #=> true

      TreeSitterLanguagePack.has_language("nonexistent")
      #=> false
  """
  @spec has_language(language_name()) :: boolean()
  def has_language(_name), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Returns the total number of available languages.

  ## Examples

      iex> TreeSitterLanguagePack.language_count() > 0
      true
  """
  @spec language_count() :: non_neg_integer()
  def language_count(), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Returns the raw `TSLanguage` pointer as a non-negative integer.

  This is useful for interop with Elixir tree-sitter bindings that
  accept a language pointer.

  ## Parameters

    * `name` - the language name (e.g. `"python"`, `"rust"`)

  ## Returns

  A non-negative integer representing the memory address of the
  native `TSLanguage` struct.

  ## Errors

  Returns `{:error, {:language_not_found, name}}` if the language is not found.

  ## Examples

      ptr = TreeSitterLanguagePack.get_language_ptr("python")
      # => 140234567890
  """
  @spec get_language_ptr(language_name()) ::
          non_neg_integer() | {:error, {:language_not_found, language_name()}}
  def get_language_ptr(_name), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Parses a source string using the named language and returns an opaque tree reference.

  ## Parameters

    * `language` - the language name (e.g. `"python"`, `"rust"`)
    * `source` - the source code string to parse

  ## Returns

  An opaque tree reference that can be passed to tree inspection functions.

  ## Errors

  Returns `{:error, {:language_not_found, name}}` if the language is not found.
  Returns `{:error, {:parse_error, reason}}` if parsing fails.

  ## Examples

      tree = TreeSitterLanguagePack.parse_string("python", "def hello(): pass")
  """
  @spec parse_string(language_name(), String.t()) :: reference()
  def parse_string(_language, _source), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Returns the type name of the root node of the parsed tree.

  ## Examples

      tree = TreeSitterLanguagePack.parse_string("python", "x = 1")
      TreeSitterLanguagePack.tree_root_node_type(tree)
      #=> "module"
  """
  @spec tree_root_node_type(reference()) :: String.t()
  def tree_root_node_type(_tree), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Returns the number of named children of the root node.

  ## Examples

      tree = TreeSitterLanguagePack.parse_string("python", "x = 1\\ny = 2")
      TreeSitterLanguagePack.tree_root_child_count(tree)
      #=> 2
  """
  @spec tree_root_child_count(reference()) :: non_neg_integer()
  def tree_root_child_count(_tree), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Checks whether any node in the tree has the given type name.

  Uses a depth-first traversal.

  ## Examples

      tree = TreeSitterLanguagePack.parse_string("python", "def hello(): pass")
      TreeSitterLanguagePack.tree_contains_node_type(tree, "function_definition")
      #=> true
  """
  @spec tree_contains_node_type(reference(), String.t()) :: boolean()
  def tree_contains_node_type(_tree, _node_type), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Checks whether the tree contains any ERROR or MISSING nodes.

  ## Examples

      tree = TreeSitterLanguagePack.parse_string("python", "def (broken @@@ !!!")
      TreeSitterLanguagePack.tree_has_error_nodes(tree)
      #=> true
  """
  @spec tree_has_error_nodes(reference()) :: boolean()
  def tree_has_error_nodes(_tree), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Processes source code and extracts file intelligence as a map.

  ## Parameters

    * `source` - the source code string to process
    * `language` - the language name (e.g. `"python"`, `"rust"`)

  ## Returns

  A map containing file intelligence: language, structure, imports, metrics, diagnostics.
  """
  @spec process(String.t(), language_name()) :: map()
  def process(_source, _language), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Processes and chunks source code, returning intelligence and chunks as a map.

  ## Parameters

    * `source` - the source code string to process
    * `language` - the language name (e.g. `"python"`, `"rust"`)
    * `max_chunk_size` - the maximum chunk size in bytes

  ## Returns

  A map with `"intelligence"` and `"chunks"` keys.
  """
  @spec process_and_chunk(String.t(), language_name(), non_neg_integer()) :: map()
  def process_and_chunk(_source, _language, _max_chunk_size),
    do: :erlang.nif_error(:nif_not_loaded)
end
