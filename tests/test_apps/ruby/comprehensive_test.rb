require "json"
require "tree_sitter_language_pack"

RSpec.describe "TreeSitterLanguagePack comprehensive tests" do
  fixtures_dir = File.expand_path("../fixtures", __dir__)

  describe "process tests" do
    process_fixtures = JSON.parse(File.read(File.join(fixtures_dir, "process.json")))

    process_fixtures.each do |fixture|
      it fixture["name"] do
        config_json = JSON.generate(fixture["config"])
        result_json = TreeSitterLanguagePack.process(fixture["source"], config_json)
        result = JSON.parse(result_json)
        expected = fixture["expected"]

        expect(result["language"]).to eq(expected["language"]) if expected.key?("language")
        if expected.key?("structure_min")
          expect(result["structure"].length).to be >= expected["structure_min"]
        end
        if expected.key?("imports_min")
          expect(result["imports"].length).to be >= expected["imports_min"]
        end
        if expected.key?("error_count")
          expect(result["metrics"]["error_count"]).to eq(expected["error_count"])
        end
      end
    end
  end

  describe "chunking tests" do
    chunking_fixtures = JSON.parse(File.read(File.join(fixtures_dir, "chunking.json")))

    chunking_fixtures.each do |fixture|
      it fixture["name"] do
        config_json = JSON.generate(fixture["config"])
        result_json = TreeSitterLanguagePack.process(fixture["source"], config_json)
        result = JSON.parse(result_json)
        expected = fixture["expected"]

        if expected.key?("chunks_min")
          expect(result["chunks"].length).to be >= expected["chunks_min"]
        end
      end
    end
  end

  describe "setup" do
    it "initializes with multiple languages" do
      config_str = '{"languages":["python","javascript","rust","go","ruby","java","c","cpp"]}'
      expect {
        TreeSitterLanguagePack.init(config_str)
      }.not_to raise_error
    end
  end
end
