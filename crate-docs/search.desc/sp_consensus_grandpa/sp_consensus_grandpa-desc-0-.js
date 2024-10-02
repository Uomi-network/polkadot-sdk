searchState.loadedDescShard("sp_consensus_grandpa", 0, "Primitives for GRANDPA integration, suitable for WASM …\nIdentity of a Grandpa authority.\nThe index of an authority.\nA list of Grandpa authorities with associated weights.\nThe grandpa crypto scheme defined via the keypair type.\nSignature for a Grandpa authority.\nThe weight of an authority.\nThe log target to be used by client code.\nA catch up message for this chain’s block type.\nA commit message for this chain’s block type.\nA compact commit message for this chain’s block type.\nAn consensus log item for GRANDPA.\nWrapper object for GRANDPA equivocation proofs, useful for …\nProof of voter misbehavior on a given set id. …\nForce an authority set change.\nThe <code>ConsensusEngineId</code> of GRANDPA.\nAPIs for integrating the GRANDPA finality gadget into …\nA GRANDPA justification for block finality, it includes a …\nKey type for GRANDPA module.\nA GRANDPA message for a substrate chain.\nNote that the authority with given index is disabled until …\nAn opaque type used to represent the key ownership proof …\nA signal to pause the current authority set after the …\nA precommit message for this chain’s block type.\nA precommit message.\nProof of equivocation at precommit stage.\nA prevote message for this chain’s block type.\nA prevote message.\nProof of equivocation at prevote stage.\nA primary propose message for this chain’s block type.\nA primary proposal message.\nThe log target to be used by runtime code.\nA signal to resume the current authority set after the …\nThe round indicator.\nA scheduled change of authority set.\nSchedule an authority set change.\nThe monotonic identifier of a GRANDPA set of authorities.\nA signed message.\nAuthentication data for the commit.\nThe base hash. See struct docs.\nThe base number. See struct docs.\nVerifies the equivocation proof by making sure that both …\nCheck a message signature by encoding the message as a …\nCheck a message signature by encoding the message as a …\nGet current GRANDPA authority set id.\nThe number of blocks to delay.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGenerates a proof of key ownership for the given authority …\nGet the current GRANDPA authorities and weights. This …\nThe Id of the signer\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nEncode round message localized to a given round and set id.\nEncode round message localized to a given round and set id …\nThe internal message which has been signed.\nCreate a new <code>EquivocationProof</code> for the given set id and …\nThe new authorities after the change, along with their …\nReturns the authority id of the equivocator.\nReturns the authority id of the equivocator.\nPrecommits for target block or any block after it that …\nPrecommits for target block or any block after it that …\nPrecommits for target block or any block after it that …\nPrevotes for target block or any block after it that …\nReturns the round number at which the equivocation …\nReturns the round number when the equivocation happened.\nRound number.\nReturns the set id at which the equivocation occurred.\nLocalizes the message to the given set and round and signs …\nThe signature on the message.\nSubmits an unsigned extrinsic to report an equivocation. …\nThe target block’s hash.\nThe target block’s hash.\nThe target block’s hash.\nThe target block’s hash.\nThe target block’s hash.\nThe target block’s number\nThe target block’s number.\nThe target block’s number\nThe target block’s number.\nThe target block’s number.\nTry to cast the log entry as a contained signal.\nTry to cast the log entry as a contained forced signal.\nTry to cast the log entry as a contained pause signal.\nTry to cast the log entry as a contained resume signal.")