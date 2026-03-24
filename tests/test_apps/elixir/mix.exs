defmodule TestAppElixir.MixProject do
  use Mix.Project

  def project do
    [
      app: :test_app_elixir,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: false,
      deps: deps()
    ]
  end

  defp deps do
    [
      {:tree_sitter_language_pack, "~> 1.1.4"},
      {:jason, "~> 1.4"}
    ]
  end
end
