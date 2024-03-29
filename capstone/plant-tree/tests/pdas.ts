import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { FarmerProgram } from '../target/types/farmer_program';
import { FarmProgram } from '../target/types/farm_program';
import { TreeProgram } from '../target/types/tree_program';
import { FruitMarketProgram } from '../target/types/fruit_market_program';

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

export let date = new Date().toISOString() ;

let payer = provider.wallet as anchor.Wallet;

const farmerProgram = anchor.workspace.FarmerProgram as Program<FarmerProgram>;

const farmProgram = anchor.workspace.FarmProgram as Program<FarmProgram>;

const fruitMarketProgram = anchor.workspace.FruitMarketProgram as Program<FruitMarketProgram>;

const treeProgram = anchor.workspace.TreeProgram as Program<TreeProgram>;

// farm
export let [farm] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('farm')],
	farmProgram.programId
);

// farmer
export let [farmer] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('farmer'), payer.publicKey.toBuffer()],
	farmerProgram.programId
);


// land_meta
export let [landMeta] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('landmeta'), farm.toBuffer()],
	farmProgram.programId
);

//land_piece
export let [landPiece] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('landpiece'), landMeta.toBuffer(), farmer.toBuffer(), Buffer.from([0,0])],
	farmerProgram.programId
);

// cultivar_meta
export let [cultivarMeta] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('cultivarmeta'), farm.toBuffer()],
	farmProgram.programId
);

let cultivarName = 'Muti';
export let [cultivar] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('cultivar'), cultivarMeta.toBuffer(), Buffer.from(cultivarName)],
	treeProgram.programId
);


// trees_meta
export let [treesMeta] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('treesmeta'), farm.toBuffer()],
	farmProgram.programId
);

//tree
export let [tree] = anchor.web3.PublicKey.findProgramAddressSync(
	[
		Buffer.from('tree'),
		treesMeta.toBuffer(),
		farmer.toBuffer(),
		Buffer.from(cultivarName),
		Buffer.from(date),
	],
	treeProgram.programId
);

// water_mint
export let [waterMint] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('watermint')],
	treeProgram.programId
);

// nutrient_mint_authority

export let [nutrientMintAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('nutrientmintauthority')],
	treeProgram.programId
);

//vault
export let [vault] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('carbonvault')],
	farmProgram.programId
);


// 	inputBalance,
export let [inputBalance] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('nutrientbalance'), tree.toBuffer()],
	treeProgram.programId
);

// nitrogenMint,
export let [nitrogenMint] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('nitrogenmint')],
	treeProgram.programId
);

// potassiumMint,
export let [potassiumMint] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('potassiummint')],
	treeProgram.programId
);
//phosphorusMint,
export let [phosphorusMint] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('phosphorusmint')],
	treeProgram.programId
);


//fruitMintAuthority,
export let [fruitMintAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('fruitmintauthority')],
	treeProgram.programId
);
//fruitMint,
export let [fruitMint] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('fruitmint'), Buffer.from(cultivarName)],
	treeProgram.programId
);




//fruitBalance, 
export let [fruitBalance] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('fruit'),tree.toBuffer()],
	treeProgram.programId
);

//requiredNutrients
export let [requiredNutrients] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('requirednutrients'), tree.toBuffer()],
	treeProgram.programId
);

// waterBalance,
export let [waterBalance] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('water'), inputBalance.toBuffer()],
	treeProgram.programId
);

// 	nitrogenBalance,
export let [nitrogenBalance] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('nitrogen'), inputBalance.toBuffer()],
	treeProgram.programId
);
// 	phosphorusBalance,
export let [phosphorusBalance] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('phosphorus'), inputBalance.toBuffer()],
	treeProgram.programId
);
// 	potassiumBalance,
export let [potassiumBalance] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('potassium'), inputBalance.toBuffer()],
	treeProgram.programId
);

// // fruitMarket

export let[fruitMarket] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('fruitmarket'), Buffer.from(cultivarName)],
	fruitMarketProgram.programId
);

// // fruitVaultAuthority,
// export let [fruitVaultAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
// 	[Buffer.from('fruitvaultauthority')],
// 	fruitMarketProgram.programId
// );

// 	fruitVault,
export let [fruitVault] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('fruit'), Buffer.from(cultivarName)],
	farmerProgram.programId
);

// 	marketEntry,
export let [marketEntry] = anchor.web3.PublicKey.findProgramAddressSync(
	[
		Buffer.from('marketentry'),
		fruitMarket.toBuffer(),
		payer.publicKey.toBuffer(),
	],
	fruitMarketProgram.programId
);


export let [entryFruitBalance] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('fruit'), marketEntry.toBuffer()],
	fruitMarketProgram.programId
);

export 	let [seedsAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('seedsauthority'), payer.publicKey.toBuffer()],
	treeProgram.programId
);

export 	let [seedsBalance] = anchor.web3.PublicKey.findProgramAddressSync(
	[
		Buffer.from('seedsbalance'),
		seedsAuthority.toBuffer(),
		Buffer.from(cultivarName),
	],
	treeProgram.programId
);

export 	let [marketAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('marketauthority')],
	fruitMarketProgram.programId
);





