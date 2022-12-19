const { defineConfig } = require('@vue/cli-service')
module.exports = defineConfig({
  publicPath: '/',
  devServer: {
    proxy: {
      '^/api/': {
        target: 'http://localhost:8010/'
      }
    }
  },
  transpileDependencies: true
})
