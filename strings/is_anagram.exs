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
