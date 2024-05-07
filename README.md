# Esplora-CLI 

A rust CLI for [esplora-client](https://github.com/bitcoindevkit/rust-esplora-client).

```
Bitcoin Esplora CLI API

Usage: esplora-cli [NETWORK] <COMMAND>

Commands:
  get-tx                        Get transaction option by id
  get-tx-at-block-index         Get transaction at block index
  get-tx-status                 Get transaction status by id
  get-header-by-hash            Get block header by block hash
  get-block-status              Get block status by block hash
  get-block-by-hash             Get block by block hash
  get-merkle-proof              Get transaction merkle proof by tx id
  get-merkle-block              Get transaction merkle block inclusion proof by id
  get-output-status             Get output spending status by tx id and output index
  broadcast                     Broadcast transaction
  get-height                    Get blockchain tip height
  get-tip-hash                  Get current blockchain tip block hash
  get-block-hash                Get block hash at height
  get-fee-estimates             Get a fee estimate by confirmation target in sat/vB
  get-script-hash-transactions  Get confirmed transaction history for the specified address/scripthash sorted by date
  get-blocks                    Get recent block summaries at the tip or at height if provided (max summaries is backend dependant)
  help                          Print this message or the help of the given subcommand(s)

Arguments:
  [NETWORK]  [default: https://blockstream.info/api]

Options:
  -h, --help     Print help
  -V, --version  Print version
```
