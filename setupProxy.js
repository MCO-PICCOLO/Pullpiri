//setupProxy.js
const { createProxyMiddleware } = require('http-proxy-middleware');

module.exports = function (app) {
  app.use(
    createProxyMiddleware('/api', {
      target: 'http://10.157.19.188:5000',
      changeOrigin: true,
    }),
  );
  
  
};
