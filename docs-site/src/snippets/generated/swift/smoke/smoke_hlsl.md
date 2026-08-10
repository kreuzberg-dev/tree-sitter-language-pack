```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"hlsl\"}")
_ = try TreeSitterLanguagePack.process(source: "float4 main() : SV_Target { return 0; }", config: configObj)

```
