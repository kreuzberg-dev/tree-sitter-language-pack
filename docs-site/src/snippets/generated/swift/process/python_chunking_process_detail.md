```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"chunk_max_size\":30,\"language\":\"python\"}")
_ = try TreeSitterLanguagePack.process(source: "def alpha():\n    pass\n\ndef beta():\n    pass\n\ndef gamma():\n    pass\n\ndef delta():\n    pass\n", config: configObj)

```
