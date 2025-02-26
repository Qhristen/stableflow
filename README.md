# StableFlow

StableFlow is a decentralized yield aggregator built on Solana, designed to maximize returns for stablecoins. By aggregating yield opportunities across multiple DeFi protocols, StableFlow enables users to deposit their stablecoins and earn optimized returns with minimal effort. Leveraging Solana's speed and low transaction costs, StableFlow provides a seamless, cost-efficient, and user-friendly experience for both new and experienced DeFi users.

## Features

- **Automated Yield Optimization**: The platform employs smart contracts to allocate stablecoins to the highest-yielding opportunities.
- **Multi-Protocol Aggregation**: Integrates with various Solana-based DeFi protocols to maximize returns.
- **Low Transaction Costs**: Built on Solana to ensure low fees and fast transactions.
- **Stablecoin Support**: Supports USDC, USDT, and other Solana-based stable assets.
- **User-Friendly Interface**: Easy-to-use interface for depositing, withdrawing, and tracking earnings.
- **Secure & Transparent**: Fully decentralized and auditable on Solana.

## How It Works

1. **Deposit Stablecoins**: Users deposit supported stablecoins into the StableFlow protocol.
2. **Automated Allocation**: The protocol automatically allocates funds to the most profitable yield strategies.
3. **Earnings Generation**: Yield is generated through DeFi lending, staking, and liquidity provision.
4. **Withdraw Anytime**: Users can withdraw their funds along with accrued earnings at any time.

## Prerequisites

To run the StableFlow Anchor program, you need:

- **Rust**: Install Rust using [Rustup](https://rustup.rs/)
- **Anchor**: Install Anchor framework by running:
  ```sh
  cargo install --git https://github.com/coral-xyz/anchor avm --force
  ```
- **Solana CLI**: Install and set up Solana CLI:
  ```sh
  sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
  solana config set --url https://api.mainnet-beta.solana.com or
  solana config set --url https://api.devnet.solana.com
  ```
- **Node.js & Yarn** (for front-end and scripting):
  ```sh
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh | bash
    command -v nvm
    nvm install node
  ```

## Running the Anchor Program Locally

1. **Start a Local Solana Test Validator**
   ```sh
   solana-test-validator -r
   ```
2. **Build and Deploy the Program Locally**
   ```sh
   anchor build
   anchor deploy
   ```
3. **Run Tests**
   ```sh
   anchor test
   ```

## Usage

### Deposit Stablecoins

Users can deposit stablecoins into the StableFlow protocol through the front-end interface.

### Withdraw Earnings

Withdraw funds anytime by calling the withdrawal function on the contract, ensuring liquidity and flexibility for users.

## Deployment to Mainnet

1. **Set Up Solana Keypair**
   ```sh
   solana-keygen new --outfile ~/.config/solana/id.json
   ```
2. **Build and Deploy to Solana Mainnet**
   ```sh
   anchor build
   anchor deploy --provider.cluster mainnet-beta
   ```
3. **Verify Deployment**
   ```sh
   solana program show <PROGRAM_ID>
   ```

## License

StableFlow is open-source and licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

