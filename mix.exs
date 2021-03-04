defmodule Oso.MixProject do
  use Mix.Project

  def project do
    [
      app: :oso,
      version: "0.1.0",
      elixir: "~> 1.10",
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
      {:rustler, "~> 0.22-rc"}
    ]
  end
end
