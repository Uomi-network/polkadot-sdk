(function() {var type_impls = {
"node_bench":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Hasher-for-Blake2Hasher\" class=\"impl\"><a class=\"src rightside\" href=\"src/sp_core/hasher.rs.html#29\">source</a><a href=\"#impl-Hasher-for-Blake2Hasher\" class=\"anchor\">§</a><h3 class=\"code-header\">impl Hasher for <a class=\"struct\" href=\"sp_core/hasher/blake2/struct.Blake2Hasher.html\" title=\"struct sp_core::hasher::blake2::Blake2Hasher\">Blake2Hasher</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Out\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Out\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Out</a> = H256</h4></section></summary><div class='docblock'>The output type of the <code>Hasher</code></div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.StdHasher\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.StdHasher\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">StdHasher</a> = Hash256StdHasher</h4></section></summary><div class='docblock'>What to use to build <code>HashMap</code>s with this <code>Hasher</code>.</div></details><details class=\"toggle\" open><summary><section id=\"associatedconstant.LENGTH\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/sp_core/hasher.rs.html#32\">source</a><a href=\"#associatedconstant.LENGTH\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a class=\"constant\">LENGTH</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.usize.html\">usize</a> = 32usize</h4></section></summary><div class='docblock'>The length in bytes of the <code>Hasher</code> output.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sp_core/hasher.rs.html#34\">source</a><a href=\"#method.hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">hash</a>(x: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>]) -&gt; &lt;<a class=\"struct\" href=\"sp_core/hasher/blake2/struct.Blake2Hasher.html\" title=\"struct sp_core::hasher::blake2::Blake2Hasher\">Blake2Hasher</a> as Hasher&gt;::Out</h4></section></summary><div class='docblock'>Compute the hash of the provided slice of bytes returning the <code>Out</code> type of the <code>Hasher</code>.</div></details></div></details>","Hasher","node_bench::simple_trie::Hasher"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Blake2Hasher\" class=\"impl\"><a class=\"src rightside\" href=\"src/sp_core/hasher.rs.html#26\">source</a><a href=\"#impl-Debug-for-Blake2Hasher\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"sp_core/hasher/blake2/struct.Blake2Hasher.html\" title=\"struct sp_core::hasher::blake2::Blake2Hasher\">Blake2Hasher</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/sp_core/hasher.rs.html#26\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","node_bench::simple_trie::Hasher"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()