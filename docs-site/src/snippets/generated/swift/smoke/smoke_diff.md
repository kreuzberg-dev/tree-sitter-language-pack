```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"diff\"}")
_ = try TreeSitterLanguagePack.process(source: "--- a/file\n+++ b/file\n@@ -1 +1 @@\n-old\n+new", config: configObj)

```
