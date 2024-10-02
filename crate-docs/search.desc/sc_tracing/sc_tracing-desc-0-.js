searchState.loadedDescShard("sc_tracing", 0, "Instrumentation implementation for substrate.\nAn event.\nOutput to logger\nTraceHandler for sending span data to the logger\nResponsible for assigning ids to new spans, which are not …\nA span.\nRepresents a single instance of a tracing span\nRepresents a tracing event, complete with values\nA handler for tracing <code>SpanDatum</code>\nTrace handler event types.\nUsed to configure how to receive the metrics\nHolds associated values for a tracing span\nAttach additional handlers to allow handling of custom …\nUtilities for tracing block execution\nFxHashMap of <code>bool</code> values\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nProcess a <code>TraceEvent</code>.\nProcess a <code>SpanDatum</code>.\nFxHashMap of <code>i64</code> values\nid for this span\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nChecks if all individual collections are empty\nLevel of the event.\nTracing Level - ERROR, WARN, INFO, DEBUG or TRACE\nLine number in source\nSubstrate logging library.\nName of the event.\nName of this span\nTakes a <code>TracingReceiver</code> and a comma separated list of …\nReturns a new instance of Values\nAllows use of a custom TraceHandler to create a new …\nTotal duration of span while entered\nId of the parent tracing event, if any.\nid of the parent span, if any\nTime that the span was last entered\nFxHashMap of <code>String</code> values\nTarget of the event.\nTarget, typically module\nFxHashMap of <code>u64</code> values\nValues for this event.\nValues recorded to this span\nHolds a reference to the client in order to execute the …\nContains the error value\nTracing Block error\nContains the success value\nTracing Block Result type alias\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a new <code>BlockExecutor</code>\nExecute block, record all spans and events belonging to …\nContains the error value\nLogging errors.\nA pre-configured event formatter.\nA structure which, when <code>Display</code>d, will print out the …\nA builder that is used to initialize the global logger.\nContains the success value\nSpan name used for the logging prefix. See macro …\nA <code>Layer</code> that captures the prefix span (<code>PREFIX_LOG_SPAN</code>) …\nLogging Result typedef.\nAdd directives to current directives\nSets whether or not an event’s level is displayed.\nSets whether or not an event’s target is displayed.\nSets whether or not the name of the current thread is …\nDuplicate INFO, WARN and ERROR messages to stdout.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nInitialize the global logger\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a new <code>LoggerBuilder</code> which can be used to initialize …\nAdd a log prefix to the function.\nReload the logging filter with the supplied directives …\nResets the log filter back to the original state when the …\nUse the given timer for log message timestamps.\nForce enable/disable colors.\nAdd a custom profiler.\nWhether detailed log output should be enabled.\nDecides whenever the fractional timestamp with be included …\nWether or not to disable log reloading.\nSet up the profiling.")