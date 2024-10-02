(function() {var type_impls = {
"relay_utils":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-Gauge%3CP%3E\" class=\"impl\"><a href=\"#impl-Clone-for-Gauge%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"relay_utils/metrics/struct.Gauge.html\" title=\"struct relay_utils::metrics::Gauge\">GenericGauge</a>&lt;P&gt;<div class=\"where\">where\n    P: <a class=\"trait\" href=\"relay_utils/metrics/trait.Atomic.html\" title=\"trait relay_utils::metrics::Atomic\">Atomic</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"relay_utils/metrics/struct.Gauge.html\" title=\"struct relay_utils::metrics::Gauge\">GenericGauge</a>&lt;P&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/clone.rs.html#172\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","relay_utils::metrics::IntGauge"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Collector-for-Gauge%3CP%3E\" class=\"impl\"><a href=\"#impl-Collector-for-Gauge%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"relay_utils/metrics/trait.Collector.html\" title=\"trait relay_utils::metrics::Collector\">Collector</a> for <a class=\"struct\" href=\"relay_utils/metrics/struct.Gauge.html\" title=\"struct relay_utils::metrics::Gauge\">GenericGauge</a>&lt;P&gt;<div class=\"where\">where\n    P: <a class=\"trait\" href=\"relay_utils/metrics/trait.Atomic.html\" title=\"trait relay_utils::metrics::Atomic\">Atomic</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.desc\" class=\"method trait-impl\"><a href=\"#method.desc\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"relay_utils/metrics/trait.Collector.html#tymethod.desc\" class=\"fn\">desc</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;&amp;Desc&gt;</h4></section></summary><div class='docblock'>Return descriptors for metrics.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.collect\" class=\"method trait-impl\"><a href=\"#method.collect\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"relay_utils/metrics/trait.Collector.html#tymethod.collect\" class=\"fn\">collect</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;MetricFamily&gt;</h4></section></summary><div class='docblock'>Collect metrics.</div></details></div></details>","Collector","relay_utils::metrics::IntGauge"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Gauge%3CP%3E\" class=\"impl\"><a href=\"#impl-Debug-for-Gauge%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"relay_utils/metrics/struct.Gauge.html\" title=\"struct relay_utils::metrics::Gauge\">GenericGauge</a>&lt;P&gt;<div class=\"where\">where\n    P: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"relay_utils/metrics/trait.Atomic.html\" title=\"trait relay_utils::metrics::Atomic\">Atomic</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","relay_utils::metrics::IntGauge"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Gauge%3CP%3E\" class=\"impl\"><a href=\"#impl-Gauge%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"struct\" href=\"relay_utils/metrics/struct.Gauge.html\" title=\"struct relay_utils::metrics::Gauge\">GenericGauge</a>&lt;P&gt;<div class=\"where\">where\n    P: <a class=\"trait\" href=\"relay_utils/metrics/trait.Atomic.html\" title=\"trait relay_utils::metrics::Atomic\">Atomic</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><h4 class=\"code-header\">pub fn <a href=\"relay_utils/metrics/struct.Gauge.html#tymethod.new\" class=\"fn\">new</a>&lt;S1, S2&gt;(name: S1, help: S2) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"relay_utils/metrics/struct.Gauge.html\" title=\"struct relay_utils::metrics::Gauge\">GenericGauge</a>&lt;P&gt;, <a class=\"enum\" href=\"relay_utils/metrics/enum.PrometheusError.html\" title=\"enum relay_utils::metrics::PrometheusError\">Error</a>&gt;<div class=\"where\">where\n    S1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;,\n    S2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Create a <a href=\"relay_utils/metrics/struct.Gauge.html\" title=\"struct relay_utils::metrics::Gauge\"><code>GenericGauge</code></a> with the <code>name</code> and <code>help</code> arguments.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.with_opts\" class=\"method\"><h4 class=\"code-header\">pub fn <a href=\"relay_utils/metrics/struct.Gauge.html#tymethod.with_opts\" class=\"fn\">with_opts</a>(opts: <a class=\"struct\" href=\"relay_utils/metrics/struct.Opts.html\" title=\"struct relay_utils::metrics::Opts\">Opts</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"relay_utils/metrics/struct.Gauge.html\" title=\"struct relay_utils::metrics::Gauge\">GenericGauge</a>&lt;P&gt;, <a class=\"enum\" href=\"relay_utils/metrics/enum.PrometheusError.html\" title=\"enum relay_utils::metrics::PrometheusError\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Create a <a href=\"relay_utils/metrics/struct.Gauge.html\" title=\"struct relay_utils::metrics::Gauge\"><code>GenericGauge</code></a> with the <code>opts</code> options.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set\" class=\"method\"><h4 class=\"code-header\">pub fn <a href=\"relay_utils/metrics/struct.Gauge.html#tymethod.set\" class=\"fn\">set</a>(&amp;self, v: &lt;P as <a class=\"trait\" href=\"relay_utils/metrics/trait.Atomic.html\" title=\"trait relay_utils::metrics::Atomic\">Atomic</a>&gt;::<a class=\"associatedtype\" href=\"relay_utils/metrics/trait.Atomic.html#associatedtype.T\" title=\"type relay_utils::metrics::Atomic::T\">T</a>)</h4></section></summary><div class=\"docblock\"><p>Set the gauge to an arbitrary value.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.inc\" class=\"method\"><h4 class=\"code-header\">pub fn <a href=\"relay_utils/metrics/struct.Gauge.html#tymethod.inc\" class=\"fn\">inc</a>(&amp;self)</h4></section></summary><div class=\"docblock\"><p>Increase the gauge by 1.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.dec\" class=\"method\"><h4 class=\"code-header\">pub fn <a href=\"relay_utils/metrics/struct.Gauge.html#tymethod.dec\" class=\"fn\">dec</a>(&amp;self)</h4></section></summary><div class=\"docblock\"><p>Decrease the gauge by 1.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.add\" class=\"method\"><h4 class=\"code-header\">pub fn <a href=\"relay_utils/metrics/struct.Gauge.html#tymethod.add\" class=\"fn\">add</a>(&amp;self, v: &lt;P as <a class=\"trait\" href=\"relay_utils/metrics/trait.Atomic.html\" title=\"trait relay_utils::metrics::Atomic\">Atomic</a>&gt;::<a class=\"associatedtype\" href=\"relay_utils/metrics/trait.Atomic.html#associatedtype.T\" title=\"type relay_utils::metrics::Atomic::T\">T</a>)</h4></section></summary><div class=\"docblock\"><p>Add the given value to the gauge. (The value can be\nnegative, resulting in a decrement of the gauge.)</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.sub\" class=\"method\"><h4 class=\"code-header\">pub fn <a href=\"relay_utils/metrics/struct.Gauge.html#tymethod.sub\" class=\"fn\">sub</a>(&amp;self, v: &lt;P as <a class=\"trait\" href=\"relay_utils/metrics/trait.Atomic.html\" title=\"trait relay_utils::metrics::Atomic\">Atomic</a>&gt;::<a class=\"associatedtype\" href=\"relay_utils/metrics/trait.Atomic.html#associatedtype.T\" title=\"type relay_utils::metrics::Atomic::T\">T</a>)</h4></section></summary><div class=\"docblock\"><p>Subtract the given value from the gauge. (The value can be\nnegative, resulting in an increment of the gauge.)</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.get\" class=\"method\"><h4 class=\"code-header\">pub fn <a href=\"relay_utils/metrics/struct.Gauge.html#tymethod.get\" class=\"fn\">get</a>(&amp;self) -&gt; &lt;P as <a class=\"trait\" href=\"relay_utils/metrics/trait.Atomic.html\" title=\"trait relay_utils::metrics::Atomic\">Atomic</a>&gt;::<a class=\"associatedtype\" href=\"relay_utils/metrics/trait.Atomic.html#associatedtype.T\" title=\"type relay_utils::metrics::Atomic::T\">T</a></h4></section></summary><div class=\"docblock\"><p>Return the gauge value.</p>\n</div></details></div></details>",0,"relay_utils::metrics::IntGauge"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Metric-for-Gauge%3CP%3E\" class=\"impl\"><a href=\"#impl-Metric-for-Gauge%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; Metric for <a class=\"struct\" href=\"relay_utils/metrics/struct.Gauge.html\" title=\"struct relay_utils::metrics::Gauge\">GenericGauge</a>&lt;P&gt;<div class=\"where\">where\n    P: <a class=\"trait\" href=\"relay_utils/metrics/trait.Atomic.html\" title=\"trait relay_utils::metrics::Atomic\">Atomic</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.metric\" class=\"method trait-impl\"><a href=\"#method.metric\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">metric</a>(&amp;self) -&gt; Metric</h4></section></summary><div class='docblock'>Return the protocol Metric.</div></details></div></details>","Metric","relay_utils::metrics::IntGauge"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()