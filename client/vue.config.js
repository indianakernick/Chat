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
        with_session: {
            title: "Chat",
            entry: "./src/pages/with_session/main.js",
            template: "./public/with_session.html",
            filename: "with_session.html",
            chunks: [ "chunk-vendors", "chunk-common", "with_session" ]
        },
        without_session: {
            title: "Chat",
            entry: "./src/pages/without_session/main.js",
            template: "./public/without_session.html",
            filename: "without_session.html",
            chunks: [ "chunk-vendors", "chunk-common", "without_session" ]
        }
    }
};
