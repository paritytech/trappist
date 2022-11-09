[`parachains-integration-tests`](https://github.com/paritytech/parachains-integration-tests) is a tool designed to test interactions between Substrate blockchains.

Trappist uses it to ensure the correctness of some of its features.

# Setup

Install `parachains-integration-tests` into your system:
```
$ yarn global add @parity/parachains-integration-tests
```

# Usage

Please refer to the [project's `README.md`](https://github.com/paritytech/parachains-integration-tests#how-to-use) for an extensive description of how to write YAML test files and how to execute tests.

For example, to use zombienet and perform a reserve transfer that tests the functionality of `asset-registry` pallet:
```
$ parachains-integration-tests -m zombienet-test -c xcm-playground.toml -t integration-tests/asset-registry/0_reserve_transfer.yml
```
