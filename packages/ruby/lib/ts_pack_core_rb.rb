# frozen_string_literal: true

require "rbconfig"

extension_name = "ts_pack_core_rb"
dlext = RbConfig::CONFIG.fetch("DLEXT")
ruby_abi = RbConfig::CONFIG.fetch("ruby_version")

candidates = [
  File.expand_path("#{extension_name}/#{ruby_abi}/#{extension_name}.#{dlext}", __dir__),
  File.expand_path("#{extension_name}.#{dlext}", __dir__)
]

native_extension = candidates.find { |path| File.file?(path) }

unless native_extension
  raise(
    LoadError,
    "cannot find #{extension_name} native extension for Ruby ABI #{ruby_abi}; " \
      "searched: #{candidates.join(", ")}"
  )
end

require native_extension
