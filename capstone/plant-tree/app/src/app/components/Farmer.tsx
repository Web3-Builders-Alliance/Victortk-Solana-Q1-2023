import React, { useState, useEffect } from 'react';
import * as anchor from '@project-serum/anchor';
import { Box, Button, Card, Stack, Typography, Grid } from '@mui/material';
import styles from './styles/farmer.module.css';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { useAnchorWallet } from '@solana/wallet-adapter-react';
import { Program, Wallet, AnchorProvider } from '@project-serum/anchor';
import { TreeProgram, IDL } from '../../../public/programs/tree_program';
import Actions from './Actions';

const Farmer = (props: {
	name: String;
	landCount: anchor.BN;
	treeCount: anchor.BN;
	cultivarName: String | null;
}) => {
	// const [data, setData] = useState<{
	// 	payer: PublicKey;
	// 	farmer: PublicKey;
	// } | null>(null);
	// const w = useAnchorWallet();

	// const connection = new Connection('https://api.devnet.solana.com');

	// const provider = new AnchorProvider(connection, w as Wallet, {
	// 	commitment: 'confirmed',
	// });
	// const farmProgram = new PublicKey(
	// 	'6DDP3hohHprxPNUWVtwpK89QAzcB27Fk4NSCgcq368P6'
	// );

	// // const farmerProgram = new PublicKey(
	// // 	'5LJq1WKXV2bdgsosp6wk2pgvk1Rhc75ffRLRXGZvPQWU'
	// // );

	// const farmerProgram = new PublicKey(
	// 	'FEa3hjWEQEmuUgZtDQ1btp1Y2EKVhChqCzADTenewCsF'
	// );

	// const programID = new PublicKey(
	// 	'EfYywm823JAajvTAHFv7wnKGi8M4R7BwqufaUEECxUxG'
	// );

	// const program = new Program(IDL, programID, provider);
	// let payer = program.provider;

	// useEffect(() => {

	// 	(async () => {
	// 		let ct: any = await program.account.cultivar.all([
	// 			{
	// 				memcmp:
	// 			}
	// 		]);

	// 		console.log('Cultivars', ct);

	// 		if (ct) {
	// 			ct.map((ctvr: any) => {
	// 				// let c: cultivar = ctvr.account;
	// 				// // console.log('c is ', c);
	// 				// let c2: cultivar[] = cultivars;
	// 				// let pubkey = c.creator;
	// 				// console.log(pubkey);
	// 				// if (c.name) {
	// 				// 	c2.push(c);
	// 				// }
	// 				// setCultivars(c2);
	// 			});
	// 			// console.log('Cultivars is now', cultivars);
	// 			// setReady(!ready);
	// 		}
	// 	})();
	// }, []);

	return (
		<Box className={styles.box}>
			  <div className={styles.content}>
				  <div className={styles.imgSection}>
					    <div className={styles.image}> </div>
				  </div>
				  <Card className={styles.card}>
					<Typography
						variant='h4'
						color='text.secondary'
						className={styles.text1}
					>
						Welcome {props.name}
					</Typography>
					<Typography
						className={styles.text2}
						variant='body1'
						color='secondary.main'
					>
						You have {props.landCount.toString()} pieces of land
					</Typography>
					<Typography
						color='secondary.main'
						className={styles.text2}
						variant='body1'
					>
						You have planted {props.treeCount.toString()} trees
					</Typography>
				  </Card>
				</div>
				<Card className={styles.dv}>
					<Actions cultivarName={props.cultivarName} />
				</Card>			
		</Box>
	);
};

export default Farmer;
