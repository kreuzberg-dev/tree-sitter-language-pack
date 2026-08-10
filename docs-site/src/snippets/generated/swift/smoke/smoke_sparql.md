```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"sparql\"}")
_ = try TreeSitterLanguagePack.process(source: "SELECT ?s WHERE { ?s ?p ?o }", config: configObj)

```
