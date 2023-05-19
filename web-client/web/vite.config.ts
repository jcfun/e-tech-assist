import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import vueJsx from '@vitejs/plugin-vue-jsx';
import viteWasm from 'vite-plugin-wasm';
import { fileURLToPath, URL } from 'node:url';
import { prismjsPlugin } from 'vite-plugin-prismjs';

export default defineConfig({
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
    extensions: ['.mjs', '.js', '.ts', '.jsx', '.tsx', '.json', '.vue', '.d.ts'],
  },
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
  server: {
    host: '::',
    port: 8002,
  },
});
