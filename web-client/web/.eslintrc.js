module.exports = {
  env: {
    browser: true,
    es2021: true,
    node: true,
  },
  extends: [
    'eslint:recommended', // 使用eslint的检查
    'plugin:vue/vue3-essential', // vue3
    'plugin:@typescript-eslint/recommended', // typescript
    'plugin:prettier/recommended', // 快捷配置，详情：https://github.com/prettier/eslint-plugin-prettier
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
    // quotes: ['error', 'single'], //必须为单引号
    // semi: ['error', 'always'], //必须有分号
    'vue/multi-word-component-names': 'off', // 取消组件大驼峰命名限制
    // 'max-len': ['error', { code: 120 }], // 每行最大字符数
    '@typescript-eslint/no-unused-vars': ['warn', { varsIgnorePattern: '^_', argsIgnorePattern: '^_' }], // 忽略以_开头的变量
    // '@typescript-eslint/no-explicit-any': 'off', // 取消any类型限制
    '@typescript-eslint/no-empty-function': 'off', // 取消空函数限制
    // '@typescript-eslint/no-var-requires': 'off', // 取消require限制
    // '@typescript-eslint/explicit-module-boundary-types': 'off', // 取消返回值类型限制
    '@typescript-eslint/no-non-null-assertion': 'off', // 取消非空断言限制
  },
  globals: {
    uni: true,
    wx: true,
  },
};
