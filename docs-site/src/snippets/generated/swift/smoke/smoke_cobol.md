```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cobol\"}")
_ = try TreeSitterLanguagePack.process(source: "       IDENTIFICATION DIVISION.\n       PROGRAM-ID. HELLO.", config: configObj)

```
