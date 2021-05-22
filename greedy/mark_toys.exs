# https://www.hackerrank.com/challenges/mark-and-toys

defmodule Solution do
  def main() do
    [_n, k] = IO.read(:stdio, :line)
      |> String.trim()
      |> String.split(" ")
      |> Enum.map(&(String.to_integer(&1)))

    IO.read(:stdio, :line)
      |> String.trim()
      |> String.split(" ")
      |> Enum.map(&(String.to_integer(&1)))
      |> Enum.sort()
      |> maximum_toys(k, 0)
      |> IO.puts()
  end

  def maximum_toys([head | tail], budget, amount_toys) when budget >= head do
    maximum_toys(tail, budget - head, amount_toys + 1)
  end

  def maximum_toys(_toy_prices, _budget, amount_toys), do: amount_toys
end

Solution.main()
