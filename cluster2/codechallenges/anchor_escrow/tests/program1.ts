import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Program1 } from "../target/types/program1";
import {
	PublicKey,
	SystemProgram,
	Transaction,
	Connection,
	Commitment,
} from '@solana/web3.js';
import { assert } from 'chai';

describe("program1", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Program1 as Program<Program1>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

});
