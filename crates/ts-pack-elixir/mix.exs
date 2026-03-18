defmodule TreeSitterLanguagePack.MixProject do
  use Mix.Project

  @version "1.0.0-rc.9"

  def project do
    [
      app: :tree_sitter_language_pack,
      version: @version,
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      description:
        "Elixir bindings for tree-sitter-language-pack, providing access to 165+ language parsers",
      package: package(),
      source_url: "https://github.com/kreuzberg-dev/tree-sitter-language-pack"
    ]
  end

  defp package do
    [
      maintainers: ["kreuzberg-dev"],
      licenses: ["MIT"],
      links: %{
        "GitHub" => "https://github.com/kreuzberg-dev/tree-sitter-language-pack"
      },
      files: ~w(lib mix.exs README.md LICENSE)
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.37"},
      {:ex_doc, "~> 0.31", only: :dev, runtime: false}
    ]
  end
end
