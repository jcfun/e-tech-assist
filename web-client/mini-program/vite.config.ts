import { defineConfig } from "vite"
import uni from "@dcloudio/vite-plugin-uni"
import * as path from 'path'
import commonjs from '@rollup/plugin-commonjs'

export default defineConfig({
  resolve: {
    alias: {
      '@': `${path.resolve(__dirname, 'src')}/`,
    },
    extensions: ['.mjs', '.js', '.jsx', '.json', '.vue'],
  },
  plugins: [
    uni(),
    commonjs()
  ],
  server: {
    host: "::",
    port: 8003
  }
})

module.exports = {
  transpileDependencies: ['uview-plus']
}