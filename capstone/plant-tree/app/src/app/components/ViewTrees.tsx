'use client';
import React, { useState } from 'react';
import {
	Box,
	Button,
	Card,
	Grid,
	Typography,
	CardActionArea,
} from '@mui/material';
// import usePdas from '../hooks/usePda';qqqq
import { Farmer, IDL } from '../../../public/programs/farmer';
import styles from './styles/viewTrees.module.css';
import * as anchor from '@project-serum/anchor';
import { useAnchorWallet } from '@solana/wallet-adapter-react';
import { Program, Wallet, AnchorProvider } from '@project-serum/anchor';
import { Connection, PublicKey } from '@solana/web3.js';
import {Link}  from '@mui/material';
import  NextLink from 'next/link';

const ViewTrees = () => {
	return (
		<Card className={styles.card}>
			<Link href='/view-trees' component={NextLink} underline='none'>
				<CardActionArea className={styles.card}>
					<div className={styles.outter}></div>
					<Typography variant='h3' className={styles.type}>
						View my trees
					</Typography>
				</CardActionArea>
			</Link>
		</Card>
	);
};

export default ViewTrees;
