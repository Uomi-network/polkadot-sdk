(function() {var type_impls = {
"sc_transaction_pool_api":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RangeExt%3Cusize%3E-for-R\" class=\"impl\"><a href=\"#impl-RangeExt%3Cusize%3E-for-R\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;R&gt; RangeExt&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.usize.html\">usize</a>&gt; for R<span class=\"where fmt-newline\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/range/trait.RangeBounds.html\" title=\"trait core::ops::range::RangeBounds\">RangeBounds</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.usize.html\">usize</a>&gt;,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.normalize\" class=\"method trait-impl\"><a href=\"#method.normalize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">normalize</a>(\n    self,\n    start: impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.usize.html\">usize</a>&gt;&gt;,\n    end: impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.usize.html\">usize</a>&gt;&gt;\n) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/range/struct.Range.html\" title=\"struct core::ops::range::Range\">Range</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.usize.html\">usize</a>&gt;</h4></section></summary><div class='docblock'>Normalizes a range-like type to a canonical half-open <code>Range</code>. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.intersection\" class=\"method trait-impl\"><a href=\"#method.intersection\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">intersection</a>&lt;R2&gt;(self, other: R2) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/range/struct.Range.html\" title=\"struct core::ops::range::Range\">Range</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.usize.html\">usize</a>&gt;&gt;<span class=\"where fmt-newline\">where\n    R2: RangeExt&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.usize.html\">usize</a>&gt;,</span></h4></section></summary><div class='docblock'>Finds the intersection between two range-likes. The produced <code>Range</code>\nspans only the elements common to both. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.union\" class=\"method trait-impl\"><a href=\"#method.union\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">union</a>&lt;R2&gt;(self, other: R2) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/range/struct.Range.html\" title=\"struct core::ops::range::Range\">Range</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.usize.html\">usize</a>&gt;&gt;<span class=\"where fmt-newline\">where\n    R2: RangeExt&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.usize.html\">usize</a>&gt;,</span></h4></section></summary><div class='docblock'>Finds the union between two range-likes. The produced <code>Range</code> spans all\nelements present in at least one of them. <a>Read more</a></div></details></div></details>","RangeExt<usize>","sc_transaction_pool_api::TxIndex"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()