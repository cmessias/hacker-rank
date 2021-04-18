# https://www.codewars.com/kata/529eef7a9194e0cbc1000255

defmodule Anagram do
  def anagram?(a, b) do
    char_frequencies(a) === char_frequencies(b)
  end

  defp char_frequencies(string) do
    string
    |> String.downcase()
    |> String.graphemes()
    |> frequencies()
  end

  defp frequencies(list) do
    Enum.reduce(list, %{}, fn (elem, acc) ->
      Map.update(acc, elem, 1, &(&1 + 1))
    end)
  end
end
