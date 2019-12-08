const express = require('express');
const app = express();

express.static.mime.define({'application/wasm': ['wasm']});

app.use(express.static('./'));

app.get('/', (req, res) => res.sendFile('index.html'))

app.listen(3000);
