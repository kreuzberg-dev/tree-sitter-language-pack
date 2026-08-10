```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"glsl\"}")
_ = try TreeSitterLanguagePack.process(source: "void main() { gl_Position = vec4(0.0); }", config: configObj)

```
