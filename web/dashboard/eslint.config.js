import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';
import svelte from 'eslint-plugin-svelte';
import prettier from 'eslint-config-prettier';
import globals from 'globals';

export default tseslint.config(
	eslint.configs.recommended,
	...tseslint.configs.recommended,
	...svelte.configs['flat/recommended'],
	prettier,
	...svelte.configs['flat/prettier'],
	{
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.node
			}
		}
	},
	{
		files: ['**/*.svelte'],
		languageOptions: {
			parserOptions: {
				parser: tseslint.parser
			}
		},
		rules: {
			// Static SPA with simple href links; resolveRoute() adds no value here
			'svelte/no-navigation-without-resolve': 'off',
			// URLSearchParams used in non-reactive function scope; SvelteURLSearchParams not needed
			'svelte/prefer-svelte-reactivity': 'off'
		}
	},
	{
		ignores: ['build/', '.svelte-kit/', 'dist/']
	}
);
