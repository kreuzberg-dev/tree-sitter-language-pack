```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"lean\"}")
_ = try TreeSitterLanguagePack.process(source: "def main : IO Unit := pure ()", config: configObj)

```
