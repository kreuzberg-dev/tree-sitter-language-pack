```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"nginx\"}")
_ = try TreeSitterLanguagePack.process(source: "worker_processes 4;\nerror_log /var/log/nginx/error.log;\n", config: configObj)

```
