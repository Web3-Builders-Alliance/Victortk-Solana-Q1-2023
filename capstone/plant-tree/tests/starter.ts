import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Account } from "@solana/web3.js";
import { Starter } from "../target/types/starter";

describe("starter", () => {
	// Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env() ;
	anchor.setProvider(provider);

	const program = anchor.workspace.Starter as Program<Starter>;

	// payer
  let payer = provider.wallet as  anchor.Wallet ;
   
	// farm 
	let [farm] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from("farm")],
		program.programId
		);
  
  let [farmer] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from("farmer"),
		payer.publicKey.toBuffer()],
		program.programId);

	// fruit_market
  let [fruitMarket] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('fruitmarket')],
		program.programId
	);
	// land_meta
  let [landMeta] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from("landmeta"), farm.toBuffer()],
		program.programId
	);

	// cultivar_meta
	let [cultivarMeta] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from("cultivarmeta"), farm.toBuffer()],
		program.programId
	);
	// trees_meta

	let [treesMeta] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from("treesmeta"),farm.toBuffer()],
		program.programId 
	)
	// water_mint
	let[ waterMint] = anchor.web3.PublicKey.findProgramAddressSync (
		[Buffer.from("watermint")],
		program.programId 
	);
	// nutrient_mint_authority

	let [nutrientMintAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from('nutrientmintauthority')],
		program.programId
	);

	//vault 
	let [vault] = anchor.web3.PublicKey.findProgramAddressSync([
		Buffer.from('carbonvault'),
	], 
	 program.programId ,
	);

	it('Initializes the Farm', async () => {
		// Add your test here.
		const tx = await program.methods.initializeFarm().accounts({
			farm,
			fruitMarket,			
			landMeta,
			cultivarMeta,
			treesMeta,
			waterMint,
			nutrientMintAuthority,
			vault
	  }).rpc();

		console.log('Your transaction signature', tx);

		let treesState = await program.account.treesMeta.fetch(treesMeta);
		let cultivarState = await program.account.cultivarMeta.fetch(cultivarMeta);
    let landState = await program.account.landMeta.fetch(landMeta);
		console.log("There are now " + treesState.treeCount.toString() + " trees on the Farm");
		console.log("There are now " + cultivarState.cultivarsCount.toString() + " cultivars on the Farm");
		console.log(
			'There are now ' +
				landState.landPieceCount.toString() +
				' trees on the Farm'
		);

	
	});
	it('Initializes the Farmer', async() => {
		let name = "Jonh Doe";
		const tx = await program.methods
		.initializeFarmer(name)
		.accounts({
			farmer,			
		})
		.rpc() ;

		console.log('Your transaction signature', tx);

		let farmerState = await program.account.farmer.fetch(farmer);
		console.log("the famer name is,",farmerState.name);
		console.log("the famer address is,",farmerState.address.toString());
		console.log("the famer land count is,",farmerState.landCount.toString());
		console.log("the famer tree count is,",farmerState.treeCount.toString());

	})
	
});
