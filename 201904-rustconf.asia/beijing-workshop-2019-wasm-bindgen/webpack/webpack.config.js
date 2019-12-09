const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');

module.exports = {
    mode: 'development',
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: 'index.html'
        }),
        // Polyfill for Edge
        // For production use polyfill from MDN instead
        // https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder#Polyfill
        // because ProvidePlugin overrides TextDecoder and TextEncoder
        // in browsers which already support them, and it leads
        // to almost 100x worse performance while working with strings
        new webpack.ProvidePlugin({
          TextDecoder: ['text-encoding', 'TextDecoder'],
          TextEncoder: ['text-encoding', 'TextEncoder'],
        })
    ],
};
