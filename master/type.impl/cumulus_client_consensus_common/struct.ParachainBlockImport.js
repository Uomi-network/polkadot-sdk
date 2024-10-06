(function() {var type_impls = {
"cumulus_test_service":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BlockImport%3CBlock%3E-for-ParachainBlockImport%3CBlock,+BI,+BE%3E\" class=\"impl\"><a href=\"#impl-BlockImport%3CBlock%3E-for-ParachainBlockImport%3CBlock,+BI,+BE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Block, BI, BE&gt; <a class=\"trait\" href=\"sc_consensus/block_import/trait.BlockImport.html\" title=\"trait sc_consensus::block_import::BlockImport\">BlockImport</a>&lt;Block&gt; for ParachainBlockImport&lt;Block, BI, BE&gt;<div class=\"where\">where\n    Block: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">Block</a>,\n    BI: <a class=\"trait\" href=\"sc_consensus/block_import/trait.BlockImport.html\" title=\"trait sc_consensus::block_import::BlockImport\">BlockImport</a>&lt;Block&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,\n    BE: <a class=\"trait\" href=\"sc_client_api/backend/trait.Backend.html\" title=\"trait sc_client_api::backend::Backend\">Backend</a>&lt;Block&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"sc_consensus/block_import/trait.BlockImport.html#associatedtype.Error\" class=\"associatedtype\">Error</a> = &lt;BI as <a class=\"trait\" href=\"sc_consensus/block_import/trait.BlockImport.html\" title=\"trait sc_consensus::block_import::BlockImport\">BlockImport</a>&lt;Block&gt;&gt;::<a class=\"associatedtype\" href=\"sc_consensus/block_import/trait.BlockImport.html#associatedtype.Error\" title=\"type sc_consensus::block_import::BlockImport::Error\">Error</a></h4></section></summary><div class='docblock'>The error type.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.check_block\" class=\"method trait-impl\"><a href=\"#method.check_block\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sc_consensus/block_import/trait.BlockImport.html#tymethod.check_block\" class=\"fn\">check_block</a>&lt;'life0, 'async_trait&gt;(\n    &amp;'life0 self,\n    block: <a class=\"struct\" href=\"sc_consensus/block_import/struct.BlockCheckParams.html\" title=\"struct sc_consensus::block_import::BlockCheckParams\">BlockCheckParams</a>&lt;Block&gt;,\n) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/pin/struct.Pin.html\" title=\"struct core::pin::Pin\">Pin</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"enum\" href=\"sc_consensus/block_import/enum.ImportResult.html\" title=\"enum sc_consensus::block_import::ImportResult\">ImportResult</a>, &lt;ParachainBlockImport&lt;Block, BI, BE&gt; as <a class=\"trait\" href=\"sc_consensus/block_import/trait.BlockImport.html\" title=\"trait sc_consensus::block_import::BlockImport\">BlockImport</a>&lt;Block&gt;&gt;::<a class=\"associatedtype\" href=\"sc_consensus/block_import/trait.BlockImport.html#associatedtype.Error\" title=\"type sc_consensus::block_import::BlockImport::Error\">Error</a>&gt;&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'async_trait&gt;&gt;<div class=\"where\">where\n    'life0: 'async_trait,\n    ParachainBlockImport&lt;Block, BI, BE&gt;: 'async_trait,</div></h4></section></summary><div class='docblock'>Check block preconditions.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.import_block\" class=\"method trait-impl\"><a href=\"#method.import_block\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sc_consensus/block_import/trait.BlockImport.html#tymethod.import_block\" class=\"fn\">import_block</a>&lt;'life0, 'async_trait&gt;(\n    &amp;'life0 self,\n    params: <a class=\"struct\" href=\"sc_consensus/block_import/struct.BlockImportParams.html\" title=\"struct sc_consensus::block_import::BlockImportParams\">BlockImportParams</a>&lt;Block&gt;,\n) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/pin/struct.Pin.html\" title=\"struct core::pin::Pin\">Pin</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"enum\" href=\"sc_consensus/block_import/enum.ImportResult.html\" title=\"enum sc_consensus::block_import::ImportResult\">ImportResult</a>, &lt;ParachainBlockImport&lt;Block, BI, BE&gt; as <a class=\"trait\" href=\"sc_consensus/block_import/trait.BlockImport.html\" title=\"trait sc_consensus::block_import::BlockImport\">BlockImport</a>&lt;Block&gt;&gt;::<a class=\"associatedtype\" href=\"sc_consensus/block_import/trait.BlockImport.html#associatedtype.Error\" title=\"type sc_consensus::block_import::BlockImport::Error\">Error</a>&gt;&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'async_trait&gt;&gt;<div class=\"where\">where\n    'life0: 'async_trait,\n    ParachainBlockImport&lt;Block, BI, BE&gt;: 'async_trait,</div></h4></section></summary><div class='docblock'>Import a block.</div></details></div></details>","BlockImport<Block>","cumulus_test_service::ParachainBlockImport"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-ParachainBlockImport%3CBlock,+I,+BE%3E\" class=\"impl\"><a href=\"#impl-Clone-for-ParachainBlockImport%3CBlock,+I,+BE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Block, I, BE&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for ParachainBlockImport&lt;Block, I, BE&gt;<div class=\"where\">where\n    Block: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">Block</a>,\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; ParachainBlockImport&lt;Block, I, BE&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/clone.rs.html#172\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","cumulus_test_service::ParachainBlockImport"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ParachainBlockImport%3CBlock,+BI,+BE%3E\" class=\"impl\"><a href=\"#impl-ParachainBlockImport%3CBlock,+BI,+BE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Block, BI, BE&gt; ParachainBlockImport&lt;Block, BI, BE&gt;<div class=\"where\">where\n    Block: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">Block</a>,\n    BE: <a class=\"trait\" href=\"sc_client_api/backend/trait.Backend.html\" title=\"trait sc_client_api::backend::Backend\">Backend</a>&lt;Block&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">new</a>(inner: BI, backend: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;BE&gt;) -&gt; ParachainBlockImport&lt;Block, BI, BE&gt;</h4></section></summary><div class=\"docblock\"><p>Create a new instance.</p>\n<p>The number of leaves per level limit is set to <code>LevelLimit::Default</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.new_with_limit\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">new_with_limit</a>(\n    inner: BI,\n    backend: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;BE&gt;,\n    level_leaves_max: LevelLimit,\n) -&gt; ParachainBlockImport&lt;Block, BI, BE&gt;</h4></section></summary><div class=\"docblock\"><p>Create a new instance with an explicit limit to the number of leaves per level.</p>\n<p>This function alone doesn’t enforce the limit on levels for old imported blocks,\nthe limit is eventually enforced only when new blocks are imported.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.new_with_delayed_best_block\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">new_with_delayed_best_block</a>(\n    inner: BI,\n    backend: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;BE&gt;,\n) -&gt; ParachainBlockImport&lt;Block, BI, BE&gt;</h4></section></summary><div class=\"docblock\"><p>Create a new instance which delays setting the best block.</p>\n<p>The number of leaves per level limit is set to <code>LevelLimit::Default</code>.</p>\n</div></details></div></details>",0,"cumulus_test_service::ParachainBlockImport"],["<section id=\"impl-ParachainBlockImportMarker-for-ParachainBlockImport%3CB,+BI,+BE%3E\" class=\"impl\"><a href=\"#impl-ParachainBlockImportMarker-for-ParachainBlockImport%3CB,+BI,+BE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;B, BI, BE&gt; ParachainBlockImportMarker for ParachainBlockImport&lt;B, BI, BE&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">Block</a>,</div></h3></section>","ParachainBlockImportMarker","cumulus_test_service::ParachainBlockImport"]],
"parachain_template_node":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BlockImport%3CBlock%3E-for-ParachainBlockImport%3CBlock,+BI,+BE%3E\" class=\"impl\"><a href=\"#impl-BlockImport%3CBlock%3E-for-ParachainBlockImport%3CBlock,+BI,+BE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Block, BI, BE&gt; <a class=\"trait\" href=\"sc_consensus/block_import/trait.BlockImport.html\" title=\"trait sc_consensus::block_import::BlockImport\">BlockImport</a>&lt;Block&gt; for ParachainBlockImport&lt;Block, BI, BE&gt;<div class=\"where\">where\n    Block: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">Block</a>,\n    BI: <a class=\"trait\" href=\"sc_consensus/block_import/trait.BlockImport.html\" title=\"trait sc_consensus::block_import::BlockImport\">BlockImport</a>&lt;Block&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,\n    BE: <a class=\"trait\" href=\"sc_client_api/backend/trait.Backend.html\" title=\"trait sc_client_api::backend::Backend\">Backend</a>&lt;Block&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"sc_consensus/block_import/trait.BlockImport.html#associatedtype.Error\" class=\"associatedtype\">Error</a> = &lt;BI as <a class=\"trait\" href=\"sc_consensus/block_import/trait.BlockImport.html\" title=\"trait sc_consensus::block_import::BlockImport\">BlockImport</a>&lt;Block&gt;&gt;::<a class=\"associatedtype\" href=\"sc_consensus/block_import/trait.BlockImport.html#associatedtype.Error\" title=\"type sc_consensus::block_import::BlockImport::Error\">Error</a></h4></section></summary><div class='docblock'>The error type.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.check_block\" class=\"method trait-impl\"><a href=\"#method.check_block\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sc_consensus/block_import/trait.BlockImport.html#tymethod.check_block\" class=\"fn\">check_block</a>&lt;'life0, 'async_trait&gt;(\n    &amp;'life0 self,\n    block: <a class=\"struct\" href=\"sc_consensus/block_import/struct.BlockCheckParams.html\" title=\"struct sc_consensus::block_import::BlockCheckParams\">BlockCheckParams</a>&lt;Block&gt;,\n) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/pin/struct.Pin.html\" title=\"struct core::pin::Pin\">Pin</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"enum\" href=\"sc_consensus/block_import/enum.ImportResult.html\" title=\"enum sc_consensus::block_import::ImportResult\">ImportResult</a>, &lt;ParachainBlockImport&lt;Block, BI, BE&gt; as <a class=\"trait\" href=\"sc_consensus/block_import/trait.BlockImport.html\" title=\"trait sc_consensus::block_import::BlockImport\">BlockImport</a>&lt;Block&gt;&gt;::<a class=\"associatedtype\" href=\"sc_consensus/block_import/trait.BlockImport.html#associatedtype.Error\" title=\"type sc_consensus::block_import::BlockImport::Error\">Error</a>&gt;&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'async_trait&gt;&gt;<div class=\"where\">where\n    'life0: 'async_trait,\n    ParachainBlockImport&lt;Block, BI, BE&gt;: 'async_trait,</div></h4></section></summary><div class='docblock'>Check block preconditions.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.import_block\" class=\"method trait-impl\"><a href=\"#method.import_block\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"sc_consensus/block_import/trait.BlockImport.html#tymethod.import_block\" class=\"fn\">import_block</a>&lt;'life0, 'async_trait&gt;(\n    &amp;'life0 self,\n    params: <a class=\"struct\" href=\"sc_consensus/block_import/struct.BlockImportParams.html\" title=\"struct sc_consensus::block_import::BlockImportParams\">BlockImportParams</a>&lt;Block&gt;,\n) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/pin/struct.Pin.html\" title=\"struct core::pin::Pin\">Pin</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"enum\" href=\"sc_consensus/block_import/enum.ImportResult.html\" title=\"enum sc_consensus::block_import::ImportResult\">ImportResult</a>, &lt;ParachainBlockImport&lt;Block, BI, BE&gt; as <a class=\"trait\" href=\"sc_consensus/block_import/trait.BlockImport.html\" title=\"trait sc_consensus::block_import::BlockImport\">BlockImport</a>&lt;Block&gt;&gt;::<a class=\"associatedtype\" href=\"sc_consensus/block_import/trait.BlockImport.html#associatedtype.Error\" title=\"type sc_consensus::block_import::BlockImport::Error\">Error</a>&gt;&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'async_trait&gt;&gt;<div class=\"where\">where\n    'life0: 'async_trait,\n    ParachainBlockImport&lt;Block, BI, BE&gt;: 'async_trait,</div></h4></section></summary><div class='docblock'>Import a block.</div></details></div></details>","BlockImport<Block>","parachain_template_node::service::ParachainBlockImport"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-ParachainBlockImport%3CBlock,+I,+BE%3E\" class=\"impl\"><a href=\"#impl-Clone-for-ParachainBlockImport%3CBlock,+I,+BE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Block, I, BE&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for ParachainBlockImport&lt;Block, I, BE&gt;<div class=\"where\">where\n    Block: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">Block</a>,\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; ParachainBlockImport&lt;Block, I, BE&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/clone.rs.html#172\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","parachain_template_node::service::ParachainBlockImport"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ParachainBlockImport%3CBlock,+BI,+BE%3E\" class=\"impl\"><a href=\"#impl-ParachainBlockImport%3CBlock,+BI,+BE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Block, BI, BE&gt; ParachainBlockImport&lt;Block, BI, BE&gt;<div class=\"where\">where\n    Block: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">Block</a>,\n    BE: <a class=\"trait\" href=\"sc_client_api/backend/trait.Backend.html\" title=\"trait sc_client_api::backend::Backend\">Backend</a>&lt;Block&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">new</a>(inner: BI, backend: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;BE&gt;) -&gt; ParachainBlockImport&lt;Block, BI, BE&gt;</h4></section></summary><div class=\"docblock\"><p>Create a new instance.</p>\n<p>The number of leaves per level limit is set to <code>LevelLimit::Default</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.new_with_limit\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">new_with_limit</a>(\n    inner: BI,\n    backend: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;BE&gt;,\n    level_leaves_max: LevelLimit,\n) -&gt; ParachainBlockImport&lt;Block, BI, BE&gt;</h4></section></summary><div class=\"docblock\"><p>Create a new instance with an explicit limit to the number of leaves per level.</p>\n<p>This function alone doesn’t enforce the limit on levels for old imported blocks,\nthe limit is eventually enforced only when new blocks are imported.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.new_with_delayed_best_block\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">new_with_delayed_best_block</a>(\n    inner: BI,\n    backend: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;BE&gt;,\n) -&gt; ParachainBlockImport&lt;Block, BI, BE&gt;</h4></section></summary><div class=\"docblock\"><p>Create a new instance which delays setting the best block.</p>\n<p>The number of leaves per level limit is set to <code>LevelLimit::Default</code>.</p>\n</div></details></div></details>",0,"parachain_template_node::service::ParachainBlockImport"],["<section id=\"impl-ParachainBlockImportMarker-for-ParachainBlockImport%3CB,+BI,+BE%3E\" class=\"impl\"><a href=\"#impl-ParachainBlockImportMarker-for-ParachainBlockImport%3CB,+BI,+BE%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;B, BI, BE&gt; ParachainBlockImportMarker for ParachainBlockImport&lt;B, BI, BE&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">Block</a>,</div></h3></section>","ParachainBlockImportMarker","parachain_template_node::service::ParachainBlockImport"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()