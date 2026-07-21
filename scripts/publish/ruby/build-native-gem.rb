#!/usr/bin/env ruby
# frozen_string_literal: true

require "rubygems"
require "rubygems/package"
require "fileutils"

platform = ARGV[0] or abort("Usage: #{$PROGRAM_NAME} <platform>")

## Supported platforms for pre-compiled native gems.
VALID_PLATFORMS = %w[x86_64-linux aarch64-linux arm64-darwin x86_64-darwin].freeze
unless VALID_PLATFORMS.include?(platform)
  abort("ERROR: Invalid platform '#{platform}'. Valid: #{VALID_PLATFORMS.join(", ")}")
end

gem_dir = File.expand_path("../../../packages/ruby", __dir__)
Dir.chdir(gem_dir)

staged_native_glob = "lib/ts_pack_core_rb/**/*.{so,bundle,dylib}"
native_extensions = Dir.glob(staged_native_glob)
if native_extensions.empty?
  abort(
    "ERROR: No staged native extensions found under lib/ts_pack_core_rb/. " \
      "Run scripts/publish/ruby/stage-native-abi.rb for each supported Ruby ABI first."
  )
end

puts("Found native extensions: #{native_extensions.join(", ")}")

spec = Gem::Specification.load("tree_sitter_language_pack.gemspec")
abort("ERROR: Could not load tree_sitter_language_pack.gemspec") unless spec

spec.platform = Gem::Platform.new(platform)
spec.extensions = []

native_extensions.each do |ext|
  spec.files << ext unless spec.files.include?(ext)
end

spec.files.reject! do |file|
  file.start_with?("vendor/") ||
    file.start_with?("ext/") ||
    file.match?(%r{\Alib/ts_pack_core_rb\.(?:so|bundle|dylib)\z})
end

spec.dependencies.reject! { |d| d.name == "rb_sys" }

spec.files.uniq!

puts("Building gem: #{spec.name}-#{spec.version}-#{spec.platform}")
puts("Files: #{spec.files.length} (native: #{native_extensions.length})")

gem_file = Gem::Package.build(spec)

puts("Built: #{gem_file}")
