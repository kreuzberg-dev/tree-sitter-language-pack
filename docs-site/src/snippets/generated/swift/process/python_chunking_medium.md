```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"chunk_max_size\":50,\"language\":\"python\"}")
_ = try TreeSitterLanguagePack.process(source: "def first():\n    x = 1\n    return x\n\ndef second():\n    y = 2\n    return y\n\ndef third():\n    z = 3\n    return z\n", config: configObj)

```
