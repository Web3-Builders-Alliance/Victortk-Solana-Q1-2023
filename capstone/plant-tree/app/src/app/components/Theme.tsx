'use client';
import React, { FC, ReactNode } from 'react';
import {
	createTheme,
	ThemeProvider,
	responsiveFontSizes,
} from '@mui/material/styles';
import Container from '@mui/material/Container';

declare module '@mui/material/styles' {
	interface Theme {
		background: {
			bg: string;
		};
	}
	// allow configuration using `createTheme`
	interface ThemeOptions {
		background?: {
			bg?: string;
		};
	}
}

let theme = createTheme({
	background: {
		bg: '#00C9EE',
	},
	palette: {
		primary: {
			main: '#402E32',
		},
		secondary: {
			main: '#FED668',
		},
	},
});
// "#F34213"
theme = responsiveFontSizes(theme);

const Theme: FC<{ children: ReactNode }> = ({ children }) => {
	return (
		<ThemeProvider theme={theme}>
			<Container maxWidth='xl' sx={{ 'overflowX': 'hidden' }}>
				{children}
			</Container>
		</ThemeProvider>
	);
};

export default Theme;
