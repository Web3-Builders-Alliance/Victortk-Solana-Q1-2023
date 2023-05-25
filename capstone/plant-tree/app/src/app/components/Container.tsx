'use client';
import React, { FC, ReactNode } from 'react';
import Connection from './Connection';
import Header from './Header';
import Theme from './Theme';
import styles from "./container.module.css";
import { Box } from '@mui/material';

const Container: FC<{ children: ReactNode }> = ({ children }) => {
	return (
		<Connection >
			<Theme>	
				<Header />
				<Box className={styles.box}>		
				{children}	
				</Box>				
			</Theme>
		</Connection>
	);
};

export default Container;
