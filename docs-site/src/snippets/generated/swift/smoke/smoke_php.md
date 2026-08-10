```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"php\"}")
_ = try TreeSitterLanguagePack.process(source: "<?php echo 'hello'; ?>", config: configObj)

```
