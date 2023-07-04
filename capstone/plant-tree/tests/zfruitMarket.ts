import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { FarmerProgram } from '../target/types/farmer_program';
import { FarmProgram } from '../target/types/farm_program';
import { FruitMarketProgram} from '../target/types/fruit_market_program';
import {TreeProgram} from '../target/types/tree_program';
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
	seedsAuthority,
	// fruitVaultAuthority,
} from './pdas';


let cultivarName = 'Muti';
// Configure the client to use the local cluster.
let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.FruitMarketProgram as Program<FruitMarketProgram>;
const treeProgram = anchor.workspace.TreeProgram as Program<TreeProgram>;
const farmProgram = anchor.workspace.FarmProgram as Program<FarmProgram>;
const farmerProgram = anchor.workspace.FarmerProgram as Program<FarmerProgram>;


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


describe('Creates Seed balance Account', () => {
	it('Creates Seed balance Account', async () => {
		const tx = await treeProgram.methods
			.createSeed(cultivarName)
			.accounts({
				farmer,
				farm,
				fruitMintAuthority,
				fruitMint,
				seedsAuthority,
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
	
				farm,
				cultivarMeta,
				cultivar,
				fruitMintAuthority,
				fruitMint,
				seedsAuthority,
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

describe('Searches for trees', () => {
	it('Searches for trees', async () => {

		let trees = await treeProgram.account.tree.all([
			{
				memcmp:{
				  offset:8,
					bytes: payer.publicKey.toBase58().toString()
				}  
				
			}
		])

		console.log("The trees", trees) ;
		console.log("The payer key v1 ", payer.publicKey.toBase58().toString()) ;
		console.log("The payer key v2 ", payer.publicKey.toString()) ;
	
  })
})

describe('calculations 2', () => {
	it('calculates the required nutrients for the period 2', async () => {
		const tx = await treeProgram.methods
			.calculateRequired()
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
		const rn = await treeProgram.account.requiredNutrients.fetch(requiredNutrients);
		console.log(
			'The nutrient required since last apply has been calculated: ',
			tx.toString()
		);
		console.log(
			`required Nitrogen: ${rn.nitrogen} Required phosphorus: ${rn.phosphorus} Required potassium: ${rn.potassium} Required water: ${rn.water}`
		);

		console.log(
			`Available Nitrogen: ${rn.percentAvailableNitrogen} Available phosphorus: ${rn.percentAvailablePhosphorus} Available potassium: ${rn.percentAvailablePotassium} Available water: ${rn.percentAvailableWater}`
		);
	});
})

describe('Consumes required nutrients 2', () => {
	it('Consumes required nutrients 2', async () => {

			// let wm = await token.getMint(, waterMint);

			let pb = await token.getAccount(provider.connection, potassiumBalance);

			console.log(
				'The Potassium balance before the transaction is ,',
				pb.amount
			);
			const tx = await treeProgram.methods
				.consumeNutrients()
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
			let pb2 = await token.getAccount(provider.connection, potassiumBalance);
			console.log('The nutrient has been consumed: ', tx.toString());
			let potassium = pb2.amount;
			console.log('The phosphorus balance is now: {} ', potassium);
	
  })
})


describe('Updates a tree 2', () => {
	it('Updates the tree 2', async () => {
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

describe('Close Farm', () => {
	it('Close the Farm', async () => {
		const fb = await farmProgram.account.farm.fetch(farm);
		
		console.log('The addresss to close ', farm);
		console.log('farm before', fb);
		// Add your test here.
		const tx = await farmProgram.methods
			.closeFarm()
			.accounts({
				farm,
				landMeta,
				cultivarMeta,
				treesMeta,
			})
			.rpc();

		console.log('The farm is closed', tx);

		// const f = await farmProgram.account.farm.fetch(farm);

		// console.log('farm after', f);
	});
});




