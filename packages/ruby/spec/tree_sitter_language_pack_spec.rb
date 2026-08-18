# frozen_string_literal: true

require_relative "../lib/tree_sitter_language_pack"

RSpec.describe TreeSitterLanguagePack do
  # Calls the generated `language_count` module function end-to-end. The
  # `require_relative` above loads the gem, whose `native.rb` dlopens the compiled
  # extension and raises LoadError when it is missing, so this example crosses the real
  # Magnus boundary: it fails on an unbuilt extension, a link error, or a removed or
  # renamed export. It does not assert *what* the value should be -- only that the
  # binding returns a value of the mapped Ruby type. Create-only scaffold seed: alef never
  # regenerates over this file, so replace it with a real suite. ~keep
  it "calls the generated `language_count` module function" do
    expect(described_class.language_count).to be_a(Integer)
  end
end
