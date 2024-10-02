searchState.loadedDescShard("frame_election_provider_support", 0, "Primitive traits for providing election functionality.\nThe accuracy of this type.\nThe account identifier type.\nThe account identifier type.\nThe account identifier type of this solver.\nThe accuracy of this solver. This will affect the accuracy …\nAn error occurred in some arithmetic operation.\nA voter’s stake assignment among a set of targets, …\nUtility struct to group parameters for the balancing …\nThe block number type.\nThe block number type.\nSame as <code>Supports</code> but bounded by <code>B</code>.\nSame as <code>BoundedSupports</code> but parameterized by a …\nA bounded vector.\nThe data provider of the election.\nTrait that allows zero-copy read of value-references from …\nDerive <code>parity_scale_codec::Decode</code> and for struct and enum.\nSomething that can provide the data to an <code>ElectionProvider</code>.\nElect a new set of winners, bounded by <code>MaxWinners</code>.\nBase trait for types that can provide election\nFinal result of the election.\nTrait that allows zero-copy write of value-references to …\nDerive <code>parity_scale_codec::Encode</code> and …\nThe errors that might occur in this crate and …\nThe error type that is returned by the provider.\nThe list’s error type.\nThe error type of this implementation.\nA type in which performing operations on vote weights are …\nA trait for querying a single value from a type.\nan aggregator trait for a generic type of a voter/target …\nThe <code>IndexAssignment</code> type is an intermediate between the …\nA type alias for <code>IndexAssignment</code> made from <code>NposSolution</code>.\nThe data type used to build this per-thingy.\nA (almost) marker trait that signifies an election …\nThe data provided to create support map was invalid.\nThe maximal weight in all dimensions.\nMaximum number of votes per voter that this data provider …\nThe upper bound on election winners that can be returned.\nAn election provider that does nothing whatsoever.\nSomething that can compute the result to an NPoS solution.\nRe-export some type as they are used in the interface. …\nAggregator trait for a PerThing that can be multiplied by …\nA wrapper for <code>sp_npos_elections::phragmms()</code> that …\nThe type used by the list to compare nodes for ordering.\nSomething that can provide the <code>Score</code> of an account. …\nA wrapper for <code>sp_npos_elections::seq_phragmen</code> that …\nOne of the index functions returned none.\nOne of the page indices was invalid.\nThe solution type has a voter who’s number of targets is …\nWhile going from solution indices to ratio, the weight of …\nA utility trait for something to implement …\nA structure to demonstrate the election result from the …\nA target-major representation of the the election outcome.\nThe number of voters is bigger than the <code>MaxVoters</code> bound.\nA data type larger than <code>Self::Inner</code>, used to avoid …\nA type which is used in the API of this crate as a numeric …\nA voter, at the level of abstraction of this crate.\nSame as <code>Voter</code>, but parameterized by an <code>ElectionDataProvider</code>…\nConstant version of Add for <code>proof_size</code> component with u64.\nConstant version of Add for <code>ref_time</code> component with u64.\nUtility function only to be used in benchmarking …\nUtility function only to be used in benchmarking …\nReturns true if all of <code>self</code>’s constituent weights is …\nReturns true if all of <code>self</code>’s constituent weights is …\nReturns true if all of <code>self</code>’s constituent weights is …\nReturns true if all of <code>self</code>’s constituent weights is …\nReturns true if any of <code>self</code>’s constituent weights is …\nReturns true if any of <code>self</code>’s constituent weights is …\nReturns true if any of <code>self</code>’s constituent weights is …\nReturns true if any of <code>self</code>’s constituent weights is …\nReturns true if any of <code>self</code>’s constituent weights is …\nReturn a <code>BoundedSlice</code> with the content and bound of <code>Self</code>.\nIndividual assignments. for each tuple, the first elements …\nGet the bound of the type in <code>usize</code>.\nSame as <code>Vec::resize</code>, but if <code>size</code> is more than <code>Self::bound</code>, …\nTypes and helpers to define and handle election bounds.\nTry to increase <code>self</code> by <code>amount</code> via checked addition.\nChecked <code>Weight</code> addition. Computes <code>self + rhs</code>, returning …\nChecked <code>Weight</code> scalar division. Computes …\nCalculates how many <code>other</code> fit into <code>self</code>.\nChecked <code>Weight</code> scalar multiplication. Computes …\nTry to reduce <code>self</code> by <code>amount</code> via checked subtraction.\nChecked <code>Weight</code> subtraction. Computes <code>self - rhs</code>, returning …\nClear all voters and targets.\nExactly the same semantics as <code>Vec::clear</code>.\nReturn true if the list already contains <code>id</code>.\nThe current count of ids in the list.\nTypes that are used by the data provider trait.\nAttempt to deserialise the value from input.\nAttempt to deserialize the value from input into a …\nConsume self and return the number of parts per thing.\nThe number of targets to elect.\nchecked call to <code>Self::DataProvider::desired_targets()</code> …\nThe distribution of the voter’s stake among winning …\nThe distribution of the voter’s stake.\nThe distribution of the voter’s stake among winning …\nConstant version of Div with u64.\nExact same semantics as <code>Vec::drain</code>.\nPerforms the election. This should be implemented as a …\nAll possible targets for the election, i.e. the targets …\nAll the voters that participate in the election, thus “…\nConvert self to an owned vector.\nConvert self to a slice and append it to the destination.\nReturns the fixed encoded size of the type.\nCalculates the encoded size.\nForces the insertion of <code>element</code> into <code>self</code> retaining all …\nForces the insertion of <code>element</code> into <code>self</code> retaining all …\nForces the insertion of <code>s</code> into <code>self</code> truncating first if …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstruct <code>Weight</code> from the same weight for all parts.\nConverts a fraction into <code>Self</code>.\nSame as <code>Self::from_float</code>.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nBuild this type from a number of parts per thing.\nConstruct <code>Weight</code> from weight parts, namely reference time …\nBuild this type from a percent. Equivalent to …\nApproximate the fraction <code>p/q</code> into a per-thing fraction. …\nSame as <code>Self::from_rational</code>.\nApproximate the fraction <code>p/q</code> into a per-thing fraction.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nRe-export the solution generation macro. Generates a …\nReturn the current value.\nExactly the same semantics as <code>slice::get_mut</code>.\nGet the score of <code>id</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConsume self, and return the inner <code>Vec</code>. Henceforth, the …\nConvert from a ratio assignment into one with absolute …\nReturns true of this collection is full.\nReturn <code>true</code> if this is one.\nReturn <code>true</code> if this is nothing.\nAn iterator over the list, which can have <code>take</code> called on …\nReturns an iterator over the list, starting right after …\nExactly the same semantics as <code>slice::iter_mut</code>.\nExactly the same semantics as <code>slice::last_mut</code>.\nReturn the part left when <code>self</code> is saturating-subtracted …\nReturn the next lower value to <code>self</code> or <code>self</code> if it is …\nGet the aggressive max of <code>self</code> and <code>other</code> weight.\nGet the conservative min of <code>self</code> and <code>other</code> weight.\nConstant version of Mul with u64.\nMultiplication that always rounds the result up to a whole …\nMultiplication that always rounds down to a whole number. …\nCreate <code>Self</code> with no items.\nProvide a best effort prediction about when the next …\nSame as <code>on_update</code>, but incorporate some decreased score.\nSame as <code>on_update</code>, but incorporate some increased score.\nHook for inserting a new id.\nHook for removing am id from the list.\nHook for updating a single id.\nAn implementation of <code>ElectionProvider</code> that uses an …\nEquivalent to <code>Self::from_parts(Self::ACCURACY)</code>.\nIndicate if this election provider is currently ongoing an …\nReturn the next higher value to <code>self</code> or <code>self</code> if it is …\nExactly the same semantics as <code>Vec::pop</code>.\nReturn the storage size part of the weight.\nReturn a mutable reference to the storage size part of the …\nUtility function only to be used in benchmarking …\nReturn the reference time part of the weight.\nReturn a mutable reference to the reference time part of …\nExactly the same semantics as <code>Vec::remove</code>.\nExactly the same semantics as <code>Vec::retain</code>.\nEnable/disable the given code depending on …\nEnable/disable the given code depending on …\nIncrement <code>Weight</code> by <code>amount</code> via saturating addition.\nSaturating <code>Weight</code> addition. Computes <code>self + rhs</code>, …\nSaturating <code>Weight</code> scalar division. Computes …\nSaturating <code>Weight</code> scalar multiplication. Computes …\nSaturating <code>Weight</code> scalar exponentiation. Computes …\nSaturating multiplication by the reciprocal of <code>self</code>.\tThe …\nSaturating multiplication by the reciprocal of <code>self</code>.\tThe …\nSaturating multiplication by the reciprocal of <code>self</code>.\tThe …\nReduce <code>Weight</code> by <code>amount</code> via saturating subtraction.\nSaturating <code>Weight</code> subtraction. Computes <code>self - rhs</code>, …\nGet the current <code>Score</code> of <code>who</code>.\nIf <code>who</code> changes by the returned amount they are guaranteed …\nSet the storage size part of the weight.\nSet the reference time part of the weight.\nFor tests, benchmarks and fuzzing, set the <code>score</code>.\nIf possible give a hint of expected size of the encoding.\nAttempt to skip the encoded value from input.\nMove the position of an item from one location to another …\nSolve an NPoS solution with the given <code>voters</code>, <code>targets</code>, and …\nExactly the same semantics as <code>slice::sort</code>.\nExactly the same semantics as <code>slice::sort_by</code>.\nExactly the same semantics as <code>slice::sort_by_key</code>.\nReturn the product of multiplication of this value by …\nConstant version of Sub for <code>proof_size</code> component with u64.\nConstant version of Sub for <code>ref_time</code> component with u64.\nExactly the same semantics as <code>slice::swap_remove</code>.\nTotal support.\nTraits for the election operations.\nExactly the same semantics as <code>Vec::truncate</code>.\nConsume and truncate the vector <code>v</code> in order to create a new …\nTry to add some <code>other</code> weight while upholding the <code>limit</code>.\nExactly the same semantics as <code>Vec::append</code>, but returns an …\nExactly the same semantics as <code>Vec::extend</code>, but returns an …\nExactly the same semantics as <code>Vec::insert</code>, but returns an …\nReturn the next lower value to <code>self</code> or an error with the …\nConsumes self and mutates self via the given <code>mutate</code> …\nTry and normalize this assignment.\nReturn the next higher value to <code>self</code> or an error with the …\nExactly the same semantics as <code>Vec::push</code>, but returns an <code>Err</code>…\nExactly the same semantics as [<code>Vec::rotate_left</code>], but …\nExactly the same semantics as [<code>Vec::rotate_right</code>], but …\nCheck internal state of the list. Only meant for debugging.\nRemove all items from the list.\nRegenerate this list from scratch. Returns the count of …\nConvert self to a slice and then invoke the given closure …\nSupport from voters.\nMeasure the weight used in the calculation of the solver.\nAutogenerated weights for …\nIndex of the voter among the voters list.\nVoter’s identifier.\nIndex of the voter among the voters list.\nJust winners zipped with their approval stake. Note that …\nPre-allocate <code>capacity</code> items in self.\nAllocate self with the maximum possible capacity.\nEquivalent to <code>Self::from_parts(0)</code>.\nReturn a <code>Weight</code> where all fields are zero.\nCount type for data provider bounds.\nData bounds for election data.\nThe voter and target bounds of an election.\nUtility builder for <code>ElectionBounds</code>.\nSize type for data provider bounds.\nReturns an instance of <code>ElectionBounds</code> from the current …\nReturns true if <code>given_count</code> exhausts <code>self.count</code>.\nReturns an error if the provided <code>count</code> and <code>size</code> do not fit …\nReturns an error if the provided <code>count</code> and <code>size</code> do not fit …\nReturns true if <code>given_size</code> or <code>given_count</code> exhausts …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns an instance of <code>Self</code> that is constructed by capping …\nReturns true if <code>given_size</code> exhausts <code>self.size</code>.\nSet the targets bounds.\nSets the targets count bounds.\nCaps the number of the target bounds in self to <code>voters</code> …\nSets the targets size bounds.\nSet the voters bounds.\nSets the voters count bounds.\nCaps the number of the voters bounds in self to <code>voters</code> …\nSets the voters size bounds.\nContains the error value\nContains the success value\nAlias for the result type of the election data provider.\nElections bounds, to use when calling into …\nConfiguration trait for an onchain election execution.\nErrors from the data provider.\nSomething that provides the data for election.\nErrors of the on-chain election.\nUpper bound on maximum winners from electable targets.\nAn internal error in the NPoS elections crate.\nSame as <code>BoundedSupportsOf</code> but for <code>onchain::Config</code>.\nA simple on-chain implementation of the election provider …\n<code>NposSolver</code> that should be used, an example would be …\nNeeded for weight registration.\nConfigurational error caused by <code>desired_targets</code> requested …\nWeight information for extrinsics in this pallet.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe weight/accuracy type of each vote.\nThe maximum number of votes that are allowed.\nAn opaque index-based, NPoS solution type.\nThe target type. Needs to be an index (convert to usize).\nThe voter type. Needs to be an index (convert to usize).\nGet the average edge count.\nGet the total count of edges.\nBuild self from a list of assignments.\nConvert self into a <code>Vec&lt;Assignment&lt;A, Self::Accuracy&gt;&gt;</code>\nRemove a certain voter.\nCompute the score of this solution type.\nGet the number of unique targets in the whole struct.\nGet the length of all the voters that this type is …\nWeights for pallet_election_provider_support_benchmarking …\nWeight functions needed for …\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalls <code>U::from(self)</code>.")