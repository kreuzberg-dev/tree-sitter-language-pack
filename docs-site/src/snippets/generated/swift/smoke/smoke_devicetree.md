```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"devicetree\"}")
_ = try TreeSitterLanguagePack.process(source: "/dts-v1/;\n/ { };", config: configObj)

```
