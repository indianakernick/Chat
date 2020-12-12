module.exports = {
    chainWebpack: config => {
        config.module
            .rule("raw")
            .test(/\.txt$/)
            .use("raw-loader")
            .loader("raw-loader")
            .end();
    },

    pages: {
        channel: {
            title: "Chat",
            entry: "./src/pages/channel/main.js",
            template: "./public/channel.html",
            filename: "channel.html",
            chunks: [ "chunk-vendors", "chunk-common", "channel" ]
        },
        login: {
            title: "Chat",
            entry: "./src/pages/login/main.js",
            template: "./public/login.html",
            filename: "login.html",
            chunks: [ "chunk-vendors", "chunk-common", "login" ]
        }
    }
};
