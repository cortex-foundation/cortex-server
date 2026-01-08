import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';
import eslintPluginPrettierRecommended from 'eslint-plugin-prettier/recommended';
import globals from 'globals';

/**
 * @param {import('typescript-eslint').Config} userConfig
 * @returns {import('typescript-eslint').Config}
 */
export const config = (...userConfig) => {
  return tseslint.config(
    eslint.configs.recommended,
    ...tseslint.configs.recommended,
    eslintPluginPrettierRecommended,
    {
       languageOptions: {
         globals: {
           ...globals.node,
         },
       },
    },
    ...userConfig
  );
};
