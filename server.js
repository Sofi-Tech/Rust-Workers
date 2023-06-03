const { setTimeout: sleep } = require('timers/promises');
const net = require('net');
const { randomUUID } = require('crypto');
const { pack, unpack } = require('msgpackr');
const { read, create } = require('veza');

const name = process.argv[2];

const promiseMap = new Map();

const client = new net.Socket();
client.connect(3000, '127.0.0.1', async () => {
  console.log(`${name}: Connected to Rust server`);

  for (const i of Array(20).keys()) {
    // Send a message to the Rust server
    const message = { payload: `Hello from ${name}` };
    const messageData = create(true, pack(message));
    const id = read(messageData).id;
    console.log(`Sending message at ${name} for ${id}:`, message);
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
  const { id } = read(_data);

  console.log({ id });

  const message = unpack(_data.subarray(11));
  console.log(`Received message at ${name}:`, message);

  if (promiseMap.has(id)) {
    const resolve = promiseMap.get(id);
    resolve(message);
    promiseMap.delete(id);
  }
});

client.on('close', () => {
  console.log('Connection closed');
});
