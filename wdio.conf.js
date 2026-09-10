import { spawn } from 'node:child_process';
import net from 'node:net';
import { existsSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const repoRoot = path.dirname(fileURLToPath(import.meta.url));
const binaryName = process.platform === 'win32' ? 'rusty-shed.exe' : 'rusty-shed';
const defaultAppBinary = path.resolve(repoRoot, 'src-tauri', 'target', 'debug', binaryName);
const appBinary = process.env.TAURI_APP_BINARY_PATH
  ? path.resolve(process.env.TAURI_APP_BINARY_PATH)
  : defaultAppBinary;
const tauriDriverPort = Number(process.env.TAURI_DRIVER_PORT ?? 4444);
const nativeDriverPath =
  process.platform === 'linux'
    ? (process.env.TAURI_NATIVE_DRIVER_PATH ?? '/usr/bin/WebKitWebDriver')
    : undefined;

let tauriDriverProcess;

function waitForDriverReady(driverProcess, host, port, timeoutMs) {
  return new Promise((resolve, reject) => {
    const startedAt = Date.now();
    let settled = false;

    const cleanup = () => {
      driverProcess.off('error', onDriverError);
      driverProcess.off('exit', onDriverExit);
    };

    const finish = (error) => {
      if (settled) {
        return;
      }

      settled = true;
      cleanup();
      reject(error);
    };

    const onDriverError = (error) => {
      finish(error);
    };

    const onDriverExit = (code, signal) => {
      finish(
        new Error(
          `tauri-driver exited before it became ready (code=${code}, signal=${signal ?? 'none'})`
        )
      );
    };

    driverProcess.once('error', onDriverError);
    driverProcess.once('exit', onDriverExit);

    const tryConnect = () => {
      if (settled) {
        return;
      }

      const socket = net.createConnection({ host, port });

      socket.once('connect', () => {
        socket.end();
        settled = true;
        cleanup();
        resolve();
      });

      socket.once('error', (error) => {
        socket.destroy();

        if (Date.now() - startedAt >= timeoutMs) {
          finish(error);
          return;
        }

        setTimeout(tryConnect, 500);
      });
    };

    tryConnect();
  });
}

function tauriDriverArgs() {
  const args = ['--port', String(tauriDriverPort)];

  if (nativeDriverPath && existsSync(nativeDriverPath)) {
    args.push('--native-driver', nativeDriverPath);
  }

  return args;
}

export const config = {
  runner: 'local',
  specs: ['./test/e2e/**/*.e2e.js'],
  maxInstances: 1,
  hostname: '127.0.0.1',
  port: tauriDriverPort,
  path: '/',
  logLevel: 'info',
  automationProtocol: 'webdriver',
  waitforTimeout: 60_000,
  connectionRetryTimeout: 120_000,
  connectionRetryCount: 2,
  framework: 'mocha',
  reporters: ['spec'],
  mochaOpts: {
    ui: 'bdd',
    timeout: 120_000
  },
  capabilities: [
    {
      browserName: 'wry',
      'wdio:enforceWebDriverClassic': true,
      'tauri:options': {
        application: appBinary
      }
    }
  ],
  onPrepare: async () => {
    tauriDriverProcess = spawn('tauri-driver', tauriDriverArgs(), {
      stdio: 'inherit',
      env: process.env
    });

    await waitForDriverReady(tauriDriverProcess, '127.0.0.1', tauriDriverPort, 60_000);
  },
  onComplete: () => {
    tauriDriverProcess?.kill();
  }
};
