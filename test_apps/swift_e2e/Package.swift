// swift-tools-version: 6.0
// The first-party dependency pin below is managed by alef (sync.text_replacements); do not edit it by hand.
// alef:hash:43fbecbaa2785e3749e4b6559f7509af39fe989b9fc8b99a6e3de7f5cd1f2c59
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    dependencies: [
        .package(url: "https://github.com/xberg-io/tree-sitter-language-pack", branch: "release/swift/1.16.1"),
    ],
    targets: [
        .testTarget(
            name: "TreeSitterLanguagePackE2ETests",
            dependencies: [.product(name: "TreeSitterLanguagePack", package: "tree-sitter-language-pack")]
        ),
    ]
)
