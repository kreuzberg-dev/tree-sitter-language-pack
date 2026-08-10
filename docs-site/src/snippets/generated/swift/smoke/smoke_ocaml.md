```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ocaml\"}")
_ = try TreeSitterLanguagePack.process(source: "let () = print_endline \"hello\"", config: configObj)

```
