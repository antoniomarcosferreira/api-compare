const express = require('express');
const bodyParser = require('body-parser');
const cors = require('cors');
const { log, ExpressAPILogMiddleware } = require('@rama41222/node-logger');

const {	Worker } = require("worker_threads");


const config = {
    name: 'sample-express-app',
    port: 7000,
    host: '0.0.0.0',
};
 

const app = express();
const logger = log({ console: true, file: false, label: config.name });

app.use(bodyParser.json());
app.use(cors());
app.use(ExpressAPILogMiddleware(logger, { request: true }));


app.get('/sleep100', async (req, res) => { 
  logger.info('inicio');
  await sleep(100);
  logger.info('retorn');
  res.status(200).send(`Node 5*5 = ${5*5}`);
});

app.get('/', (req, res) => {
    res.status(200).send('Welcome Nodejs');
});

app.get('/inc', async (req, res) => { 

  balance = 0;
  for(i=0; i < 50; i++){  
    balance += await (async () => {
      await sleep(100); 
      console.log("increment ", balance);
      return 1;
    })();
  }
  
  res.send(`Node balance: ${balance}`)
}) 

const sleep = (waitTimeInMs) => new Promise(resolve => setTimeout(resolve, waitTimeInMs));


app.listen(config.port, config.host, (e)=> {
    if(e) {
        throw new Error('Internal Server Error');
    }
    logger.info(`${config.name} running on ${config.host}:${config.port}`);
});
