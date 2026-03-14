defmodule Smoke.MixProject do
  use Mix.Project

  def project do
    [
      app: :smoke,
      version: "0.0.1",
      elixir: "~> 1.14",
      deps: deps()
    ]
  end

  defp deps do
    [
      {:tree_sitter_language_pack, "~> 1.0"}
    ]
  end
end
