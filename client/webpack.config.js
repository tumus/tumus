module.exports = {
    entry: './main.js',
    output: {
        filename: 'build/bundle.js'
    },
    module: {
        loaders: [
            test: /\.js$/,
            loader: 'babel-loader',
            query: {
                presets: ['es2015', 'react']
            }
        ]
    }
};
