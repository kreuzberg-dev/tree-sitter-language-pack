# frozen_string_literal: true

require "tree_sitter_language_pack"

langs = TreeSitterLanguagePack.available_languages
raise "no languages available" unless langs.length.positive?

puts "Available languages: #{langs.length}"
puts "Ruby smoke test passed"
