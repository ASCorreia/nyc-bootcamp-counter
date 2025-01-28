import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterNyc } from "../target/types/counter_nyc";

describe("counter-nyc", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CounterNyc as Program<CounterNyc>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize()
    .accounts({
      user: provider.publicKey,
    })
    .rpc();

    console.log("User account Initialized - Your transaction signature", tx);
  });

  it("Increments counter", async () => {
    // Add your test here.
    const tx = await program.methods.increment()
    .accounts({
      user: provider.publicKey,
    })
    .rpc();

    console.log("Counter incremented - Your transaction signature", tx);
  });
});
