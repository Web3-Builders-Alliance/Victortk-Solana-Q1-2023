import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Account } from '@solana/web3.js';
import { Starter } from '../target/types/starter';

describe('starter', () => {
	// Configure the client to use the local cluster.
	let provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const program = anchor.workspace.Starter as Program<Starter>;

	// payer
	let payer = provider.wallet as anchor.Wallet;

	// farm
	let [farm] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('farm')],
		program.programId
	);

	let [farmer] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('farmer'), payer.publicKey.toBuffer()],
		program.programId
	);

	// fruit_market
	let [fruitMarket] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('fruitmarket')],
		program.programId
	);
	// land_meta
	let [landMeta] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('landmeta'), farm.toBuffer()],
		program.programId
	);
	//land_piece
	let [landPiece] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('landpiece'), landMeta.toBuffer(), farmer.toBuffer()],
		program.programId
	);
	// cultivar_meta
	let [cultivarMeta] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('cultivarmeta'), farm.toBuffer()],
		program.programId
	);
	let cultivarName = 'Muti';
	let initHeight = new anchor.BN(50000000); //micrometers => 50cm
	//cultivar
	let initWidth = new anchor.BN(2000000); //2cm
	let [cultivar] = anchor.web3.PublicKey.findProgramAddressSync(
		[
			Buffer.from('cultivar'),
			cultivarMeta.toBuffer(),
			Buffer.from(cultivarName),			
		],
		program.programId
	);
	// trees_meta
	let [treesMeta] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('treesmeta'), farm.toBuffer()],
		program.programId
	);

	let [tree] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('tree'), treesMeta.toBuffer(), farmer.toBuffer()],
		program.programId
	);

	// water_mint
	let [waterMint] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('watermint')],
		program.programId
	);
	// nutrient_mint_authority

	let [nutrientMintAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('nutrientmintauthority')],
		program.programId
	);

	//vault
	let [vault] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('carbonvault')],
		program.programId
	);

	// fruitMintAuthority,

	let [fruitMintAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('fruitmintauthority'), tree.toBuffer()],
		program.programId
	);

	// 	fruitMint,
	let [fruitMint] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('fruitmint'), Buffer.from(cultivarName)],
		program.programId
	);

	// 	fruitBalance,

	let [fruitBalance] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('fruit'), tree.toBuffer()],
		program.programId
	);

	// 	inputBalance,

	let [inputBalance] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('nutrientbalance'), landPiece.toBuffer(), tree.toBuffer()],
		program.programId
	);

	it('Initializes the Farm', async () => {
		// Add your test here.
		const tx = await program.methods
			.initializeFarm()
			.accounts({
				farm,
				fruitMarket,
				landMeta,
				cultivarMeta,
				treesMeta,
				waterMint,
				nutrientMintAuthority,
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
	it('Initializes the Farmer', async () => {
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

	it('Buys land', async () => {
		const tx = await program.methods
			.buyLand()
			.accounts({
				farm,
				farmer,
				landMeta,
				landPiece,
				vault,
			})
			.rpc();
		console.log('Your transaction signature', tx);
		let farmerState2 = await program.account.farmer.fetch(farmer);
		let landState2 = await program.account.landMeta.fetch(landMeta);
		let landPieceState2 = await program.account.landPiece.fetch(landPiece);
		console.log('the famer land count is,', farmerState2.landCount.toString());
		console.log(
			'There are now ' +
				landState2.landPieceCount.toString() +
				' owned land pieces'
		);

		console.log('Land piece authority: ', landPieceState2.owner.toString());

		console.log('Land piece number: ', landPieceState2.number.toString());

		console.log('Land is planted? ', landPieceState2.isPlanted);
	});
	it('Creates a cultivar', async () => {
			let tx = await program.methods
				.createCultivar(cultivarName, initHeight, initWidth)
				.accounts({
					farm,
					cultivarMeta,
					cultivar,
				})
				.rpc();
			console.log('create cultivar transaction', tx);
	});
	it('Plants a tree', async () => {
			const tx = await program.methods
			.plantTree(cultivarName)
			.accounts({
				farm,
				farmer,
				cultivarMeta,
				cultivar,
				landMeta,
				landPiece,
				treesMeta,
				tree,
				fruitMintAuthority,
				fruitMint,
				fruitBalance,
				inputBalance,
			})
			.rpc();
		  console.log(`The transaction signature is ${tx.toString()}`);
	});
});
