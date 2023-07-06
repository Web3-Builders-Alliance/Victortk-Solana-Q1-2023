import React, { useState } from 'react';
import * as anchor from '@project-serum/anchor';
import { TreeProgram, IDL } from '../../../public/programs/tree_program';
import { Connection, PublicKey } from '@solana/web3.js';
import {
	Box,
	Button,
	Card,
	Stack,
	Typography,
	Grid,
	CardActionArea,
	Link,
} from '@mui/material';
import styles from './styles/buytree.module.css';
import NextLink from 'next/link';
import { useAnchorWallet } from '@solana/wallet-adapter-react';
import { Program, Wallet, AnchorProvider } from '@project-serum/anchor';

type Nutrient =
	| 'consumeNitrogen'
	| 'consumePhosphorus'
	| 'consumePotassium';

const ConsumeNutrients = (props: {
	cultivarName: String;
	nutrient: Nutrient;
}) => {
	const w = useAnchorWallet();
	// const { connection } = useConnection();

	const connection = new Connection('https://api.devnet.solana.com');

	const provider = new AnchorProvider(connection, w as Wallet, {
		commitment: 'confirmed',
	});

	const farmerProgram = new PublicKey(
		'3pEgxEH8RhxKtdx3qsvcmrZQUMxeyQisiiBAJ52FmtMx'
	);

	const farmProgram = new PublicKey(
		'CrYtrU5xK6S98iGQVnyag1XKG9vSYzw2M3Mq4JNHLGSA'
	);

	const programID = new PublicKey(
		'CUJ8TCeGSKKhqYtZYiBZRghTJvRRRpm9qR2ykX91N1ns'
	);

	const program = new Program(IDL, programID, provider);
	let payer = program.provider;

	const handleClick = async () => {
		if (payer.publicKey) {
			console.log('Inside if satement');
			let [farm] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('farm')],
				farmProgram
			);
			// farmer
			let [farmer] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('farmer'), payer.publicKey.toBuffer()],
				farmerProgram
			);

			let [landMeta] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('landmeta'), farm.toBuffer()],
				farmProgram
			);

			let [landPiece] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('landpiece'), landMeta.toBuffer(), farmer.toBuffer()],
				farmerProgram
			);

			let [vault] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('carbonvault')],
				farmProgram
			);

			console.log('farm', farm.toString());

			// cultivar_meta
			let [cultivarMeta] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('cultivarmeta'), farm.toBuffer()],
				farmProgram
			);

			console.log('cultivarMeta', cultivarMeta.toString());

			let [cultivar] = anchor.web3.PublicKey.findProgramAddressSync(
				[
					Buffer.from('cultivar'),
					cultivarMeta.toBuffer(),
					Buffer.from(props.cultivarName),
				],
				program.programId
			);

			console.log('cultivar', cultivar.toString());

			let [fruitMint] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('fruitmint'), Buffer.from(props.cultivarName)],
				program.programId
			);
			console.log('fruitMint', fruitMint.toString());

			let [fruitMintAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('fruitmintauthority')],
				program.programId
			);

			// trees_meta
			let [treesMeta] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('treesmeta'), farm.toBuffer()],
				farmProgram
			);

			//tree
			let [tree] = anchor.web3.PublicKey.findProgramAddressSync(
				[
					Buffer.from('tree'),
					treesMeta.toBuffer(),
					farmer.toBuffer(),
					Buffer.from(props.cultivarName),
				],
				program.programId
			);
			let [inputBalance] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('nutrientbalance'), tree.toBuffer()],
				program.programId
			);

			//fruitBalance,
			let [fruitBalance] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('fruit'), tree.toBuffer()],
				program.programId
			);

			//requiredNutrients
			let [requiredNutrients] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('requirednutrients'), tree.toBuffer()],
				program.programId
			);
			// waterBalance,
			let [waterBalance] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('water'), inputBalance.toBuffer()],
				program.programId
			);

			// 	nitrogenBalance,
			let [nitrogenBalance] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('nitrogen'), inputBalance.toBuffer()],
				program.programId
			);
			// 	phosphorusBalance,
			let [phosphorusBalance] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('phosphorus'), inputBalance.toBuffer()],
				program.programId
			);
			// 	potassiumBalance,
			let [potassiumBalance] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('potassium'), inputBalance.toBuffer()],
				program.programId
			);

			// nitrogenMint,
			let [nitrogenMint] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('nitrogenmint')],
				program.programId
			);

			// potassiumMint,
			let [potassiumMint] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('potassiummint')],
				program.programId
			);
			//phosphorusMint,
			let [phosphorusMint] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('phosphorusmint')],
				program.programId
			);

			// water_mint
			let [waterMint] = anchor.web3.PublicKey.findProgramAddressSync(
				[Buffer.from('watermint')],
				program.programId
			);

			// nutrient_mint_authority

			let [nutrientMintAuthority] =
				anchor.web3.PublicKey.findProgramAddressSync(
					[Buffer.from('nutrientmintauthority')],
					program.programId
				);

			const tx = await program.methods[props.nutrient](new anchor.BN(201))
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
					farmProgram,
				})
				.rpc();
				
			console.log(`The transaction signature is ${tx.toString()}`);
			alert('success ' + tx);
			// setData({
			// 	farmer: farmer,
			// 	payer: payer.publicKey,
			// 	landPieces: landP.length,
			// });
		}
	};

	return <Button onClick={handleClick}>ConsumeNutrients</Button>;
};

export default ConsumeNutrients;
