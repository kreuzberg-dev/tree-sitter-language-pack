---
id: fixture_swift_smoke_vue
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"vue\"}")
_ = try TreeSitterLanguagePack.process(source: "<template><div>hello</div></template>", config: configObj)

```
