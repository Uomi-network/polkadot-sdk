searchState.loadedDescShard("slot_range_helper", 0, "A helper macro for generating <code>SlotRange</code> enum.\nThe addition operator <code>+</code>.\nPerforms subtraction that returns <code>None</code> instead of wrapping …\nTrait that allows zero-copy read of value-references from …\nDerive <code>parity_scale_codec::Decode</code> and for struct and enum.\nTrait that allows zero-copy write of value-references to …\nDerive <code>parity_scale_codec::Encode</code> and …\nThe resulting type after applying the <code>+</code> operator.\nPerforms the <code>+</code> operation.\nSubtracts two numbers, checking for underflow. If …\nAttempt to deserialise the value from input.\nAttempt to deserialize the value from input into a …\nConvert self to an owned vector.\nConvert self to a slice and append it to the destination.\nReturns the fixed encoded size of the type.\nCalculates the encoded size.\nThis macro generates a <code>SlotRange</code> enum of arbitrary length …\nError handling with the <code>Result</code> type.\nIf possible give a hint of expected size of the encoding.\nAttempt to skip the encoded value from input.\nConvert self to a slice and then invoke the given closure …\nContains the error value\nAn iterator over the value in a <code>Ok</code> variant of a <code>Result</code>.\nAn iterator over a reference to the <code>Ok</code> variant of a <code>Result</code>.\nAn iterator over a mutable reference to the <code>Ok</code> variant of …\nContains the success value\n<code>Result</code> is a type that represents either success (<code>Ok</code>) or …\nReturns <code>res</code> if the result is <code>Ok</code>, otherwise returns the <code>Err</code> …\nCalls <code>op</code> if the result is <code>Ok</code>, otherwise returns the <code>Err</code> …\nConverts from <code>Result&lt;T, E&gt;</code> (or <code>&amp;Result&lt;T, E&gt;</code>) to …\nConverts from <code>Result&lt;T, E&gt;</code> (or <code>&amp;mut Result&lt;T, E&gt;</code>) to …\nConverts from <code>&amp;mut Result&lt;T, E&gt;</code> to <code>Result&lt;&amp;mut T, &amp;mut E&gt;</code>.\nConverts from <code>&amp;Result&lt;T, E&gt;</code> to <code>Result&lt;&amp;T, &amp;E&gt;</code>.\nMaps a <code>Result&lt;&amp;T, E&gt;</code> to a <code>Result&lt;T, E&gt;</code> by cloning the …\nMaps a <code>Result&lt;&amp;mut T, E&gt;</code> to a <code>Result&lt;T, E&gt;</code> by cloning the …\nMaps a <code>Result&lt;&amp;T, E&gt;</code> to a <code>Result&lt;T, E&gt;</code> by copying the …\nMaps a <code>Result&lt;&amp;mut T, E&gt;</code> to a <code>Result&lt;T, E&gt;</code> by copying the …\nConverts from <code>Result&lt;T, E&gt;</code> to <code>Option&lt;E&gt;</code>.\nReturns the contained <code>Ok</code> value, consuming the <code>self</code> value.\nReturns the contained <code>Err</code> value, consuming the <code>self</code> value.\nConverts from <code>Result&lt;Result&lt;T, E&gt;, E&gt;</code> to <code>Result&lt;T, E&gt;</code>\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nTakes each element in the <code>Iterator</code>: if it is an <code>Err</code>, no …\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalls a function with a reference to the contained value …\nCalls a function with a reference to the contained value …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the contained <code>Err</code> value, but never panics.\nReturns a consuming iterator over the possibly contained …\nReturns the contained <code>Ok</code> value, but never panics.\nReturns <code>true</code> if the result is <code>Err</code>.\nReturns <code>true</code> if the result is <code>Err</code> and the value inside of …\nReturns <code>true</code> if the result is <code>Ok</code>.\nReturns <code>true</code> if the result is <code>Ok</code> and the value inside of …\nReturns an iterator over the possibly contained value.\nReturns a mutable iterator over the possibly contained …\nMaps a <code>Result&lt;T, E&gt;</code> to <code>Result&lt;U, E&gt;</code> by applying a function …\nMaps a <code>Result&lt;T, E&gt;</code> to <code>Result&lt;T, F&gt;</code> by applying a function …\nReturns the provided default (if <code>Err</code>), or applies a …\nMaps a <code>Result&lt;T, E&gt;</code> to <code>U</code> by applying fallback function …\nConverts from <code>Result&lt;T, E&gt;</code> to <code>Option&lt;T&gt;</code>.\nReturns <code>res</code> if the result is <code>Err</code>, otherwise returns the <code>Ok</code> …\nCalls <code>op</code> if the result is <code>Err</code>, otherwise returns the <code>Ok</code> …\nTakes each element in the <code>Iterator</code>: if it is an <code>Err</code>, no …\nTakes each element in the <code>Iterator</code>: if it is an <code>Err</code>, no …\nTransposes a <code>Result</code> of an <code>Option</code> into an <code>Option</code> of a <code>Result</code>…\nReturns the contained <code>Ok</code> value, consuming the <code>self</code> value.\nReturns the contained <code>Err</code> value, consuming the <code>self</code> value.\nReturns the contained <code>Err</code> value, consuming the <code>self</code> value, …\nReturns the contained <code>Ok</code> value or a provided default.\nReturns the contained <code>Ok</code> value or a default\nReturns the contained <code>Ok</code> value or computes it from a …\nReturns the contained <code>Ok</code> value, consuming the <code>self</code> value, …")