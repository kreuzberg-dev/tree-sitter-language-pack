defmodule TreeSitterLanguagePackTest do
  use ExUnit.Case, async: true

  doctest TreeSitterLanguagePack

  describe "available_languages/0" do
    test "returns a non-empty list" do
      languages = TreeSitterLanguagePack.available_languages()
      assert is_list(languages)
      assert length(languages) > 0
    end

    test "includes common languages" do
      languages = TreeSitterLanguagePack.available_languages()
      assert "python" in languages
      assert "javascript" in languages
      assert "rust" in languages
    end

    test "returns strings" do
      languages = TreeSitterLanguagePack.available_languages()
      assert Enum.all?(languages, &is_binary/1)
    end
  end

  describe "has_language/1" do
    test "returns true for known languages" do
      assert TreeSitterLanguagePack.has_language("python") == true
      assert TreeSitterLanguagePack.has_language("rust") == true
    end

    test "returns false for unknown languages" do
      assert TreeSitterLanguagePack.has_language("nonexistent_language_xyz") == false
    end
  end

  describe "language_count/0" do
    test "returns a positive integer" do
      count = TreeSitterLanguagePack.language_count()
      assert is_integer(count)
      assert count > 0
    end

    test "matches the length of available_languages" do
      count = TreeSitterLanguagePack.language_count()
      languages = TreeSitterLanguagePack.available_languages()
      assert count == length(languages)
    end
  end

  describe "get_language_ptr/1" do
    test "returns a non-zero integer for a valid language" do
      ptr = TreeSitterLanguagePack.get_language_ptr("python")
      assert is_integer(ptr)
      assert ptr > 0
    end

    test "raises for an unknown language" do
      assert_raise ErlangError, ~r/language_not_found/, fn ->
        TreeSitterLanguagePack.get_language_ptr("nonexistent_language_xyz")
      end
    end
  end
end
