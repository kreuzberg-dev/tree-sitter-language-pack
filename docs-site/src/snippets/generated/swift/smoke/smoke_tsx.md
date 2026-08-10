```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"tsx\"}")
_ = try TreeSitterLanguagePack.process(source: "const App = () => <div />;", config: configObj)

```
