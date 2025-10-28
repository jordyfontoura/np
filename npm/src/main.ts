#!/usr/bin/env node

import * as process from 'process';
import { executeRaw } from './command-executor';
import { resolvePackageManager } from './cli';

/**
 * Main entry point for the np CLI
 */
async function main(): Promise<void> {
  const cwd = process.cwd();

  // Pure proxy: capture everything after the binary and forward as-is
  const raw = process.argv.slice(2);

  if (raw.length === 0) {
    console.log(
      "Use 'np <command>' to forward to the detected package manager. Ex.: 'np -v', 'np add axios'."
    );
    return;
  }

  const managerResult = await resolvePackageManager(cwd);

  if (!managerResult.success) {
    console.error(`Error: ${managerResult.reason}`);
    process.exit(1);
  }

  const executeResult = await executeRaw(managerResult.data, raw);

  if (!executeResult.success) {
    console.error(executeResult.reason);
    process.exit(1);
  }
}

main().catch((error) => {
  console.error('Unexpected error:', error);
  process.exit(1);
});

