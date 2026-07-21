#!/usr/bin/env ruby
# frozen_string_literal: true

require "fileutils"
require "rbconfig"
require "rubygems"

gem_dir = File.expand_path("../../../packages/ruby", __dir__)
Dir.chdir(gem_dir)

ruby_abi = RbConfig::CONFIG.fetch("ruby_version")
bundler_version = File
  .read("Gemfile.lock")
  .match(/\nBUNDLED WITH\n\s+(\S+)\n/)
  &.[](1)
abort("ERROR: Could not determine Bundler version from Gemfile.lock") unless bundler_version

unless Gem::Specification.find_all_by_name("bundler", bundler_version).any?
  abort(
    "ERROR: Bundler #{bundler_version} is required by Gemfile.lock but is not installed. " \
      "Run `gem install bundler -v #{bundler_version} --no-document` first."
  )
end

bundle = ["bundle", "_#{bundler_version}_"]
bundle_path = File.join("vendor", "bundle", ruby_abi)
bundle_env = {"BUNDLE_PATH" => bundle_path}
staged_root = File.join("lib", "ts_pack_core_rb")
preserved_staged_root = File.join("tmp", "ruby-native-abi-stage")

system(bundle_env, *bundle, "check") ||
  system(bundle_env, *bundle, "install", exception: true)

FileUtils.rm_rf(preserved_staged_root)
if Dir.exist?(staged_root)
  FileUtils.mkdir_p(File.dirname(preserved_staged_root))
  FileUtils.mv(staged_root, preserved_staged_root)
end

Dir.glob("lib/ts_pack_core_rb.{so,bundle,dylib}").each { |path| FileUtils.rm_f(path) }

begin
  system(bundle_env, *bundle, "exec", "rake", "compile", exception: true)
ensure
  if Dir.exist?(preserved_staged_root) && !Dir.exist?(staged_root)
    FileUtils.mkdir_p(File.dirname(staged_root))
    FileUtils.mv(preserved_staged_root, staged_root)
  end
end

native_extension = Dir.glob("lib/ts_pack_core_rb.{so,bundle,dylib}").first
abort("ERROR: No compiled native extension found for Ruby ABI #{ruby_abi}") unless native_extension

destination_dir = File.join("lib", "ts_pack_core_rb", ruby_abi)
FileUtils.mkdir_p(destination_dir)
destination = File.join(destination_dir, File.basename(native_extension))
FileUtils.cp(native_extension, destination)

puts("Staged Ruby ABI #{ruby_abi} native extension: #{destination}")
