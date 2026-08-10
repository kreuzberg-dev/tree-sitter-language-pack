```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"python\"}")
_ = try TreeSitterLanguagePack.process(source: "import os\nimport sys\nfrom pathlib import Path\n\ndef main():\n    pass\n", config: configObj)

```
