import { fileURLToPath, URL } from 'node:url';

import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import vueJsx from '@vitejs/plugin-vue-jsx';
import viteWasm from 'vite-plugin-wasm';
import { prismjsPlugin } from 'vite-plugin-prismjs';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueJsx(),
    viteWasm(),
    prismjsPlugin({
      languages: 'all', // 语言
      plugins: ['line-numbers', 'show-language', 'copy-to-clipboard', 'inline-color'],
      theme: 'okaidia', // 主题
      css: true,
    }),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
    // extensions: ['.mjs', '.js', '.ts', '.jsx', '.tsx', '.json', '.vue', '.less', '.sass'],
  },
  css: {
    preprocessorOptions: {
      less: {
        additionalData: `@import "src/styles/variables.less";`,
        modifyVars: {},
        javascriptEnabled: true,
      },
    },
  },
  server: {
    open: false,
    host: '::',
    port: 8001,
  },
});
