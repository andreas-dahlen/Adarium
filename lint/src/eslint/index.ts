import { defineConfig, globalIgnores } from 'eslint/config'
import { ignores } from '../globalIgnores.ts'
import base from './base.config.ts'
import { json } from './json.ts'
import { local } from './local.ts'
import { plugins } from './plugins.ts'
import { unicorn } from './unicorn.ts'
import oxlint from 'eslint-plugin-oxlint'


export default defineConfig([
  globalIgnores(ignores),
  base,
  plugins,
  ...json,
  local,
  ...unicorn,
  ...oxlint.configs['flat/recommended']
])
