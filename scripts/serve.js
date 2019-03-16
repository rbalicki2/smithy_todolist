import WebpackDevServer from 'webpack-dev-server';
import webpack from 'webpack';
import webpackConfig from '../webpack.config.babel';

const options = {
  contentBase: './dist',
  host: 'localhost',
};

WebpackDevServer.addDevServerEntrypoints(webpackConfig, options);
const compiler = webpack(webpackConfig);
const server = new WebpackDevServer(compiler, options);

server.listen(8080, 'localhost');
