```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"elisp\"}")
_ = try TreeSitterLanguagePack.process(source: "(defun hello () (message \"hello\"))", config: configObj)

```
