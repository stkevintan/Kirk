const path = require('path')
const webpack = require('webpack')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')
const HtmlWebPackPlugin = require('html-webpack-plugin')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')
const { InjectManifest } = require('workbox-webpack-plugin')
const CopyPlugin = require('copy-webpack-plugin')

const createCssLoader = (mode = 'development') => {
  return /development|staging/.test(mode)
    ? ['style-loader', 'css-loader']
    : [
        {
          loader: MiniCssExtractPlugin.loader,
          options: {
            hmr: false
          }
        },
        { loader: 'css-loader', options: { importLoaders: 1 } },
        {
          loader: 'postcss-loader',
          options: {
            ident: 'postcss',
            plugins: loader => [require('cssnano')()]
          }
        }
      ]
}
module.exports = ({ mode } = { mode: 'development' }) => {
  // console.log('mode,', mode)
  return {
    entry: './web/bootstrap',
    output: {
      filename: '[name].[contenthash:8].js',
      chunkFilename: '[name].[contenthash:8].async.js',
      publicPath: '/',
      path: path.resolve(__dirname, 'public')
    },
    mode,
    module: {
      rules: [
        { test: /\.(js|ts)x?$/, loader: 'ts-loader' },
        {
          test: /\.less$/,
          use: [
            ...createCssLoader(mode),
            {
              loader: 'less-loader',
              options: {
                noIeCompat: true,
                javascriptEnabled: true
              }
            }
          ]
        },
        {
          test: /\.css$/,
          use: [...createCssLoader(mode)]
        },
        {
          test: /\.(png|jpg|bmp|ttf|svg|woff|eot)$/,
          use: [
            {
              loader: 'file-loader',
              options: {
                emitFile: true
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
      new webpack.DefinePlugin({
        BUILD_TIME: Date.now()
      }),
      new HtmlWebPackPlugin({
        inject: true,
        template: './web/index.ejs'
      }),
      new CopyPlugin([
        {
          from: 'web/static',
          to: 'static'
        }
      ]),
      new MiniCssExtractPlugin({
        filename: '[name].[contenthash:8].css',
        chunkFilename: '[id].[contenthash:8].css'
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, '.')
      }),
      new InjectManifest({
        importWorkboxFrom: 'local',
        swSrc: path.join('web', 'sw.js'),
        swDest: 'service-worker.js',
        include: [/\.(js|css|wasm)$/, /\.(svg|png|jpg)$/],
        exclude: [/\.html$/]
      })
    ],
    ...getExtraConf(mode)
  }
}

const getExtraConf = (mode = 'development') => {
  switch (mode) {
    case 'development':
      return {
        devtool: false,
        output: {
          publicPath: '/',
          filename: '[name].[contenthash:8].js',
          chunkFilename: '[name].[contenthash:8].async.js'
        },
        devServer: {
          publicPath: '/',
          host: '127.0.0.1',
          port: 3001,
          // historyApiFallback: true,
          contentBase: './dist'
        }
      }
    default:
      return {}
  }
}
