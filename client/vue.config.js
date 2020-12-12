const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
    chainWebpack: config => {
        config.module
            .rule("raw")
            .test(/\.txt$/)
            .use("raw-loader")
            .loader("raw-loader")
            .end();
    },

    configureWebpack: {
        plugins: [
            // vue-cli is still copying public/login.html into dist but here we
            // are overwriting it.
            new HtmlWebpackPlugin({
                title: "Chat",
                template: "./public/login.html",
                filename: "login.html",
                chunks: [],
                // Simply setting this to true doesn't seem to do anything.
                // https://github.com/jantimon/html-webpack-plugin/issues/1094
                minify: {
                    collapseWhitespace: true,
                    removeComments: true,
                    removeRedundantAttributes: true,
                    removeScriptTypeAttributes: true,
                    removeStyleLinkTypeAttributes: true,
                    useShortDoctype: true
                }
            })
        ]
    },

    pages: {
        channel: {
            title: "Chat",
            entry: "./src/pages/channel/main.js",
            template: "./public/channel.html",
            filename: "channel.html",
            chunks: [ "chunk-vendors", "chunk-common", "channel" ]
        }
    }
};
