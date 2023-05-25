import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
// import { Starter } from '../target/types/starter';
import { Farmer } from '../target/types/farmer';
import { Farm } from '../target/types/farm';
import { FruitMarket} from '../target/types/fruit_market';
import {Tree} from '../target/types/tree';
import * as token from '@solana/spl-token';

// import { assert } from 'chai';
import {
	farmer,

	vault,
	farm,
	landMeta,
	landPiece,

	
} from './pdas';

let cultivarName = 'Muti';
// Configure the client to use the local cluster.
let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);


const program = anchor.workspace.Farmer as Program<Farmer>;
const farmProgram = anchor.workspace.Farm as Program<Farm>;
const marketProgram = anchor.workspace.FruitMarket as Program<FruitMarket>;
const treeProgram = anchor.workspace.Tree as Program<Tree>;

	// payer
let payer = provider.wallet as anchor.Wallet;

// console.log('the farmer should be ', farmer);

describe('Initializes the Farmer', () => {
		it ("initializes the farmer" , async () => {
					let name = 'Jonh Doe';
					const tx = await program.methods
						.initializeFarmer(name)
						.accounts({
							farmer,
						})
						.rpc();

					console.log('Your transaction signature', tx);

					let farmerState = await program.account.farmer.fetch(farmer);

					console.log('the famer name is,', farmerState.name);
					console.log('the famer address is,', farmerState.address.toString());
					console.log('the famer land count is,', farmerState.landCount.toString());
					console.log('the famer tree count is,', farmerState.treeCount.toString());
			});
});

	describe("Update farmer values:", async() =>{
		it("adds land count", async () => {
			const tx = await program.methods
					.addLandCount()
					.accounts({
						farmer,
					})
					.rpc();

				console.log('Your land count signature is', tx);
				let farmerState = await program.account.farmer.fetch(farmer);

				console.log('the famer name is,', farmerState.name);
				console.log('the famer address is,', farmerState.address.toString());
				console.log(
					'the famer land count is,',
					farmerState.landCount.toString()
				);
				console.log(
					'the famer tree count is,',
					farmerState.treeCount.toString()
				);
		})
		it("adds tree count", async () => {
			const tx = await program.methods
					.addTreeCount()
					.accounts({
						farmer,
					})
					.rpc();
			 console.log('Your tree count transaction signature', tx);
			 let farmerState = await program.account.farmer.fetch(farmer);

				console.log('the famer name is,', farmerState.name);
				console.log('the famer address is,', farmerState.address.toString());
				console.log(
					'the famer land count is,',
					farmerState.landCount.toString()
				);
				console.log(
					'the famer tree count is,',
					farmerState.treeCount.toString()
				);
		})
			 
	});
	describe('farm Operations', () => {
		it('Buys land', async () => {
			const tx = await program.methods
				.buyLand()
				.accounts({
					farm,
					farmer,
					landMeta,
					landPiece,
					vault,
					farmProgram: farmProgram.programId,
				})
				.rpc();
			console.log('Your transaction signature', tx);
			let farmerState2 = await program.account.farmer.fetch(farmer);
			let landState2 = await farmProgram.account.landMeta.fetch(landMeta);
			let landPieceState2 = await program.account.landPiece.fetch(landPiece);
			console.log(
				'There are now ' +
					landState2.landPieceCount.toString() +
					' owned land pieces'
			);

			console.log('Land piece authority: ', landPieceState2.owner.toString());

			console.log('Land piece number: ', landPieceState2.number.toString());

			console.log('Land is planted? ', landPieceState2.isPlanted);
		});

	// it('sets the land piece is_planted to true', async () => {
	// 	const tx = await program.methods
	// 		.updateLand()
	// 		.accounts({
	// 			farm,
	// 			farmer,
	// 			landMeta,
	// 			landPiece,
	// 			vault,
	// 			farmerProgram: farmerProgram.programId,
	// 			cultivarMeta,
	// 			treesMeta,
	// 		})
	// 		.rpc();
	// 	console.log('the land piece is planted on ', tx);
	// 	let landP = await program.account.landPiece.fetch(landPiece);

	// 	console.log('landPiece is planted on: ', landP.isPlanted);
	// });
	});

	





	