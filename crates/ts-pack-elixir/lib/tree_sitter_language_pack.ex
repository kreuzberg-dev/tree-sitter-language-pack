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

  @version "1.1.3"

  @force_build System.get_env("TSLP_BUILD") in ["1", "true"]

  use RustlerPrecompiled,
    otp_app: :tree_sitter_language_pack,
    crate: "ts-pack-elixir",
    path: ".",
    base_url:
      "https://github.com/kreuzberg-dev/tree-sitter-language-pack/releases/download/v#{@version}",
    version: @version,
    force_build: @force_build,
    skip_compilation?: not @force_build,
    targets: ~w(
      aarch64-apple-darwin
      aarch64-unknown-linux-gnu
      x86_64-unknown-linux-gnu
    ),
    nif_versions: ["2.16", "2.17"]

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
    * `config_json` - a JSON string containing at least `"language"`. Optional fields:
      `"structure"`, `"imports"`, `"exports"`, `"comments"`, `"docstrings"`,
      `"symbols"`, `"diagnostics"` (booleans, default true) and `"chunk_max_size"`
      (integer, optional).

  ## Returns

  A map containing the processing result with metadata and optionally chunks.
  """
  @spec process(String.t(), String.t()) :: map()
  def process(_source, _config_json), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Initializes the language pack with the given configuration JSON.

  Downloads specified languages/groups and applies cache settings.

  ## Parameters

    * `config_json` - JSON string with optional `cache_dir`, `languages`, `groups`
  """
  @spec init(String.t()) :: :ok | {:error, term()}
  def init(_config_json), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Applies download configuration without downloading anything.

  Use to set a custom cache directory before first language load.

  ## Parameters

    * `config_json` - JSON string with optional `cache_dir`
  """
  @spec configure(String.t()) :: :ok | {:error, term()}
  def configure(_config_json), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Downloads specific languages to the local cache.

  Returns the number of newly downloaded languages.

  ## Parameters

    * `names` - list of language name strings
  """
  @spec download([String.t()]) :: non_neg_integer()
  def download(_names), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Downloads all available languages from the remote manifest.

  Returns the number of newly downloaded languages.
  """
  @spec download_all() :: non_neg_integer()
  def download_all(), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Returns all language names available in the remote manifest.

  Fetches and caches the remote manifest to discover downloadable languages.
  """
  @spec manifest_languages() :: [String.t()]
  def manifest_languages(), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Returns languages that are already downloaded and cached locally.

  Does not perform any network requests.
  """
  @spec downloaded_languages() :: [String.t()]
  def downloaded_languages(), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Deletes all cached parser shared libraries.
  """
  @spec clean_cache() :: :ok | {:error, term()}
  def clean_cache(), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Returns the effective cache directory path as a string.
  """
  @spec cache_dir() :: String.t()
  def cache_dir(), do: :erlang.nif_error(:nif_not_loaded)
end
