const { setTimeout: sleep } = require('timers/promises');
const net = require('net');
const { randomUUID } = require('crypto');

const name = process.argv[2];

const promiseMap = new Map();

const client = new net.Socket();
client.connect(3000, '127.0.0.1', async () => {
  console.log(`${name}: Connected to Rust server`);

  for (const i of Array(20).keys()) {
    const id = randomUUID();

    // Send a message to the Rust server
    const message = { payload: `Hello from ${name}` };
    const messageData = JSON.stringify({ data: message, id });
    client.write(messageData);

    const promise = new Promise((resolve) => {
      promiseMap.set(id, resolve);
    });

    const response = await promise;

    console.log(`Received response at ${name} for ${id}:`, response);

    await sleep(1000);
  }
});

client.on('data', (_data) => {
  // Process the received data
  const message = JSON.parse(_data);
  console.log(`Received message at ${name}:`, message);

  const { id, data } = message;

  if (promiseMap.has(id)) {
    const resolve = promiseMap.get(id);
    resolve(data);
    promiseMap.delete(id);
  }
});

client.on('close', () => {
  console.log('Connection closed');
});
