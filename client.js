import { setTimeout as sleep } from 'timers/promises';
import { Client } from '@sofidev/ipc';
import { Timer } from './Timer.js';

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

// client.on('raw', (message) => {
//   console.log(thing);
//   console.log(`Received raw message at ${name}:`, message);
//   timer.timeEnd(thing, true, (t) => `Time taken: ${t}`);
// });

client.on('disconnect', () => {
  console.log('Connection lost');
});

await client.connectTo(3000, '127.0.0.1');
console.log(`${name}: Connected to Rust server`);

await sleep(5000);
const timer = new Timer();
for (let i = 0; i < 100; i++) {
  // Send a message to the Rust server
  const message = { payload: `ping` };

  let thing = timer.time();
  const msg = await client.sendTo('Sofi', message, { receptive: true });
  timer.timeEnd(thing, true, (t) => `Time taken: ${t}`);
  console.log(msg);
  // await sleep(1000);
}
