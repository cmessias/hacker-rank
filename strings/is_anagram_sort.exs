# https://www.codewars.com/kata/529eef7a9194e0cbc1000255

defmodule Anagram do
  def anagram?(a, b) do
    sorted_string(a) == sorted_string(b)
  end

  defp sorted_string(string) do
    string
    |> String.downcase()
    |> String.graphemes()
    |> Enum.sort()
  end
end
