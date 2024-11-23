## [0.3.1](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/compare/v0.3.0...v0.3.1) (2024-11-23)



# [0.3.0](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/compare/v0.2.0...v0.3.0) (2024-11-23)


### Bug Fixes

* Remove wasm from release artifacts and update build script ([1d93c2c](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/1d93c2cb292f937801a0982cb6508db6ff777bb6))


### Features

* Add package.json ([706725c](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/706725c57abbf97dfcb503ff79d97c561fcf29a2))



# [0.2.0](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/compare/v0.1.0...v0.2.0) (2024-11-22)


### Features

* add javascript client and script to attach it to wasm-pack ([0a7fced](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/0a7fced4b6d877bea20a675f968e63553789744d))
* set rust toolchain channel to 1.81 ([ed9beba](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/ed9bebad76d27b7d1826b6fc26272e239ee1249c))



# [0.1.0](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/compare/7aa1610d39d8c9198bf578b1b1c5c7030d9cbabc...v0.1.0) (2024-11-22)


### Bug Fixes

* change default SUBSTRATE_NODE_URL to use wss. ([adcb4f2](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/adcb4f2e4a3b7d1f2ee1cd7bdda9475ad7ce2a3e))
* happy clippy ([72db025](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/72db025f6e0ea1bf84698bcf76b89ecaf2b21d89))
* happy clippy ([996d400](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/996d4004f6d6e138f9d56e68b4f648c615347000))
* happy fmt ([d03f732](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/d03f7328d7946307f3420f30693a10c6489124de))
* happy fmt ([f3f78cd](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/f3f78cd455ba75f53077f7a2865657ad88bcabab))
* happy fmt && clippy ([17f0d74](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/17f0d7462292f5ebcfeb75fa1590449c50df41c2))
* make `PostprocessingLevel` `ValueEnum` ([6f4a527](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/6f4a527dbdd1d58c621d9ebf04e6a84397e52645))
* make `prover_root_url` and `auth_root_url` more natural, replace placeholder for default `substrate_root_url` ([1a0762c](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/1a0762c52def1941bdbb440fce9d5f7f0378f25a))
* make sure it runs && address reviews ([2f02091](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/2f020912acda340d4cee930f866c9a94d88499ce))
* not use manual parsing ([b183ec7](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/b183ec78ba9c38e4b5a3526cb1e46a807d14c719))
* regenerate subxt for dynamic dory changes ([dc5274e](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/dc5274e9f5aab6a0d27d2e2e118daca126f927b3))
* remove references to particular tables in `query_and_verify` ([026e95a](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/026e95a1f1c0e231fea10f1e8f9a8fb126331f68))
* switch to blitzar ([7713991](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/77139914d8438854282f7b89b04335ebf8d452f1))
* update generated javascript wasm bindings ([07959b4](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/07959b496ca51f2306130d8ac38a57d5709ab7ea))
* uppercase tables and other misc changes ([d12619d](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/d12619d66cc1c5dd8fe4e2a0e588412e4e810fae))
* use correct VerifierSetup ([7ee77e1](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/7ee77e1f89dab9e250be8e262f21931f43943f31))
* use count(*) in count_table ([0d1dec6](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/0d1dec65dbe25c8cc0db0dd34a05fd5ece273bae))
* warn on equality, error on decrease && simplify code ([c85d633](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/c85d63334f3a580f0ba1aeaa4ed99247b811e7a2))


### Features

* add .env to .gitignore ([8b717b8](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/8b717b87c385da22f4a69b0b4d0e46cce55e4075))
* add `PostprocessingLevel` ([c27df89](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/c27df89728de6f75af8a54c3df731e4519038dd7))
* add `SxTClient` ([bb8644d](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/bb8644d117700ebb0c5b0aaf6181af083377e8ba))
* add and warn about postprocessing ([c7d5054](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/c7d505439dedc1cc8ebd98bf4f7c6312b9bc3df7))
* add bearer token generation ([6de29a7](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/6de29a7aacf98e331d903ff475c9bf8f26b83037))
* add CLI args ([dea746d](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/dea746d8a669800267f72244388939c23a82ee24))
* add code to retrieve sxt access token ([906129f](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/906129fd4149dd87a6ca53feae15c04850706d43))
* add commitment request and decoding ([1dcfbd5](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/1dcfbd5b93fad01807d364aba46f45b87d1e14e1))
* add crate with wasm bindings for stateless sdk logic ([6d72f01](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/6d72f0121d147adfd48d16526ed0bc197cff7662))
* add FunctionsConsumerAbi.json ([0e77823](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/0e7782377afd4bad08a087c45a84e5956af3a8ac))
* add generated file ([65332be](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/65332be8188933e48d096664e4a1b0af7fae93e3))
* add I/O and get the code to run ([42560ff](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/42560ff89c45eadc1edbd0727d1f1af4ac258bfa))
* add IO ([551d01c](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/551d01cd19347151369bc9bad3ea0c7fe710cc51))
* add json files to .gitignore ([cdc7956](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/cdc795654d4983433cec585467f39cfc2414a973))
* add minimal example ([a3d4a81](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/a3d4a81ae7ed3d9c30d49854451adc353de6f707))
* add prover query and verification ([b216b7e](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/b216b7e2d7c34a06d770124149c9f54771bb3f1a))
* add standard Rust .gitignore && remove PoSQL repo-specific stuff from PR template ([cc880ea](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/cc880ea31ccdb5856972247a14a285ae4a9767fa))
* bump PoSQL to 0.33.5 && switch to loading of verifier setup from file ([fe1cd53](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/fe1cd530eae93b137d7ddbeecfabcd540288fd20))
* copy chainlink example code ([d6479bd](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/d6479bdf2d4404ab0bd947874404e715a43a3961))
* Create README ([376c965](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/376c96521aa98a4fa8e16d0d1d0e7721fa09bb55))
* derive common traits for SdkArgs ([af2c9da](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/af2c9da0b5f841d4e690eb825192f2df1b5414dd))
* fix errors and get the code to compile ([957fe22](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/957fe22f83abf1d9214f443afe5e0bd968d3e5fd))
* fix table range && bump proofs to 0.33.15 ([32946c5](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/32946c529fac8c9d7347aa8388365f4ec5a1fcec))
* make repo a cargo workspace with an sdk member ([b4e8e0c](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/b4e8e0cf9af7df4cd02a1f66aaff60496a27d6e7))
* move most functionality to a lib.rs function ([cff1e5f](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/cff1e5f114dd51354143bc851cf5720c1bc35e1a))
* prepend wasm codegen to javascript job ([0b49b46](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/0b49b46f3a4504ebba51631621f658b21c12f5f2))
* remove hardcoded values from example ([5d18dc9](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/5d18dc9f16b2c35e5cc6b70c135ca5f03dd06a72))
* switch to an external prover ([7ab8eb6](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/7ab8eb6a93f38562816b3a0f342b6c8456f2b546))
* switch to DynamicDory in substrate module ([e21503b](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/e21503b76022072f678c470a1c1b0862bb319e15))
* switch to REST endpoint ([c0c1eed](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/c0c1eedd46c82d681184e9f4361e1fd6bbdca89d))
* switch to substrate ([b4c9785](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/b4c9785e50e4818b3387f9afe273242db150dc6d))
* update `sxt_chain_runtime.rs` ([db69e39](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/db69e3981f67fc6db353b7b9586cf52c2b419a3f))
* Update Code Owners ([7aa1610](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/7aa1610d39d8c9198bf578b1b1c5c7030d9cbabc))
* update codegen ([99a186f](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/99a186f873d797a5c6f3fe418b7baced1b571486))
* Update License ([d77235f](https://github.com/spaceandtimelabs/sxt-proof-of-sql-sdk/commit/d77235fd14a0dcffa17bc7c2478ec95ffff2bd41))



