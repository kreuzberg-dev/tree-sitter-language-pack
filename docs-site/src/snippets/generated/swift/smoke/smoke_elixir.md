```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"elixir\"}")
_ = try TreeSitterLanguagePack.process(source: "IO.puts(\"hello\")", config: configObj)

```
