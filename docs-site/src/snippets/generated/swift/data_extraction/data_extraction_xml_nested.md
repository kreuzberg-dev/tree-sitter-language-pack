```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"xml\"}")
_ = try TreeSitterLanguagePack.process(source: "<config><host>localhost</host><port>8080</port></config>", config: configObj)

```
