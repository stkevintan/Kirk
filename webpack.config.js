const path = require('path')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')
const HtmlWebPackPlugin = require('html-webpack-plugin')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
  entry: './web/bootstrap',
  output: {
    filename: '[name].[contenthash:8].js',
    chunkFilename: '[name].[contenthash:8].async.js',
    path: path.resolve(__dirname, 'dist')
  },
  mode: 'development',
  module: {
    rules: [
      { test: /\.tsx?$/, loader: 'ts-loader' },
      {
        test: /\.less$/,
        use: [
          'style-loader',
          'css-loader',
          {
            loader: 'less-loader',
            options: {
              noIeCompat: true,
              javascriptEnabled: true
            }
          }
        ]
      }
    ]
  },
  resolve: {
    extensions: ['.wasm', '.ts', '.tsx', '.js', '.jsx', '.json', '.less'],
    alias: {
      web: path.resolve(__dirname, 'web'),
      pkg: path.resolve(__dirname, 'pkg')
    }
  },
  plugins: [
    new CleanWebpackPlugin({
      baseUrl: __dirname
    }),
    new HtmlWebPackPlugin({
      inject: true,
      template: './web/index.ejs'
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, '.')
    })
  ]
}
