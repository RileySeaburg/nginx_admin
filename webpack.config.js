const path = require('path');

module.exports = {
    mode: 'development',
    entry: './client/index.js',
    output: {
        filename: 'index.js',
        path: path.resolve(__dirname, 'static/js')
    },
    module: {
        
        rules: [
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader'],
                test: /\.(js|jsx)$/,
                exclude: /node_modules/,
                use: ['babel-loader'],
            },
        ],
    },
};
