import { warn, debug, trace, info, error  } from '@tauri-apps/plugin-log';

function forwardConsole(
  fnName,
  logger
) {
  const original = console[fnName];
  console[fnName] = (message) => {
    original(message);
    logger(message);
  };
}

export function forwardAll() {
    forwardConsole('log', trace);
    forwardConsole('debug', debug);
    forwardConsole('info', info);
    //forwardConsole('warn', warn);
    forwardConsole('error', error);
}

