# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = 'tree_sitter_language_pack'
  spec.version = '1.1.4'
  spec.authors = ['kreuzberg.dev']
  spec.email = ['dev@kreuzberg.dev']

  spec.summary = 'Ruby bindings for tree-sitter-language-pack'
  spec.description = '197 pre-compiled tree-sitter language parsers with Ruby bindings via Magnus native extensions.'
  spec.homepage = 'https://github.com/kreuzberg-dev/tree-sitter-language-pack'
  spec.license = 'MIT'
  spec.required_ruby_version = '>= 3.4.0'
  spec.metadata['cargo_crate_name'] = 'ts-pack-ruby'

  spec.files = Dir['lib/**/*.rb', 'ext/**/*', 'Cargo.toml', 'src/**/*.rs']
  spec.require_paths = ['lib']
  spec.extensions = ['extconf.rb']

  spec.add_dependency 'rb_sys', '~> 0.9'
end
