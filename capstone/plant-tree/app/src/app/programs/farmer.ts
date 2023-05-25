export type Farmer = {
	version: '0.1.0';
	name: 'farmer';
	instructions: [
		{
			name: 'initializeFarmer';
			accounts: [
				{
					name: 'payer';
					isMut: true;
					isSigner: true;
				},
				{
					name: 'farmer';
					isMut: true;
					isSigner: false;
				},
				{
					name: 'systemProgram';
					isMut: false;
					isSigner: false;
				}
			];
			args: [
				{
					name: 'userName';
					type: 'string';
				}
			];
		}
	];
	accounts: [
		{
			name: 'farmer';
			type: {
				kind: 'struct';
				fields: [
					{
						name: 'name';
						type: 'string';
					},
					{
						name: 'address';
						type: 'publicKey';
					},
					{
						name: 'landCount';
						type: 'u64';
					},
					{
						name: 'treeCount';
						type: 'u64';
					}
				];
			};
		}
	];
};

export const IDL: Farmer = {
	version: '0.1.0',
	name: 'farmer',
	instructions: [
		{
			name: 'initializeFarmer',
			accounts: [
				{
					name: 'payer',
					isMut: true,
					isSigner: true,
				},
				{
					name: 'farmer',
					isMut: true,
					isSigner: false,
				},
				{
					name: 'systemProgram',
					isMut: false,
					isSigner: false,
				},
			],
			args: [
				{
					name: 'userName',
					type: 'string',
				},
			],
		},
	],
	accounts: [
		{
			name: 'farmer',
			type: {
				kind: 'struct',
				fields: [
					{
						name: 'name',
						type: 'string',
					},
					{
						name: 'address',
						type: 'publicKey',
					},
					{
						name: 'landCount',
						type: 'u64',
					},
					{
						name: 'treeCount',
						type: 'u64',
					},
				],
			},
		},
	],
};
