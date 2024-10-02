searchState.loadedDescShard("sc_chain_spec", 0, "This crate includes structs and utilities for defining …\nA base <code>Group</code> type.\nThe import operation used to import the genesis block into …\nTrait for building the genesis block.\nCommon interface of a chain specification.\nBuilder for creating <code>ChainSpec</code> instances.\nThe type of chain.\nSome custom chain type.\nThe default <code>development</code> preset used to communicate with …\nA development chain that runs mainly on one node.\nA collection of <code>ChainSpec</code> extensions.\nA <code>ChainSpec</code> extension fork definition.\nAn associated type containing fork definition.\nA configuration of a chain. Can be used to build a genesis …\nDefault genesis block builder in Substrate.\nA utility that facilitates calling the GenesisBuilder API …\nA subset of the <code>Extension</code> trait that only allows for …\nA <code>ChainSpec</code> extension.\nThe default <code>local_testnet</code> preset used to communicate with …\nA live chain.\nA local chain that runs locally on multiple nodes for …\nA type denoting empty extensions.\nNo value.\nArbitrary properties defined in chain spec as a JSON object\nSome value of type <code>T</code>.\nAdd a bootnode to the list.\nAdd a bootnode to the list.\nReturn spec as JSON.\nDump the chain specification to JSON string.\nReturn StorageBuilder for this spec.\nReturn a set of parameters for <code>Group</code> including all forks …\nA list of bootnode addresses.\nA list of bootnode addresses.\nBuilds a <code>ChainSpec</code> instance using the provided settings.\nReturns the built genesis block along with the block …\nProvides a <code>ChainSpec</code> builder.\nType of the chain.\nReturns a cloned <code>Box&lt;dyn ChainSpec&gt;</code>.\nReturns code substitutes that should be used for the on …\nCombine with another struct.\nCreate a genesis block, given the initial storage.\nReturns a reference to the defined chain spec extensions.\nReturns a reference to the defined chain spec extensions.\nReturns a mutable reference to the defined chain spec …\nReturns a mutable reference to the defined chain spec …\nGet forks definition for a subset of this extension.\nOptional network fork identifier. <code>None</code> by default.\nOptional network fork identifier.\nGet forkable extensions of specific type.\nGet forkable extensions of specific type.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nParse json content into a <code>ChainSpec</code>\nParse json file into a <code>ChainSpec</code>\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet an extension of specific type.\nGet an extension of specific type as reference to <code>Any</code>.\nGet an extension of specific type.\nGet an extension of specific type as mutable reference to …\nGet an extension of specific type with mutable access.\nReturns a json representation of the default …\nHelper function that queries an extension by type from …\nHelper function that queries an extension by type from …\nReturns a JSON blob representation of the builtin …\nCalls <code>sp_genesis_builder::GenesisBuilder::build_state</code> …\nCreates the genesis state by patching the default …\nSpec id.\nSpec id.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nA helper module providing json patching functions.\nSpec name.\nSpec name.\nCreates a new builder instance with no defaults.\nConstructs a new instance of <code>GenesisBlockBuilder</code>.\nCreates new instance using the provided code blob.\nCreate new fork definition given the base and the forks.\nAdditional loosely-typed properties of the chain.\nAdditional loosely-typed properties of the chain.\nNetwork protocol id.\nNetwork protocol id.\nReturn the state version given the genesis storage and …\nThis function sets a codeSubstitute in the chain spec.\nSet the storage that should be used by this chain spec.\nTelemetry endpoints (if any)\nTelemetry endpoints (if any)\nAttempt to convert to the base type if all parameters are …\nConvert to fork type.\nThis function updates the code in given chain spec.\nSets a list of bootnode addresses.\nSets the type of the chain.\nSets the code.\nSets chain spec extensions.\nSets an optional network fork identifier.\nSets the full runtime’s GenesisConfig JSON.\nSets the JSON patch for runtime’s GenesisConfig.\nSets the name of runtime-provided JSON patch for runtime’…\nSets the spec ID.\nSets the spec name.\nSets additional loosely-typed properties of the chain.\nSets the network protocol ID.\nSets telemetry endpoints.\nRecursively merges two JSON objects, <code>a</code> and <code>b</code>, into a …")