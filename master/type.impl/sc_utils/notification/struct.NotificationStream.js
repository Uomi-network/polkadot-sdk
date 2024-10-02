(function() {var type_impls = {
"sc_consensus_beefy":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-NotificationStream%3CPayload,+TK%3E\" class=\"impl\"><a href=\"#impl-Clone-for-NotificationStream%3CPayload,+TK%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Payload, TK&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for NotificationStream&lt;Payload, TK&gt;<div class=\"where\">where\n    Payload: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    TK: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + TracingKeyStr,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; NotificationStream&lt;Payload, TK&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/clone.rs.html#172\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","sc_consensus_beefy::communication::notification::BeefyBestBlockStream","sc_consensus_beefy::communication::notification::BeefyVersionedFinalityProofStream"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-NotificationStream%3CPayload,+TK%3E\" class=\"impl\"><a href=\"#impl-NotificationStream%3CPayload,+TK%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Payload, TK&gt; NotificationStream&lt;Payload, TK&gt;<div class=\"where\">where\n    TK: TracingKeyStr,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.channel\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">channel</a>() -&gt; (NotificationSender&lt;Payload&gt;, NotificationStream&lt;Payload, TK&gt;)</h4></section></summary><div class=\"docblock\"><p>Creates a new pair of receiver and sender of <code>Payload</code> notifications.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.subscribe\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">subscribe</a>(\n    &amp;self,\n    queue_size_warning: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.usize.html\">usize</a>,\n) -&gt; NotificationReceiver&lt;Payload&gt;</h4></section></summary><div class=\"docblock\"><p>Subscribe to a channel through which the generic payload can be received.</p>\n</div></details></div></details>",0,"sc_consensus_beefy::communication::notification::BeefyBestBlockStream","sc_consensus_beefy::communication::notification::BeefyVersionedFinalityProofStream"]],
"sc_consensus_grandpa":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-NotificationStream%3CPayload,+TK%3E\" class=\"impl\"><a href=\"#impl-Clone-for-NotificationStream%3CPayload,+TK%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Payload, TK&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for NotificationStream&lt;Payload, TK&gt;<div class=\"where\">where\n    Payload: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    TK: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + TracingKeyStr,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; NotificationStream&lt;Payload, TK&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/clone.rs.html#172\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","sc_consensus_grandpa::notification::GrandpaJustificationStream"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-NotificationStream%3CPayload,+TK%3E\" class=\"impl\"><a href=\"#impl-NotificationStream%3CPayload,+TK%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Payload, TK&gt; NotificationStream&lt;Payload, TK&gt;<div class=\"where\">where\n    TK: TracingKeyStr,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.channel\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">channel</a>() -&gt; (NotificationSender&lt;Payload&gt;, NotificationStream&lt;Payload, TK&gt;)</h4></section></summary><div class=\"docblock\"><p>Creates a new pair of receiver and sender of <code>Payload</code> notifications.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.subscribe\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">subscribe</a>(\n    &amp;self,\n    queue_size_warning: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.usize.html\">usize</a>,\n) -&gt; NotificationReceiver&lt;Payload&gt;</h4></section></summary><div class=\"docblock\"><p>Subscribe to a channel through which the generic payload can be received.</p>\n</div></details></div></details>",0,"sc_consensus_grandpa::notification::GrandpaJustificationStream"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()