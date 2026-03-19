require "json"
require "tree_sitter_language_pack"

RSpec.describe "TreeSitterLanguagePack smoke tests" do
  fixtures_dir = File.expand_path("../fixtures", __dir__)
  basic_fixtures = JSON.parse(File.read(File.join(fixtures_dir, "basic.json")))

  describe "basic fixtures" do
    basic_fixtures.each do |fixture|
      it fixture["name"] do
        case fixture["test"]
        when "language_count"
          count = TreeSitterLanguagePack.language_count
          expect(count).to be >= fixture["expected_min"]
        when "has_language"
          result = TreeSitterLanguagePack.has_language(fixture["language"])
          expect(result).to eq(fixture["expected"])
        when "available_languages"
          langs = TreeSitterLanguagePack.available_languages
          fixture["expected_contains"].each do |lang|
            expect(langs).to include(lang)
          end
        else
          raise "Unknown test type: #{fixture["test"]}"
        end
      end
    end
  end

  describe "parse validation" do
    it "parses Python code" do
      tree = TreeSitterLanguagePack.parse_string("python", "def hello(): pass\n")
      expect(tree.root_node_type).to eq("module")
      expect(tree.root_child_count).to be >= 1
      expect(tree.has_error_nodes).to be false
    end

    it "raises on invalid language" do
      expect {
        TreeSitterLanguagePack.parse_string("nonexistent_xyz_123", "code")
      }.to raise_error
    end
  end

  describe "download API" do
    it "downloaded_languages returns array" do
      langs = TreeSitterLanguagePack.downloaded_languages
      expect(langs).to be_a(Array)
      expect(langs.length).to be > 0
    end

    it "cache_dir returns non-empty string" do
      cache_dir = TreeSitterLanguagePack.cache_dir
      expect(cache_dir).to be_a(String)
      expect(cache_dir.length).to be > 0
    end

    it "manifest_languages returns 170+ items" do
      manifest_langs = TreeSitterLanguagePack.manifest_languages
      expect(manifest_langs).to be_a(Array)
      expect(manifest_langs.length).to be >= 170
    end

    it "process with invalid language raises" do
      expect {
        TreeSitterLanguagePack.process("code", '{"language":"nonexistent_xyz"}')
      }.to raise_error
    end
  end

  describe "error handling" do
    it "has_language for nonexistent language returns false" do
      result = TreeSitterLanguagePack.has_language("nonexistent_lang_xyz_123")
      expect(result).to be false
    end
  end
end
