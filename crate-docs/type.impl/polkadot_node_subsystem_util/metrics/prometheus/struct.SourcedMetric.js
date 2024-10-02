(function() {var type_impls = {
"polkadot_node_subsystem_util":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SourcedMetric%3CT,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#61\">source</a><a href=\"#impl-SourcedMetric%3CT,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, S&gt; <a class=\"struct\" href=\"polkadot_node_subsystem_util/metrics/prometheus/struct.SourcedMetric.html\" title=\"struct polkadot_node_subsystem_util::metrics::prometheus::SourcedMetric\">SourcedMetric</a>&lt;T, S&gt;<span class=\"where fmt-newline\">where\n    T: SourcedType,\n    S: <a class=\"trait\" href=\"polkadot_node_subsystem_util/metrics/prometheus/trait.MetricSource.html\" title=\"trait polkadot_node_subsystem_util::metrics::prometheus::MetricSource\">MetricSource</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#63\">source</a><h4 class=\"code-header\">pub fn <a href=\"polkadot_node_subsystem_util/metrics/prometheus/struct.SourcedMetric.html#tymethod.new\" class=\"fn\">new</a>(opts: &amp;<a class=\"struct\" href=\"polkadot_node_subsystem_util/metrics/prometheus/struct.Opts.html\" title=\"struct polkadot_node_subsystem_util::metrics::prometheus::Opts\">Opts</a>, source: S) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"polkadot_node_subsystem_util/metrics/prometheus/struct.SourcedMetric.html\" title=\"struct polkadot_node_subsystem_util::metrics::prometheus::SourcedMetric\">SourcedMetric</a>&lt;T, S&gt;, <a class=\"enum\" href=\"polkadot_node_subsystem_util/metrics/prometheus/enum.PrometheusError.html\" title=\"enum polkadot_node_subsystem_util::metrics::prometheus::PrometheusError\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Creates a new metric that obtains its values from the given source.</p>\n</div></details></div></details>",0,"polkadot_node_subsystem_util::metrics::prometheus::SourcedCounter","polkadot_node_subsystem_util::metrics::prometheus::SourcedGauge"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-SourcedMetric%3CT,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#46\">source</a><a href=\"#impl-Debug-for-SourcedMetric%3CT,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"polkadot_node_subsystem_util/metrics/prometheus/struct.SourcedMetric.html\" title=\"struct polkadot_node_subsystem_util::metrics::prometheus::SourcedMetric\">SourcedMetric</a>&lt;T, S&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#46\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","polkadot_node_subsystem_util::metrics::prometheus::SourcedCounter","polkadot_node_subsystem_util::metrics::prometheus::SourcedGauge"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-SourcedMetric%3CT,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#46\">source</a><a href=\"#impl-Clone-for-SourcedMetric%3CT,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"polkadot_node_subsystem_util/metrics/prometheus/struct.SourcedMetric.html\" title=\"struct polkadot_node_subsystem_util::metrics::prometheus::SourcedMetric\">SourcedMetric</a>&lt;T, S&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#46\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"polkadot_node_subsystem_util/metrics/prometheus/struct.SourcedMetric.html\" title=\"struct polkadot_node_subsystem_util::metrics::prometheus::SourcedMetric\">SourcedMetric</a>&lt;T, S&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.75.0/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","polkadot_node_subsystem_util::metrics::prometheus::SourcedCounter","polkadot_node_subsystem_util::metrics::prometheus::SourcedGauge"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Collector-for-SourcedMetric%3CT,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#69\">source</a><a href=\"#impl-Collector-for-SourcedMetric%3CT,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, S&gt; <a class=\"trait\" href=\"polkadot_node_subsystem_util/metrics/prometheus/prometheus/core/trait.Collector.html\" title=\"trait polkadot_node_subsystem_util::metrics::prometheus::prometheus::core::Collector\">Collector</a> for <a class=\"struct\" href=\"polkadot_node_subsystem_util/metrics/prometheus/struct.SourcedMetric.html\" title=\"struct polkadot_node_subsystem_util::metrics::prometheus::SourcedMetric\">SourcedMetric</a>&lt;T, S&gt;<span class=\"where fmt-newline\">where\n    T: SourcedType,\n    S: <a class=\"trait\" href=\"polkadot_node_subsystem_util/metrics/prometheus/trait.MetricSource.html\" title=\"trait polkadot_node_subsystem_util::metrics::prometheus::MetricSource\">MetricSource</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.desc\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#70\">source</a><a href=\"#method.desc\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"polkadot_node_subsystem_util/metrics/prometheus/prometheus/core/trait.Collector.html#tymethod.desc\" class=\"fn\">desc</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;&amp;<a class=\"struct\" href=\"polkadot_node_subsystem_util/metrics/prometheus/prometheus/core/struct.Desc.html\" title=\"struct polkadot_node_subsystem_util::metrics::prometheus::prometheus::core::Desc\">Desc</a>&gt;</h4></section></summary><div class='docblock'>Return descriptors for metrics.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.collect\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#74\">source</a><a href=\"#method.collect\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"polkadot_node_subsystem_util/metrics/prometheus/prometheus/core/trait.Collector.html#tymethod.collect\" class=\"fn\">collect</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"polkadot_node_subsystem_util/metrics/prometheus/prometheus/proto/struct.MetricFamily.html\" title=\"struct polkadot_node_subsystem_util::metrics::prometheus::prometheus::proto::MetricFamily\">MetricFamily</a>&gt;</h4></section></summary><div class='docblock'>Collect metrics.</div></details></div></details>","Collector","polkadot_node_subsystem_util::metrics::prometheus::SourcedCounter","polkadot_node_subsystem_util::metrics::prometheus::SourcedGauge"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()