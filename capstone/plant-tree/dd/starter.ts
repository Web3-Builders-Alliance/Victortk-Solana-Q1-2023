



	



	


	it('Updates the tree Again', async () => {
		// let fm = await token.getMint(provider.connection, fruitMint);

		// let fruitBalance = await token.getOrCreateAssociatedTokenAccount(
		// 	provider.connection,
		// 	payer.payer,
		// 	fruitMint,
		// 	payer.publicKey
		// );

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
			})
			.rpc();
		let pb = await token.getAccount(provider.connection, fruitBalance);
		let t = await program.account.tree.fetch(tree);

		console.log('Checks and updates the tree data: ', tx.toString());

		console.log('balance ', pb.amount);
	});

});



	







	

	// it ("Initializes market",async () =>  {

	// 	const tx = await program.methods
	// 		.initializeMarket()
	// 		.accounts({
	// 			fruitMintAuthority,
	// 			fruitMint,
	// 			fruitVault,
	// 			fruitMarket,
	// 			marketEntry,											
	// 			entryFruitBalance,								
	// 		})	
	// 		.rpc();

	// })

	




// const getReturnLog = (confirmedTransaction) => {
// 	const prefix = "Program return: " ;
//   let log = confirmedTransaction.meta.logMessages.find((log) =>
// 		log.startsWith(prefix)
// 	);
// 	log = log.slice(prefix.length);
// 	const [key,data] = log.split(" ", 2) ;
// 	const buffer = Buffer.from(data,"base64") ;
// 	return [key,data,buffer];
// };
