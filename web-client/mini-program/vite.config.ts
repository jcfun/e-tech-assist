import { defineConfig } from 'vite';
import uni from '@dcloudio/vite-plugin-uni';
import * as path from 'path';
import commonjs from '@rollup/plugin-commonjs';
import eslintPlugin from 'vite-plugin-eslint';

export default defineConfig({
  resolve: {
    alias: {
      '@': `${path.resolve(__dirname, 'src')}/`,
    },
    extensions: ['.mjs', '.js', '.ts', '.jsx', '.tsx', '.json', '.vue'],
  },
  plugins: [uni(), eslintPlugin(), commonjs()],
  server: {
    host: '::',
    port: 8003,
  },
});

// module.exports = {
//   transpileDependencies: ['uview-plus']
// }
