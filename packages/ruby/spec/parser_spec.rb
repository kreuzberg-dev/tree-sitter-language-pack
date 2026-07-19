# frozen_string_literal: true

require "tree_sitter_language_pack"

RSpec.describe TreeSitterLanguagePack::Parser do
  it "exposes parser methods and parses source" do
    parser = TreeSitterLanguagePack.get_parser("json")

    expect(parser).to respond_to(:set_language)
    expect(parser).to respond_to(:parse)
    expect(parser).to respond_to(:parse_bytes)
    expect(parser).to respond_to(:reset)

    tree = parser.parse('{"name":"tslp"}')

    expect(tree).not_to be_nil
    expect(tree.root_node.kind).not_to be_empty
  end
end
