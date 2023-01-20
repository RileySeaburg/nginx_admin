const path = require('path');

module.exports = {
    entry: './static/js/index.js',
    output: {
        filename: 'main.js',
        path: path.resolve(__dirname, 'static/dist')
    },
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: ['style-loader', 'css-loader'],
            },
        ],
    },
};
