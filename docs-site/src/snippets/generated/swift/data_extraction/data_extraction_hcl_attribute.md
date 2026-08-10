```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"hcl\"}")
_ = try TreeSitterLanguagePack.process(source: "region = \"us-east-1\"\ncount  = 3\n", config: configObj)

```
