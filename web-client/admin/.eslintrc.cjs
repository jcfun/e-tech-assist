/* eslint-env node */
require('@rushstack/eslint-patch/modern-module-resolution');

module.exports = {
  env: {
    browser: true,
    node: true,
    es6: true,
  },
  root: true,
  extends: [
    'eslint:recommended', // 使用eslint的检查
    'plugin:vue/vue3-essential', // vue3
    'plugin:@typescript-eslint/recommended', // typescript
    'plugin:prettier/recommended', // 快捷配置，详情：https://github.com/prettier/eslint-plugin-prettier
  ],
  parser: 'vue-eslint-parser',
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    jsxPragma: 'React',
    ecmaFeatures: {
      jsx: true,
    },
    parser: '@typescript-eslint/parser',
  },
  plugins: ['vue', '@typescript-eslint'],
  rules: {
    'vue/multi-word-component-names': 'off', // 取消组件大驼峰命名限制
    '@typescript-eslint/no-unused-vars': ['warn', { varsIgnorePattern: '^_', argsIgnorePattern: '^_' }], // 忽略以_开头的变量
    '@typescript-eslint/no-non-null-assertion': 'off', // 允许非空断言
    '@typescript-eslint/no-explicit-any': 'off', // 允许any类型
    'vue/no-reserved-component-names': 'off', // 允许使用保留字命名组件
  },
};
