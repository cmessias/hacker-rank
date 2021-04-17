# https://www.codewars.com/kata/529eef7a9194e0cbc1000255

defmodule Anagram do
  def anagram?(a, b) do
    a_freq = char_frequencies(a)
    b_freq = char_frequencies(b)

    a_freq === b_freq
  end

  defp char_frequencies(string) do
    string
    |> String.downcase()
    |> String.graphemes()
    |> frequencies()
  end

  defp frequencies(list) do
    list
    |> Enum.reduce(%{}, fn (elem, acc) ->
         Map.update(acc, elem, 1, &(&1 + 1))
       end)
  end
end
