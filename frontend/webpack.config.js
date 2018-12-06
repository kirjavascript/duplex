const webpack = require('webpack');
const StyleLintPlugin = require('stylelint-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

module.exports = (env={}, args={}) => {

    const config = {
        mode: env.dev ? 'development' : 'production',
        entry : {
            main: './app/main.js',
            styles: './styles/main.scss',
        },
        output: {
            path:     __dirname + '/../static/',
            filename: '[name].js',
        },
        module: {
            rules: [
                {
                    test: /\.jsx?$/,
                    exclude: env.dev ? /node_modules/ : void 0,
                    use: [
                        {
                            loader: 'babel-loader',
                            options: {
                                babelrc: false,
                                presets: [
                                    '@babel/preset-env',
                                    '@babel/preset-react',
                                ],
                                plugins: [
                                    'transform-class-properties',
                                    'transform-do-expressions',
                                    '@babel/plugin-syntax-dynamic-import',
                                ],
                            }
                        }
                    ],
                },
                {
                    test:  /\.s?[ac]ss$/,
                    use: [
                        MiniCssExtractPlugin.loader, // in lieu of style-loader
                        { loader:'css-loader', options: { url: false } },
                        { loader:'sass-loader' },
                        { loader:'import-glob-loader' },
                    ],
                },
                {
                    test:  /\.?worker\.js$/,
                    use: [
                        {
                            loader: 'worker-loader',
                            options: {
                                inline: true,
                                fallback: false,
                            }
                        },
                    ],
                },
            ],
        },
        plugins: [
            new webpack.DefinePlugin({
                __DEV__: env.dev
            }),
            new webpack.ProvidePlugin({
                React: 'react',
            }),
            new StyleLintPlugin({
                configFile: '.stylelintrc',
                syntax: 'scss',
            }),
            new MiniCssExtractPlugin({
                filename: "[name].css",
            }),
        ],
        resolve: {
            extensions: ['.js', '.json', '.jsx'],
            alias: {
                '#app': __dirname + '/app',
            },
        },
        devtool: env.dev && 'source-map',
        node: {
            fs: 'empty',
        },
    };

    if (env.dev) {
        // add linting
        config.module.rules.push({
            test: /\.js$/,
            enforce: 'pre',
            loader: 'eslint-loader',
            exclude: /node_modules|wasm32-unknown-unknown\/release\/duplex\.js$/,
            options: {
                configFile: '.eslintrc',
                failOnWarning: false,
                failOnError: false,
                emitError: false,
                fix: true
            }
        });
    }

    return config;
};
