initSidebarItems({"derive":[["Schema",""]],"enum":[["Level","Logging levels, used for stratifying logs, and disabling less important ones for performance reasons"],["LevelFilter","A definition of the most verbose `Level` allowed, or completely off."],["SecurityEvent",""],["Value","The value part of a logging key value pair e.g. `info!(key = value)`"]],"fn":[["flush","Flush the global `Logger`"]],"macro":[["debug","Log at the `debug` level"],["error","Log at the `error` level"],["info","Log at the `info` level"],["log","Log at the given level, it’s recommended to use a specific level macro instead"],["sample","Samples a given function at a `SampleRate`, useful for periodically emitting logs or metrics on high throughput pieces of code."],["trace","Log at the `trace` level"],["warn","Log at the `warn` level"]],"mod":[["aptos_logger","Implementation of writing logs to both local printers (e.g. stdout) and remote loggers (e.g. Logstash)"],["prelude",""],["sample","Periodic sampling for logs, metrics, and other use cases through a simple macro"],["tracing_adapter",""]],"struct":[["Event","An individual structured logging event from a log line.  Includes the"],["Filter","A logging filter to determine which logs to keep or remove based on `Directive`s"],["Key","The key part of a logging key value pair e.g. `info!(key = value)`"],["KeyValue","The logging key value pair e.g. `info!(key = value)`"],["Metadata","Associated metadata with every log to identify what kind of log and where it came from"]],"trait":[["Schema","A schema of key-value pairs."],["Visitor","A visitor for the key-value pairs in a `Schema`."]]});