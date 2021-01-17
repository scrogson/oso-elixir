defmodule OsoTest do
  use ExUnit.Case
  doctest Oso

  test "greets the world" do
    assert Oso.hello() == :world
  end
end
