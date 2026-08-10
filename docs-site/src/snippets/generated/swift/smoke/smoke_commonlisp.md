```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"commonlisp\"}")
_ = try TreeSitterLanguagePack.process(source: "(defun hello () (print \"hello\"))", config: configObj)

```
