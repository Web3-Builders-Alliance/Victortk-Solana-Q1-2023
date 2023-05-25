import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { ConfirmOptions, PublicKey } from '@solana/web3.js';
import { Farm } from '../target/types/farm';
// import { Farmer } from '../target/types/farmer';
import { Tree } from '../target/types/tree';
import * as token from '@solana/spl-token';
import { assert } from 'chai';
import {
	farmer,
	farm,
	landMeta,
	landPiece,
	cultivarMeta,
	treesMeta,
	vault,
} from './pdas';

let cultivarName = 'Muti';
// Configure the client to use the local cluster.
let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.Farm as Program<Farm>;

// payer
let payer = provider.wallet as anchor.Wallet;


describe('Initialize', () => {
	it('Initializes the Farm', async () => {
		// Add your test here.
		const tx = await program.methods
			.initializeFarm()
			.accounts({
				farm,
				landMeta,
				cultivarMeta,
				treesMeta,			
				vault,
			})
			.rpc();

		console.log('Your transaction signature', tx);

		let treesState = await program.account.treesMeta.fetch(treesMeta);
		let cultivarState = await program.account.cultivarMeta.fetch(cultivarMeta);
		let landState = await program.account.landMeta.fetch(landMeta);
		console.log(
			'There are now ' + treesState.treeCount.toString() + ' trees on the Farm'
		);
		console.log(
			'There are now ' +
				cultivarState.cultivarsCount.toString() +
				' cultivars on the Farm'
		);
		console.log(
			'There are now ' +
				landState.landPieceCount.toString() +
				' owned land pieces'
		);
	});
})



describe('farm Updates' , () => {
	it("add one tree to trees meta" , async () => {
		const tx = await program.methods
			.updateTrees()
			.accounts({
				farm,
				landMeta,
				vault,
				cultivarMeta,
				treesMeta,
			})
			.rpc();
		console.log("the tree meta updated", tx) ;
		let treesM = await program.account.treesMeta.fetch(treesMeta);
		console.log('The trees count is now: ', treesM.treeCount.toString());

	})

	it("add one cultivars meta" , async () => {
		const tx = await program.methods
			.updateCultivars()
			.accounts({
				farm,
				landMeta,
				vault,
				cultivarMeta,
				treesMeta,
			})
			.rpc();
		console.log('the cultivar meta updated', tx);
		let cultivarM = await program.account.cultivarMeta.fetch(cultivarMeta);
		console.log('The cultivar count is now: ', cultivarM.cultivarsCount.toString());

	})
	
});

