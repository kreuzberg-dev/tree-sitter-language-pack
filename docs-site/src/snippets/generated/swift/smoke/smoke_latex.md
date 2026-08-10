```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"latex\"}")
_ = try TreeSitterLanguagePack.process(source: "\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}", config: configObj)

```
