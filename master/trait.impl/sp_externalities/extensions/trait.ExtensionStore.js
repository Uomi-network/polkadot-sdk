(function() {var implementors = {
"sp_state_machine":[["impl ExtensionStore for <a class=\"struct\" href=\"sp_state_machine/struct.BasicExternalities.html\" title=\"struct sp_state_machine::BasicExternalities\">BasicExternalities</a>"],["impl&lt;'a, H, B&gt; ExtensionStore for <a class=\"struct\" href=\"sp_state_machine/struct.Ext.html\" title=\"struct sp_state_machine::Ext\">Ext</a>&lt;'a, H, B&gt;<div class=\"where\">where\n    H: Hasher,\n    B: 'a + <a class=\"trait\" href=\"sp_state_machine/backend/trait.Backend.html\" title=\"trait sp_state_machine::backend::Backend\">Backend</a>&lt;H&gt;,</div>"],["impl&lt;'a, H: Hasher, B: 'a + <a class=\"trait\" href=\"sp_state_machine/backend/trait.Backend.html\" title=\"trait sp_state_machine::backend::Backend\">Backend</a>&lt;H&gt;&gt; ExtensionStore for <a class=\"struct\" href=\"sp_state_machine/struct.ReadOnlyExternalities.html\" title=\"struct sp_state_machine::ReadOnlyExternalities\">ReadOnlyExternalities</a>&lt;'a, H, B&gt;"],["impl&lt;H&gt; ExtensionStore for <a class=\"struct\" href=\"sp_state_machine/struct.TestExternalities.html\" title=\"struct sp_state_machine::TestExternalities\">TestExternalities</a>&lt;H&gt;<div class=\"where\">where\n    H: Hasher,\n    H::Out: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a> + Codec,</div>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()