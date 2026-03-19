defmodule SmokeTest do
  use ExUnit.Case

  @fixtures_dir Path.join([__DIR__, "..", "..", "fixtures"])

  defp load_fixtures(name) do
    @fixtures_dir
    |> Path.join(name)
    |> File.read!()
    |> Jason.decode!()
  end

  describe "basic fixtures" do
    test "language_count is positive" do
      count = TreeSitterLanguagePack.language_count()
      assert count >= 100, "language_count #{count} < expected min 100"
    end

    test "has_language for known languages" do
      assert TreeSitterLanguagePack.has_language("python") == true
      assert TreeSitterLanguagePack.has_language("javascript") == true
      assert TreeSitterLanguagePack.has_language("rust") == true
      assert TreeSitterLanguagePack.has_language("go") == true
    end

    test "has_language returns false for nonexistent" do
      assert TreeSitterLanguagePack.has_language("nonexistent_xyz") == false
    end

    test "available_languages contains expected languages" do
      langs = TreeSitterLanguagePack.available_languages()
      assert is_list(langs)
      assert "python" in langs
      assert "javascript" in langs
      assert "rust" in langs
      assert "go" in langs
    end
  end

  describe "parse validation" do
    test "parses Python code" do
      {:ok, tree} = TreeSitterLanguagePack.parse_string("python", "def hello(): pass\n")
      {:ok, node_type} = TreeSitterLanguagePack.tree_root_node_type(tree)
      assert node_type == "module"

      {:ok, child_count} = TreeSitterLanguagePack.tree_root_child_count(tree)
      assert child_count >= 1

      {:ok, has_errors} = TreeSitterLanguagePack.tree_has_error_nodes(tree)
      assert has_errors == false
    end

    test "errors on invalid language" do
      assert {:error, _reason} = TreeSitterLanguagePack.parse_string("nonexistent_xyz_123", "code")
    end
  end

  describe "process tests" do
    setup do
      {:ok, fixtures} = File.read(Path.join(@fixtures_dir, "process.json"))
      fixtures_data = Jason.decode!(fixtures)
      {:ok, fixtures: fixtures_data}
    end

    test "python_function", %{fixtures: fixtures} do
      fixture = Enum.find(fixtures, fn f -> f["name"] == "python_function" end)
      assert fixture != nil

      config = Jason.encode!(fixture["config"])
      {:ok, result_json} = TreeSitterLanguagePack.process(fixture["source"], config)
      result = Jason.decode!(result_json)

      assert result["language"] == "python"
      assert length(result["structure"]) >= fixture["expected"]["structure_min"]
      assert result["metrics"]["error_count"] == 0
    end

    test "javascript_function", %{fixtures: fixtures} do
      fixture = Enum.find(fixtures, fn f -> f["name"] == "javascript_function" end)
      assert fixture != nil

      config = Jason.encode!(fixture["config"])
      {:ok, result_json} = TreeSitterLanguagePack.process(fixture["source"], config)
      result = Jason.decode!(result_json)

      assert result["language"] == "javascript"
      assert length(result["structure"]) >= fixture["expected"]["structure_min"]
    end

    test "rust_function", %{fixtures: fixtures} do
      fixture = Enum.find(fixtures, fn f -> f["name"] == "rust_function" end)
      assert fixture != nil

      config = Jason.encode!(fixture["config"])
      {:ok, result_json} = TreeSitterLanguagePack.process(fixture["source"], config)
      result = Jason.decode!(result_json)

      assert result["language"] == "rust"
      assert length(result["structure"]) >= fixture["expected"]["structure_min"]
    end

    test "python_with_imports", %{fixtures: fixtures} do
      fixture = Enum.find(fixtures, fn f -> f["name"] == "python_with_imports" end)
      assert fixture != nil

      config = Jason.encode!(fixture["config"])
      {:ok, result_json} = TreeSitterLanguagePack.process(fixture["source"], config)
      result = Jason.decode!(result_json)

      assert result["language"] == "python"
      assert length(result["imports"]) >= fixture["expected"]["imports_min"]
      assert length(result["structure"]) >= fixture["expected"]["structure_min"]
    end
  end

  describe "chunking tests" do
    setup do
      {:ok, fixtures} = File.read(Path.join(@fixtures_dir, "chunking.json"))
      fixtures_data = Jason.decode!(fixtures)
      {:ok, fixtures: fixtures_data}
    end

    test "python_chunking", %{fixtures: fixtures} do
      fixture = Enum.find(fixtures, fn f -> f["name"] == "python_chunking" end)
      assert fixture != nil

      config = Jason.encode!(fixture["config"])
      {:ok, result_json} = TreeSitterLanguagePack.process(fixture["source"], config)
      result = Jason.decode!(result_json)

      assert length(result["chunks"]) >= fixture["expected"]["chunks_min"]
    end

    test "javascript_chunking", %{fixtures: fixtures} do
      fixture = Enum.find(fixtures, fn f -> f["name"] == "javascript_chunking" end)
      assert fixture != nil

      config = Jason.encode!(fixture["config"])
      {:ok, result_json} = TreeSitterLanguagePack.process(fixture["source"], config)
      result = Jason.decode!(result_json)

      assert length(result["chunks"]) >= fixture["expected"]["chunks_min"]
    end
  end

  describe "download API" do
    test "downloaded_languages returns list" do
      {:ok, langs} = TreeSitterLanguagePack.downloaded_languages()
      assert is_list(langs)
      assert length(langs) > 0
    end

    test "cache_dir returns string" do
      {:ok, cache_dir} = TreeSitterLanguagePack.cache_dir()
      assert is_binary(cache_dir)
      assert byte_size(cache_dir) > 0
    end

    test "manifest_languages returns 170+ items" do
      {:ok, langs} = TreeSitterLanguagePack.manifest_languages()
      assert is_list(langs)
      assert length(langs) >= 170
    end

    test "process with invalid language errors" do
      config = Jason.encode!(%{"language" => "nonexistent_xyz"})
      assert {:error, _reason} = TreeSitterLanguagePack.process("code", config)
    end
  end

  describe "error handling" do
    test "has_language returns false for nonexistent" do
      assert TreeSitterLanguagePack.has_language("nonexistent_lang_xyz_123") == false
    end
  end

  describe "setup" do
    test "initializes with multiple languages" do
      config = Jason.encode!(%{"languages" => ["python", "javascript", "rust", "go", "ruby", "java", "c", "cpp"]})
      assert :ok = TreeSitterLanguagePack.init(config)
    end
  end
end
