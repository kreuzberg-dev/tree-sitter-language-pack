---
id: fixture_swift_smoke_nix
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"nix\"}")
_ = try TreeSitterLanguagePack.process(source: "{ pkgs ? import <nixpkgs> {} }: pkgs.hello", config: configObj)

```
