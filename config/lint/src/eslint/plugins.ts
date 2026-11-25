import type { Linter } from 'eslint'
import unicorn from 'eslint-plugin-unicorn'
import jsonSchemaValidator from 'eslint-plugin-json-schema-validator'

export const plugins: Linter.Config = {
  plugins: {
    unicorn,
    jsonSchemaValidator
  }
}