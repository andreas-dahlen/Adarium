import type { OxlintConfig } from 'oxlint'
import { fileURLToPath } from 'node:url'

const pluginPath = (path: string) =>
  fileURLToPath(new URL(path, import.meta.url))

export const jsPlugins: OxlintConfig['jsPlugins'] = [
  {
    name: 'test-api',
    specifier: pluginPath('../oxlint-plugins/testApi-ox/no-test-only-api-plugin.ts'),
  },
  {
    name: 'internal-imports',
    specifier: pluginPath('../oxlint-plugins/internalImports-ox/no-internal-import-extensions-plugin.ts'),
  },
  {
    name: 'existing-imports',
    specifier: pluginPath('../oxlint-plugins/importExists-ox/no-nonexistent-import-extensions-plugin.ts'),
  },
]