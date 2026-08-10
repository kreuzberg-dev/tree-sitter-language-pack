```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"jinja2\"}")
_ = try TreeSitterLanguagePack.process(source: "{{ variable }}", config: configObj)

```
