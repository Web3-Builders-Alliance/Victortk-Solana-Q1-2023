import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Starter } from "../target/types/starter";

describe("starter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  let authority_key = provider.wallet.publicKey as anchor.Wallet

  const program = anchor.workspace.Starter as Program<Starter>;

  it("Is initialized!", async () => {
    

    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
