module.exports = {
  env: {
    browser: true,
    es2021: true,
    node: true,
  },
  extends: [
    // 'eslint:recommended',
    'prettier/@typescript-eslint',
    // 'plugin:vue/vue3-essential',
    // 'plugin:@typescript-eslint/recommended',
    'plugin:prettier/recommended',
  ],
  overrides: [],
  parser: 'vue-eslint-parser',
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    parser: '@typescript-eslint/parser',
  },
  plugins: ['vue', '@typescript-eslint'],
  rules: {
    quotes: ['error', 'single'], //必须为单引号
    semi: ['error', 'always'], //必须有分号
    'vue/multi-word-component-names': 'off',
    'prettier/prettier': 'error',
  },
  globals: {
    uni: true,
    wx: true,
  },
};
