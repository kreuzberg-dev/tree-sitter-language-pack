```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"powershell\"}")
_ = try TreeSitterLanguagePack.process(source: "Write-Host 'hello'", config: configObj)

```
