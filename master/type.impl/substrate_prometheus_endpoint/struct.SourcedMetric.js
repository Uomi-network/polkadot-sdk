(function() {var type_impls = {
"substrate_prometheus_endpoint":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-SourcedMetric%3CT,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#46\">source</a><a href=\"#impl-Clone-for-SourcedMetric%3CT,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"substrate_prometheus_endpoint/struct.SourcedMetric.html\" title=\"struct substrate_prometheus_endpoint::SourcedMetric\">SourcedMetric</a>&lt;T, S&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#46\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"substrate_prometheus_endpoint/struct.SourcedMetric.html\" title=\"struct substrate_prometheus_endpoint::SourcedMetric\">SourcedMetric</a>&lt;T, S&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/clone.rs.html#172\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","substrate_prometheus_endpoint::sourced::SourcedCounter","substrate_prometheus_endpoint::sourced::SourcedGauge"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Collector-for-SourcedMetric%3CT,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#69-133\">source</a><a href=\"#impl-Collector-for-SourcedMetric%3CT,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: SourcedType, S: <a class=\"trait\" href=\"substrate_prometheus_endpoint/trait.MetricSource.html\" title=\"trait substrate_prometheus_endpoint::MetricSource\">MetricSource</a>&gt; Collector for <a class=\"struct\" href=\"substrate_prometheus_endpoint/struct.SourcedMetric.html\" title=\"struct substrate_prometheus_endpoint::SourcedMetric\">SourcedMetric</a>&lt;T, S&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.desc\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#70-72\">source</a><a href=\"#method.desc\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">desc</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;&amp;Desc&gt;</h4></section></summary><div class='docblock'>Return descriptors for metrics.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.collect\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#74-132\">source</a><a href=\"#method.collect\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">collect</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;MetricFamily&gt;</h4></section></summary><div class='docblock'>Collect metrics.</div></details></div></details>","Collector","substrate_prometheus_endpoint::sourced::SourcedCounter","substrate_prometheus_endpoint::sourced::SourcedGauge"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-SourcedMetric%3CT,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#46\">source</a><a href=\"#impl-Debug-for-SourcedMetric%3CT,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"substrate_prometheus_endpoint/struct.SourcedMetric.html\" title=\"struct substrate_prometheus_endpoint::SourcedMetric\">SourcedMetric</a>&lt;T, S&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#46\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","substrate_prometheus_endpoint::sourced::SourcedCounter","substrate_prometheus_endpoint::sourced::SourcedGauge"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-SourcedMetric%3CT,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#61-67\">source</a><a href=\"#impl-SourcedMetric%3CT,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: SourcedType, S: <a class=\"trait\" href=\"substrate_prometheus_endpoint/trait.MetricSource.html\" title=\"trait substrate_prometheus_endpoint::MetricSource\">MetricSource</a>&gt; <a class=\"struct\" href=\"substrate_prometheus_endpoint/struct.SourcedMetric.html\" title=\"struct substrate_prometheus_endpoint::SourcedMetric\">SourcedMetric</a>&lt;T, S&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/substrate_prometheus_endpoint/sourced.rs.html#63-66\">source</a><h4 class=\"code-header\">pub fn <a href=\"substrate_prometheus_endpoint/struct.SourcedMetric.html#tymethod.new\" class=\"fn\">new</a>(opts: &amp;<a class=\"struct\" href=\"substrate_prometheus_endpoint/struct.Opts.html\" title=\"struct substrate_prometheus_endpoint::Opts\">Opts</a>, source: S) -&gt; Result&lt;Self&gt;</h4></section></summary><div class=\"docblock\"><p>Creates a new metric that obtains its values from the given source.</p>\n</div></details></div></details>",0,"substrate_prometheus_endpoint::sourced::SourcedCounter","substrate_prometheus_endpoint::sourced::SourcedGauge"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()