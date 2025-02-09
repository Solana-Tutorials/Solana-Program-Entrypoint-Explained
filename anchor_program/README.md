## Prerequisites

Before you get started, ensure you have both the [Solana CLI and the Anchor CLI](https://solana.com/docs/intro/installation) installed.

## Running the Example

Follow these steps to run the example program:

### 1. Install Dependencies

Install the TypeScript dependencies (for the test file) by running:

```
yarn
```

### 2. Run the Test File

Run the `anchor test` command, which will build the program, deploy it to a local validator, execute the tests, and then stop the validator.

```sh
anchor test
```

You should see output similar to this:

```
  anchor_program
Your transaction signature 2xmUJc7pwvtxpZ1PgAtEaRt9kJFExUdjY5o2ccgY8Nfkz73BpEpfhRxsKiB2knYtUTBdvnBmWE2zztG93SUNS3iX
    ✔ Is initialized! (337ms)

  1 passing (344ms)
```

If you want the local validator to remain active after the tests, run:

```sh
anchor test --detach
```

### 3. Running Steps Individually (Optional)

Alternatively, you can execute the steps separately:

1. **Start the Local Validator:**
   ```sh
   solana-test-validator
   ```
2. **Build the Program:**
   This will also update your program ID automatically.
   ```sh
   anchor build
   ```
3. **Synchronize Keys:**
   This will ensure the program ID in the declare_id! macro is updated.
   ```sh
   anchor keys sync
   ```
4. **Deploy the Program:**
   ```sh
   anchor deploy
   ```
5. **Run the Tests:**
   ```sh
   anchor run test
   ```

Follow these instructions to build, deploy, and test the example program seamlessly.

## Additional Information

The `Anchor.toml` file contains the following configuration:

```
[provider]
cluster = "Localnet"
wallet = "~/.config/solana/id.json"
```

This configuration tells Anchor to use the local validator and the keypair located in the `~/.config/solana/id.json` path as the upgrade authority and payer for the deployed program.
