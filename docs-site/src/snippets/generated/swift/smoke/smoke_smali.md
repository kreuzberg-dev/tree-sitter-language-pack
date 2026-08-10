```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"smali\"}")
_ = try TreeSitterLanguagePack.process(source: ".class public LMain;\n.super Ljava/lang/Object;", config: configObj)

```
