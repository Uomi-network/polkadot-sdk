searchState.loadedDescShard("snowbridge_pallet_ethereum_client", 0, "Ethereum Beacon Client\nThe <code>pallet</code> module in each FRAME pallet hosts the most …\nAutogenerated weights for ethereum_beacon_client\nGeneralized Indices related to Merkle proofs …\nThe index of the block_roots field in the beacon state …\nget_generalized_index(BeaconState, ‘…\nDomainType(‘0x07000000’) …\n256 epochs in a sync committee period. Frequency of sync …\nget_generalized_index(BeaconBlockBody, ‘execution_payload…\nget_generalized_index(BeaconState, ‘finalized_checkpoint…\nSanity value to constrain the max size of a merkle branch …\nSizes related to SSZ encoding\nget_generalized_index(BeaconState, ‘next_sync_committee…\nValidators public keys are 48 bytes.\nSignatures produced by validators are 96 bytes.\nDefined in …\nThe size of the block root array in the beacon state, used …\nAn array of sync committee block votes, one bit …\nA sync committee contains 512 randomly selected validators.\nCompute epoch in which a slot is contained.\nCompute the sync committee period in which a slot is …\nDecompress packed bitvector into byte vector according to …\nSums the bit vector of sync committee participation.\nAll pallets included in the runtime as a nested tuple of …\nAll pallets included in the runtime as a nested tuple of …\nProvides an implementation of <code>PalletInfo</code> to provide …\nThe aggregated runtime call type.\nA reason for placing a freeze on funds.\nA reason for placing a hold on funds.\nAn identifier for each lock placed on funds.\nThe runtime origin type representing the origin of a call.\nA reason for slashing funds.\nAn aggregation of all <code>Task</code> enums across all pallets …\nReturns the argument unchanged.\nReturns the argument unchanged.\nConvert to runtime origin with caller being system signed …\nReturns the argument unchanged.\nConvert to runtime origin, using as filter: …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nOptionally convert the <code>DispatchError</code> into the <code>RuntimeError</code>.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nReturns the value of this parameter type.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate with system none origin and …\nCreate with system root origin and …\nCreate with system signed origin and …\nContains a variant per dispatchable extrinsic that this …\nConfiguration trait of this pallet.\nSync committee for current period\nThe <code>Error</code> enum of this pallet.\nThe <code>Event</code> enum of this pallet\nBeacon state by finalized block root\nFinalized Headers: Current position in ring buffer\nFinalized Headers: Mapping of ring buffer index to a …\nMinimum gap between finalized headers for an update to be …\nLatest imported checkpoint root\nThe gap between the finalized headers is larger than the …\nThe given update is not in the expected period, or the …\nAttested header is older than latest finalized header.\nLatest imported finalized block root\nThe last period where the next sync committee was updated …\nType alias to <code>Pallet</code>, to be used by <code>construct_runtime</code>.\nSync committee for next period\nThe current operating mode of the pallet.\nSet OperatingMode\nThe <code>Pallet</code> struct, the main type that implements traits …\nStorage type is <code>StorageValue</code> with value type <code>H256</code>.\nAuto-generated docs-only module listing all defined …\nAn auto-generated getter for <code>FinalizedBeaconState</code>.\nReturns a vector of public keys that participated in the …\nUsed for pallet initialization and light client resetting. …\nUsed for pallet initialization and light client resetting. …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nAn auto-generated getter for <code>InitialCheckpointRoot</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nAn auto-generated getter for <code>LatestFinalizedBlockRoot</code>.\nCreate a call with the variant <code>force_checkpoint</code>.\nCreate a call with the variant <code>set_operating_mode</code>.\nCreate a call with the variant <code>submit</code>.\nAn auto-generated getter for <code>OperatingMode</code>.\nHalt or resume all pallet operations. May only be called …\nHalt or resume all pallet operations. May only be called …\nCalculates signing root for BeaconHeader. The signing root …\nAuto-generated docs-only module listing all (public and …\nStores a compacted (slot and block roots root (hash of the …\nSubmits a new finalized beacon header update. The update …\nSubmits a new finalized beacon header update. The update …\nAn auto-generated getter for <code>ValidatorsRoot</code>.\nVerify a message by verifying the existence of the …\nVerifies that the receipt encoded in <code>proof.data</code> is …\nUsed for pallet initialization and light client resetting. …\nHalt or resume all pallet operations. May only be called …\nSubmits a new finalized beacon header update. The update …\nSync committee for current period\nBeacon state by finalized block root\nFinalized Headers: Current position in ring buffer\nFinalized Headers: Mapping of ring buffer index to a …\nLatest imported checkpoint root\nLatest imported finalized block root\nThe last period where the next sync committee was updated …\nSync committee for next period\nThe current operating mode of the pallet.\nWarning: Doc-Only\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nFinalizedState ring buffer implementation\nProof that <code>header</code> is an ancestor of a finalized header\nA recent header attesting to the finalized header, using …\nThe merkle path to prove the <code>block_roots_root</code> value.\nThe finalized_header’s <code>block_roots</code> root in the beacon …\nMerkle proof that execution payload is contained within …\nThe execution header to be verified\nThe merkle proof testifying to the finalized header, using …\nRoot of a finalized block that has already been imported …\nThe latest finalized header.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nHeader for the beacon block containing the execution …\nMerkle proof that <code>header</code> is an ancestor of <code>finalized_header</code>\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe next sync committee for the next sync committee …\nThe slot at which the sync aggregate can be found, …\nThe signing data that the sync committee produced for this …\nWeight functions needed for ethereum_beacon_client.")