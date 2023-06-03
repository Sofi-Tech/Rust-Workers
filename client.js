import { setTimeout as sleep } from 'timers/promises';
import { Client } from '@sofidev/ipc';

const name = process.argv[2];

const client = new Client(name, {
  maximumRetries: Infinity,
  retryTime: 2_000,
});

client.on('ready', () => {
  console.log(`${name}: Ready`);
});

client.on('connecting', () => {
  console.log(`${name}: Connecting`);
});

client.on('message', (message) => {
  console.log(`Received message at ${name}:`, message.data);
});

client.on('disconnect', () => {
  console.log('Connection lost');
});

await client.connectTo(3000, '127.0.0.1');
console.log(`${name}: Connected to Rust server`);

await sleep(5000);

for (const _ of Array(20).keys()) {
  // Send a message to the Rust server
  const message = { payload: `Hello from ${name}` };
  console.log(`Sending message from ${name}:`, message);

  const res = await client.sendTo('server', message);

  console.log(`Received response at ${name}:`, res);

  await sleep(1000);
}
