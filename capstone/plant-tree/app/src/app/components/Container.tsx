'use client';
import React, { FC, ReactNode } from 'react';
import Connection from './Connection';
import Header from './Header';
import Theme from './Theme';

const Container: FC<{ children: ReactNode }> = ({ children }) => {
	return (
		<Connection>
			<Theme>
				<Header />
				{children}
			</Theme>
		</Connection>
	);
};

export default Container;
