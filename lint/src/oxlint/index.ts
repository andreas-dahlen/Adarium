import { defineConfig } from 'oxlint'
import { ignores } from '../globalIgnores.ts'
import { unusedVars } from './unusedVars.ts'
import { jsPlugins } from './plugins.ts'
import { typeAwareRules } from './typeAwareRules.ts'
export default defineConfig({
  ignorePatterns: ignores,
  jsPlugins,

  overrides: [
    unusedVars
  ],
  options: { "typeAware": true },

  plugins: [
    'eslint',
    'typescript',
    'unicorn',
    'oxc',
    'react',
    'import'
  ],

  rules: {
    'test-api/no-test-only-api': 'error',
    'existing-imports/no-nonexistent-import-extensions': 'error',
    'internal-imports/no-internal-import-extensions': 'error',
    'react/static-components': 'error',
    'react/use-memo': 'error',
    'react/preserve-manual-memoization': 'error',
    'react/incompatible-library': 'error',
    'react/immutability': 'error',
    'react/globals': 'error',
    'react/refs': 'error',
    'react/set-state-in-effect': 'error',
    'react/error-boundaries': 'error',
    'react/purity': 'error',
    'react/set-state-in-render': 'error',

    'import/extensions': [
      'error',
      'always',
      {
        js: 'never',
        jsx: 'never',
        ignorePackages: true,
        checkTypeImports: true,
      }
    ],


    ...typeAwareRules,
  }
})
