```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.packConfigFromJson("{\"cache_dir\":\"/tmp/tslp_test_cache\"}")
try TreeSitterLanguagePack.configure(config: configObj)

```
