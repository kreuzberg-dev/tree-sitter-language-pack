```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"qmljs\"}")
_ = try TreeSitterLanguagePack.process(source: "import QtQuick 2.0\nItem {}", config: configObj)

```
