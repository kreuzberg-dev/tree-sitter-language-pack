defmodule TreeSitterLanguagePack.MixProject do
  use Mix.Project

  @version "2.0.0-alpha.1"

  def project do
    [
      app: :tree_sitter_language_pack,
      version: @version,
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.36"},
      {:ex_doc, "~> 0.31", only: :dev, runtime: false}
    ]
  end
end
