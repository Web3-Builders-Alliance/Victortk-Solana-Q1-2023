import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Farmer } from '../target/types/farmer';
import { Tree } from '../target/types/tree';
import { Farm } from '../target/types/farm';
import { FruitMarket } from '../target/types/fruit_market';
import * as token from '@solana/spl-token';

import {
	farmer,
	farm,
	cultivarMeta,
	treesMeta,
	cultivar,
	fruitMintAuthority,
	fruitMint,
	fruitBalance,
	tree,
	marketEntry,
	fruitMarket,
	entryFruitBalance,
	fruitVault,
	waterMint,
	nitrogenMint,
	potassiumMint,
	phosphorusMint,
	nutrientMintAuthority,
	landMeta,
	landPiece,
	inputBalance,
	waterBalance,
	nitrogenBalance,
	phosphorusBalance,
	potassiumBalance,
	requiredNutrients,
	vault,
	seedsBalance,
	marketAuthority,
	// fruitVaultAuthority,
} from './pdas';


let cultivarName = 'Muti';
// Configure the client to use the local cluster.
let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.FruitMarket as Program<FruitMarket>;
const treeProgram = anchor.workspace.Tree as Program<Tree>;
const farmProgram = anchor.workspace.Farm as Program<Farm>;
const farmerProgram = anchor.workspace.Farmer as Program<Farmer>;


// payer
let payer = provider.wallet as anchor.Wallet;

describe('Updates a tree', () => {
	it('Updates the tree', async () => {
		// let fm = await token.getMint(provider.connection, fruitMint);

		// let fruitBalance = await token.getOrCreateAssociatedTokenAccount(
		// 	provider.connection,
		// 	payer.payer,
		// 	fruitMint,
		// 	payer.publicKey
		// );

		console.log('the tree metadata account address is,', treesMeta.toString());
		console.log('the Owner of the program, ', farmProgram.programId);
		
		const tx = await treeProgram.methods
			.checkAndUpdate()
			.accounts({
				farm,
				farmer,
				waterMint,
				nitrogenMint,
				potassiumMint,
				phosphorusMint,
				nutrientMintAuthority,
				landMeta,
				treesMeta,
				tree,
				landPiece,
				inputBalance,
				waterBalance,
				nitrogenBalance,
				phosphorusBalance,
				potassiumBalance,
				fruitMintAuthority,
				fruitMint,
				fruitBalance,
				requiredNutrients,
				vault,
				farmProgram: farmProgram.programId,
			})
			.rpc();
		let t = await treeProgram.account.tree.fetch(tree);

		console.log('Checks and updates the tree data: ', tx.toString());

		console.log('lets see what tree state is like ', t);

		let fb = await token.getAccount(provider.connection, fruitBalance);

		console.log('fruit Account initiailized with free fruitus ', fb);
		console.log('the fruit balance is now ', fb.amount);
	});
});


describe('Creates Harvest Account or fruit vault', () => {
		it("This initializes the user's  fruit vault", async () => {
			const tx = await farmerProgram.methods
					.createHarvestAccount(cultivarName)
					.accounts({
						payer: payer.publicKey,
						farmer,
						fruitMint,
						fruitMintAuthority,
						fruitVault
					})
					.rpc();
				console.log(
					'the market initialized successfully , transaction hash:',
					tx)
		})
})

describe('Harvest fruits', () => {
		it("Harvests fruits", async () => {
       	const tx = await treeProgram.methods
					.harvestFruit(new anchor.BN(2))
					.accounts({
						payer: payer.publicKey,
						farm,
						farmer,
						treesMeta,
						fruitBalance,
						fruitMint,
						fruitMintAuthority,
						fruitVault,
						tree,
						farmProgram: farmProgram.programId,
					})
					.rpc();
				console.log(
					'the market initialized successfully , transaction hash:',
					tx
				);
		})
})

describe('Initializes the market', () => {
	it('Initializes the market', async () => {
		let [marketEntry] = anchor.web3.PublicKey.findProgramAddressSync(
			[
				Buffer.from('marketentry'),
				fruitMarket.toBuffer(),
				program.programId.toBuffer(),
			],
			program.programId
		);

		const tx = await program.methods
			.initializeMarket()
			.accounts({
				payer: payer.publicKey,
				farm,
				farmer,
				treesMeta,
				tree,
				fruitMarket,
				marketEntry,
				farmProgram: farmProgram.programId,
				treeProgram: treeProgram.programId,
			})
			.rpc();	
			console.log('the market initialized successfully , transaction hash:', tx);
	})
})
	

describe('Creates Market ', () => {
	it('Creates market Entry', async () => {
		// let t = await program.account.tree.fetch(tree);
		let fm = await program.account.fruitMarket.fetch(fruitMarket);
		// currentTopMarketEntry,
		let [currentTopMarketEntry] = anchor.web3.PublicKey.findProgramAddressSync(
			[
				Buffer.from('marketentry'),
				fruitMarket.toBuffer(),
				fm.topMaker.toBuffer(),
			],
			program.programId
		);

		// 	entryFruitBalance,
		let [entryFruitBalance] = anchor.web3.PublicKey.findProgramAddressSync(
			[Buffer.from('fruit'), marketEntry.toBuffer()],
			program.programId
		);
		let tknn = await token.getAccount(provider.connection, fruitBalance);
		console.log(`token tkn b4: ${tknn.amount}`);
		// let fv = await token.getAccount(provider.connection, fruitVault);
		// console.log(`token vault tkn b4: ${fv.amount}`);
		// console.log('currentTopMarketEntry ', currentTopMarketEntry.toString());
		// console.log('EntryFruitBalance ', entryFruitBalance.toString());
		// console.log(farmer.toString());

		const tx = await program.methods
			.createMarket(cultivarName)
			.accounts({
				payer: payer.publicKey,
				fruitMarket,
				fruitMintAuthority,
				fruitMint,
				marketEntry,
				currentTopMarketEntry,
				marketAuthority,
				entryFruitBalance,
			})
			.rpc();

		console.log('the fruits have been listed , transaction hash:', tx);
		let fm2 = await program.account.fruitMarket.fetch(fruitMarket);
		console.log('the fruit market account:', fm2);

		// console.log('the fruit market address:', fm2.address);

		console.log(`the new fruit market top is ${fm2.topMaker.toString()}`);
		let tkn = await token.getAccount(provider.connection, fruitBalance);

		let fvv = await token.getAccount(provider.connection, entryFruitBalance);

		console.log(`farmer fruit balance tkn b4: ${tkn.amount}`);

		console.log(`token entry address: ${fvv.address}`);

		console.log(`token entry tkns after: ${fvv.amount}`);
	});
});


describe('Lists fruits', () => {
	it('Lists fruits to market account', async () => {
		let fm = await program.account.fruitMarket.fetch(fruitMarket);

		// currentTopMarketEntry,
		let [currentTopMarketEntry] = anchor.web3.PublicKey.findProgramAddressSync(
			[
				Buffer.from('marketentry'),
				fruitMarket.toBuffer(),
				fm.topMaker.toBuffer(),
			],
			program.programId
		);

		// 	entryFruitBalance,

		const tx = await farmerProgram.methods
			.listFruits(cultivarName, new anchor.BN(2))
			.accounts({
				payer: payer.publicKey,
				farm,
				farmer,
				fruitMarket,
				marketEntry,
				fruitMint,
				fruitMintAuthority,
				fruitVault,
				currentTopMarketEntry,
				entryFruitBalance,
				marketAuthority,
				marketProgram: program.programId,
				farmProgram: farmProgram.programId,
			})
			.rpc();
		console.log('the market initialized successfully , transaction hash:', tx);
	});
});


describe('Creates Seed balnce Account', () => {
	it('Creates Seed balnce Account', async () => {
		const tx = await treeProgram.methods
			.createSeed(cultivarName)
			.accounts({
				farmer,
				farm,
				fruitMintAuthority,
				fruitMint,
				seedsBalance,
				farmProgram: farmProgram.programId,
			})
			.rpc();
	  	console.log('The seed balance has ben initialized : -> ', tx);

	})
})


describe('Buys fruits', () => {
	it('buys fruits', async () => {
		let fm = await program.account.fruitMarket.fetch(fruitMarket);

		// console.log('fruit market address is now ', fm.address);
		console.log('Same as: ', fm);

		console.log(`the  fruit market top is ${fm.topMaker.toString()}`);

		// console.log(`token entry address: ${fvv.address}`);

		// console.log(`token entry tkns after: ${fvv.amount}`);

		// // currentTopMarketEntry,
		let [currentTopMarketEntry] = anchor.web3.PublicKey.findProgramAddressSync(
			[
				Buffer.from('marketentry'),
				fruitMarket.toBuffer(),
				fm.topMaker.toBuffer(),
			],
			program.programId
		);

		console.log(
			'one ' + payer.publicKey.toString() + ' vs two ' + fm.topMaker.toString()
		);

		let [topEntryFruitBalance] = anchor.web3.PublicKey.findProgramAddressSync(
			[Buffer.from('fruit'), currentTopMarketEntry.toBuffer()],
			program.programId
		);

		console.log('currentTopMarketEntry, ', currentTopMarketEntry.toString());

		console.log('topEntryFruitBalance ', topEntryFruitBalance.toString());

		// let balance = await token.getAccount(provider.connection , topEntryFruitBalance);

		// console.log('topEntryFruitBalance before ', balance.amount);

		// let [seedVault] = anchor.web3.PublicKey.findProgramAddressSync(
		// 	[Buffer.from('seedvault'), farmer.toBuffer()],
		// 	program.programId
		// );

	
		const tx = await program.methods
			.buyFruits(cultivarName)
			.accounts({
				farmer,
				farm,
				cultivarMeta,
				cultivar,
				fruitMintAuthority,
				fruitMint,
				seedsBalance,
				fruitMarket,
				currentTopMarketEntry,
				topEntryFruitBalance,
				farmProgram: farmProgram.programId,
				treeProgram: treeProgram.programId,
				marketAuthority,
				
			})
			.rpc();

		console.log('The buy fruit transaction has been processed: -> ', tx);

		let balance2 = await token.getAccount(
			provider.connection,
			topEntryFruitBalance
		);

		console.log('topEntryFruitBalance in now ', balance2.amount);

		let seed = await token.getAccount(provider.connection, seedsBalance);

		console.log('seedsBalance is now ', seed.amount);
	});
});



