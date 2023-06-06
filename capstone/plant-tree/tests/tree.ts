import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Farmer } from '../target/types/farmer';
import { Farm } from '../target/types/farm';
import { Tree } from '../target/types/tree';
import { FruitMarket } from '../target/types/fruit_market';
import * as token from '@solana/spl-token';
import { assert } from 'chai';
import {
	farmer,
	farm,
	landMeta,
	landPiece,
	cultivarMeta,
	treesMeta,
	waterMint,
	nutrientMintAuthority,
	nitrogenMint,
	potassiumMint,
	phosphorusMint,
	phosphorusBalance,
	potassiumBalance,
	nitrogenBalance,
	waterBalance,
	cultivar,
	fruitMintAuthority,
	fruitMint,
	fruitBalance,
	tree,
	requiredNutrients,
	inputBalance,
	vault,
	seedsBalance,
	seedsAuthority,
	marketAuthority,
} from './pdas';


// Configure the client to use the local cluster.
let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.Tree as Program<Tree>;
const farmProgram = anchor.workspace.Farm as Program<Farm>;
const farmerProgram = anchor.workspace.Farmer as Program<Farmer>;
const marketProgram = anchor.workspace.FruitMarket as Program<FruitMarket>;

// payer
let payer = provider.wallet as anchor.Wallet;


// // fruit_market
// // let [fruitMarket] = anchor.web3.PublicKey.findProgramAddressSync(
// // 	[Buffer.from('fruitmarket')],
// // 	program.programId
// // );

// // let [fruitMarket] = anchor.web3.PublicKey.findProgramAddressSync(
// // 	[Buffer.from('fruitmarket'), Buffer.from(cultivarName)],
// // 	program.programId
// // );
// // land_meta


// //land_piece


let cultivarName = 'Muti';
let initHeight = new anchor.BN(50000000); //micrometers => 50cm
//cultivar
let initWidth = new anchor.BN(2000000); //2cm



// // // fruitVaultAuthority,
// // let [fruitVaultAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
// // 	[Buffer.from('fruitvaultauthority')],
// // 	program.programId
// // );

// // // 	fruitVault,
// // let [fruitVault] = anchor.web3.PublicKey.findProgramAddressSync(
// // 	[Buffer.from('fruit'), Buffer.from(cultivarName)],
// // 	program.programId
// // );

// // // 	marketEntry,
// // let [marketEntry] = anchor.web3.PublicKey.findProgramAddressSync(
// // 	[
// // 		Buffer.from('marketentry'),
// // 		fruitMarket.toBuffer(),
// // 		payer.publicKey.toBuffer(),
// // 	],
// // 	program.programId
// // );

// // before("before", () => {
	
// // 	});


describe('Creates a cultivar', () => {
  it('Creates a cultivar', async () => {
			// currentTopMarketEntry,
			// let [currentTopMarketEntry] =
			// 	anchor.web3.PublicKey.findProgramAddressSync(
			// 		[
			// 			Buffer.from('marketentry'),
			// 			fruitMarket.toBuffer(),
			// 			program.programId.toBuffer(),
			// 		],
			// 		program.programId
			// 	);

			
				let tx = await program.methods
					.createCultivar(cultivarName, initHeight, initWidth)
					.accounts({
						farm,
						farmer,
						cultivarMeta,
						cultivar,
						fruitMint,
						fruitMintAuthority,
						seedsBalance,
						seedsAuthority,
						farmProgram: farmProgram.programId,
					})
					.rpc();
			console.log('create cultivar transaction', tx);	
			
			let c = await program.account.cultivar.fetch(cultivar);
			let cMeta = await farmProgram.account.cultivarMeta.fetch(cultivarMeta);

			console.log('the cultivar name is,', c.name);
			console.log('the cultivar count in meta is,', cMeta.cultivarsCount.toString());
	});
});

describe('Creates A tree', () => {
	it('Creates a tree', async () => {		

		const tx = await program.methods
			.createTree()
			.accounts({
				farmer,
				farm,
				cultivarMeta,
				cultivar,
				treesMeta,
				tree,
				fruitBalance,
				seedsAuthority,
				seedsBalance,
				fruitMint,
				fruitMintAuthority,
				inputBalance,
				requiredNutrients,
				farmProgram: farmProgram.programId,
			})
			.rpc();
		console.log(`The transaction signature is ${tx.toString()}`);

		let t = await program.account.tree.fetch(tree);
		let tMeta = await farmProgram.account.treesMeta.fetch(treesMeta);

		console.log('the tree cultivar name is,', t.cultivarName);
		console.log('the tree health is,', t.health.toString());
		console.log('the height is ,', t.height.toString());
		console.log('The tree count in meta,', tMeta.treeCount.toString());
    console.log('the tree land number is,', t.landNumber);
	});
});

describe('plants the tree ', () => {
	it('Plants a tree In the farmer', async () => {	
		const tx = await farmerProgram.methods
			.plant()
			.accounts({
				farmer,
				farm,
				landMeta,
				landPiece,
				cultivarMeta,
				cultivar,
				treesMeta,
				tree,
				farmProgram: farmProgram.programId,
				treeProgram: program.programId,
			})
			.rpc();
		console.log(`The transaction signature is ${tx.toString()}`);

		let t = await program.account.tree.fetch(tree);
		let f = await farmerProgram.account.farmer.fetch(farmer);
		let tMeta = await farmProgram.account.treesMeta.fetch(treesMeta);

		console.log('the tree land number is,', t.landNumber);
		
		console.log('farmer tree count ,', f.treeCount.toString());
		// console.log('The tree count in meta,', tMeta.treeCount.toString());
	});
});


describe('Updates a tree', () => {
	it('Updates the tree', async () => {
		// let fm = await token.getMint(provider.connection, fruitMint);

		// let fruitBalance = await token.getOrCreateAssociatedTokenAccount(
		// 	provider.connection,
		// 	payer.payer,
		// 	fruitMint,
		// 	payer.publicKey
		// );

		console.log("the tree metadata account address is,", treesMeta.toString())
		console.log("the Owner of the program, ", farmProgram.programId)
		const tx = await program.methods
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
		let t = await program.account.tree.fetch(tree);

		console.log('Checks and updates the tree data: ', tx.toString());

		console.log('lets see what tree state is like ', t);

		let fb = await token.getAccount(provider.connection, fruitBalance);

		console.log('fruit Account initiailized with free fruitus ', fb);
		console.log('the fruit balance is now ', fb.amount);
	});
});

describe('Adds nutriens', () => {
		it('Waters tree', async () => {
			// let wm = await token.getMint(, waterMint);

			// let wb = await token.getAccount(provider.connection, waterBalance);

			// console.log('The water balance before the transaction is ,', wb.amount);

			const tx = await program.methods
				.waterTree(new anchor.BN(2000))
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
			let wm2 = await token.getAccount(provider.connection, waterBalance);
			console.log('The plant has been watered: ', tx.toString());
			let water = wm2.amount;
			console.log('The water balance is now: {} ', water);
		});

		it('Applies Nitrogen', async () => {
			// let wm = await token.getMint(, waterMint);

			let nb = await token.getAccount(provider.connection, nitrogenBalance);

			console.log(
				'The nitrogen balance before the transaction is ,',
				nb.amount
			);

			const tx = await program.methods
				.addNitrogen(new anchor.BN(2001))
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
			let nb2 = await token.getAccount(provider.connection, nitrogenBalance);
			console.log('The nutrient has been Apllied: ', tx.toString());
			let nitrogen = nb2.amount;
			console.log('The nitrogen balance is now: {} ', nitrogen);
		});
		
		it('Applies Phosphorus', async () => {
		
			let pb = await token.getAccount(provider.connection, phosphorusBalance);

			console.log(
				'The phosphorus balance before the transaction is ,',
				pb.amount
			);

			const tx = await program.methods
				.addPhosphorus(new anchor.BN(2000))
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
			let pb2 = await token.getAccount(provider.connection, phosphorusBalance);
			console.log('The nutrient has been Apllied: ', tx.toString());
			let phosphorus = pb2.amount;
			console.log('The phosphorus balance is now: {} ', phosphorus);
		});
		it('Applies Potassium', async () => {
				// let wm = await token.getMint(, waterMint);

				let pb = await token.getAccount(provider.connection, potassiumBalance);

				console.log(
					'The potassium balance before the transaction is ,',
					pb.amount
				);

				const tx = await program.methods
					.addPotassium(new anchor.BN(2000))
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
				console.log('The nutrient has been Apllied: ', tx.toString());
				let potassium = pb2.amount;
				console.log('The water balance is now: {} ', potassium);
		});
});

describe('Consumes nutrients ', () => {	
	
	it('Burns Nitrogen', async () => {
		// let wm = await token.getMint(, waterMint);

		let nb = await token.getAccount(provider.connection, nitrogenBalance);

		console.log('The nitrogen balance before the transaction is ,', nb.amount);

		const tx = await program.methods
			.consumeNitrogen(new anchor.BN(201))
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
		let nb2 = await token.getAccount(provider.connection, nitrogenBalance);
		console.log('The nutrient has been consumed: ', tx.toString());
		let nitrogen = nb2.amount;
		console.log('The nitrogen balance is now: {} ', nitrogen);
	});

	it('Burns Phosphorus', async () => {
			// let wm = await token.getMint(, waterMint);

			let pb = await token.getAccount(provider.connection, phosphorusBalance);

			console.log(
				'The phosphorus balance before the transaction is ,',
				pb.amount
			);

			const tx = await program.methods
				.consumePhosphorus(new anchor.BN(100))
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
			let pb2 = await token.getAccount(provider.connection, phosphorusBalance);
			console.log('The nutrient has been consumed: ', tx.toString());
			let phosphorus = pb2.amount;
			console.log('The phosphorus balance is now: {} ', phosphorus);
	});

	it('Burns Potassium', async () => {
			// let wm = await token.getMint(, waterMint);

			let pb = await token.getAccount(provider.connection, potassiumBalance);

			console.log(
				'The Potassium balance before the transaction is ,',
				pb.amount
			);
			const tx = await program.methods
				.consumePotassium(new anchor.BN(201))
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
	});
	
});

describe('calculations', () => {
	it('calculates the required nutrients for the period', async () => {
		const tx = await program.methods
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
		const rn = await program.account.requiredNutrients.fetch(requiredNutrients);
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