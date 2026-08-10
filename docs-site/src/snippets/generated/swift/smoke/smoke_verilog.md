```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"verilog\"}")
_ = try TreeSitterLanguagePack.process(source: "module main; endmodule", config: configObj)

```
