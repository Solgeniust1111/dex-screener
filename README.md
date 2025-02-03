# DEX Checker: Solana Token Market Data Retrieval

## Overview

DEX Checker is a Rust application designed to fetch and analyze market data for Solana tokens from various Automated Market Makers (AMMs) including Pump.fun, Raydium, Orca, and Meteora. It aims to provide liquidity, price, and market cap information in an efficient manner by utilizing asynchronous programming to fetch data from multiple sources in parallel.

## Features

- **Asynchronous Data Retrieval**: Leverages Tokio for non-blocking I/O to fetch data concurrently from multiple DEX sources.
- **Health Analysis**: Evaluates the health of liquidity pools by analyzing market data to determine the best option for traders.
- **Flexibility**: Supports any Solana token by allowing users to input a mint address, fetching relevant data accordingly.
- **Integration**: Works seamlessly with existing Rust projects and can be integrated with other financial analysis tools.

## Installation

To get started with DEX Checker, ensure you have Rust and Cargo installed on your machine. Clone the repository and build the project:

```bash
cd dex_checker
cargo build
```

## Usage

Run the main application with a specified mint address:

```bash
cargo run -- E6AujzX54E1ZoPDFP2CyG3HHUVKygEkp6DRqig61pump
```

Replace the mint address with the desired Solana token mint.

## Testing

To run tests included in the project, utilize Cargo’s testing capabilities:

```bash
cargo test
```

This will run all tests specified in the test modules of your project.

