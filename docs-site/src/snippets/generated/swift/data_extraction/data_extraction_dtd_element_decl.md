```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"dtd\"}")
_ = try TreeSitterLanguagePack.process(source: "<!ELEMENT server (host, port)>\n<!ELEMENT host (#PCDATA)>\n", config: configObj)

```
