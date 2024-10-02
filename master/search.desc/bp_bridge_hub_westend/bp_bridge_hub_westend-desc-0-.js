searchState.loadedDescShard("bp_bridge_hub_westend", 0, "Module with configuration which reflects BridgeHubWestend …\nAverage block interval in Cumulus-based parachains.\nAll cumulus bridge hubs assume that about 5 percent of the …\nId of account on Polkadot-like chains.\nProvides a storage key for account data.\nPublic key of account on Polkadot-like chains.\nPublic key of the chain account that may be used to verify …\nThe address format for describing accounts.\nIt’s a 20 byte representation.\nIt’s a 32 byte representation.\nName of the <code>&lt;ThisChain&gt;FinalityApi::best_finalized</code> runtime …\nIdentifier of BridgeHubWestend in the Westend relay chain.\nThe balance of an account on Polkadot-like chain.\nImporting a block with 0 Extrinsics.\nSize limit of the Cumulus-based bridge hub blocks.\nBlock number type used in Polkadot-like chains.\nWeight limit of the Cumulus-based bridge hub blocks.\nWeight limit of the Cumulus-based bridge hub blocks when …\nBridgeHubWestend parachain.\nTransaction fee that is paid at the Westend BridgeHub for …\nTransaction fee that is paid at the Westend BridgeHub for …\nThe XCM fee that is paid for executing XCM program (with …\nAPI for querying information about the finalized chain …\nNumber of extra bytes (excluding size of storage value …\nAn SECP256k1/ECDSA identity (actually, the Blake2 hash of …\nAn ECDSA/SECP256k1 signature.\nAn Ed25519 identity.\nAn Ed25519 signature.\nExecuting a NO-OP <code>System::remarks</code> Extrinsic.\nName of the <code>&lt;ThisChain&gt;FinalityApi::free_headers_interval</code> …\nName of the <code>From&lt;ThisChain&gt;InboundLaneApi::message_details</code> …\nInbound message lane API for messages sent by this chain.\nHash type used in Polkadot-like chains.\nThe type of object that can produce hashes on …\nHashing type.\nThe header type used by Polkadot-like chains.\nIt’s an account ID (pubkey).\nIt’s an account index.\nMaximal bridge hub header size.\nMaximal number of unconfirmed messages at inbound lane for …\nMaximal number of unrewarded relayer entries at inbound …\nAll cumulus bridge hubs allow normal extrinsics to fill …\nNonce of a transaction on the Polkadot-like chains.\nA fixed point representation of a number in the range [0, 1…\nIt’s some arbitrary raw bytes.\nWrapper over <code>BridgeHubWestend</code>’s <code>RuntimeCall</code> that can be …\nSignature type used by Polkadot-like chains.\nPolkadot-like block signed with a Justification.\nSigned extension that is used by all bridge hubs.\nAn Sr25519 identity.\nAn Sr25519 signature.\nName of the <code>To&lt;ThisChain&gt;OutboundLaneApi::message_details</code> …\nMaximal number of bytes, included in the signed …\nOutbound message lane API for messages that are sent to …\nUnchecked Extrinsic type.\nName of the With-BridgeHubWestend messages pallet instance …\nName of the With-BridgeHubWestend bridge-relayers pallet …\nPallet index of …\nPoints to the <code>pallet_xcm_bridge_hub</code> pallet instance for …\nReturns number and hash of the best finalized header known …\nReturns number and hash of the best finalized header known …\nFull block.\nConsume self and return the number of parts per thing.\nSee <code>PerThing::deconstruct</code>.\nA chain-specific digest of data useful for light clients …\nThe merkle root of the extrinsics.\nReturn storage key for given account data.\nReturns free headers interval, if it is configured in the …\nReturns free headers interval, if it is configured in the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nNOTE: saturate to 0 or 1 if x is beyond <code>[0, 1]</code>\nSee <code>PerThing::from_float</code>.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nBuild this type from a number of parts per thing.\nFrom an explicitly defined number of parts per maximum of …\nConverts a percent into <code>Self</code>. Equal to <code>x / 100</code>.\nConverts a percent into <code>Self</code>. Equal to <code>x / 1000</code>.\nSee <code>PerThing::from_rational</code>.\nSee <code>PerThing::from_rational</code>.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nThe function that should be called.\nReturns the value of this parameter type.\nReturns the value of this parameter type.\nReturns the value of this parameter type.\nReturns the value of this parameter type.\nReturns the value of this parameter type.\nReturns the value of this parameter type.\nReturns the value of this parameter type.\nReturns the value of this parameter type.\nInteger division with another value, rounding down.\nInteger multiplication with another value, saturating at 1.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSee <code>PerThing::is_one</code>.\nSee <code>PerThing::is_zero</code>.\nBlock justification.\nReturns dispatch weight, encoded payload size and …\nReturns dispatch weight, encoded payload size and …\nReturn details of given inbound messages.\nReturn details of given inbound messages.\nSee <code>PerThing::mul_ceil</code>.\nSee <code>PerThing::mul_floor</code>.\nThe block number.\nSee <code>PerThing::one</code>\nThe parent hash.\nA payload that is included in the transaction.\nSaturating addition. Compute <code>self + rhs</code>, saturating at the …\nSaturating division. Compute <code>self / rhs</code>, saturating at one …\nSaturating multiply. Compute <code>self * rhs</code>, saturating at the …\nSaturating exponentiation. Computes <code>self.pow(exp)</code>, …\nSee <code>PerThing::saturating_reciprocal_mul</code>.\nSee <code>PerThing::saturating_reciprocal_mul_ceil</code>.\nSee <code>PerThing::saturating_reciprocal_mul_floor</code>.\nSaturating subtraction. Compute <code>self - rhs</code>, saturating at …\nThe signature, address, number of extrinsics have come …\nSee <code>PerThing::square</code>.\nThe state trie merkle root\nSee <code>PerThing::zero</code>.")