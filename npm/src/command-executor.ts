import * as childProcess from 'child_process';
import { err, IResult, ok } from 'tryless';
import { PackageManagerInfo } from './managers';

/**
 * Executes a raw command with the package manager
 * @param managerInfo - Package manager to use
 * @param args - Command arguments
 * @returns Result indicating success or failure
 */
export async function executeRaw(
  managerInfo: PackageManagerInfo,
  args: string[]
): Promise<IResult<undefined, { 'command-execution-failed': string }>> {
  return new Promise((resolve) => {
    const bin = managerInfo.name;
    const cmd = childProcess.spawn(bin, args, {
      stdio: 'inherit',
    });

    cmd.on('close', (code) => {
      if (code !== 0) {
        resolve(err('command-execution-failed', `❌ Command failed with exit code ${code ?? -1}`));
      } else {
        resolve(ok());
      }
    });

    cmd.on('error', (error: Error) => {
      resolve(err('command-execution-failed', `Failed to execute ${bin}: ${error.message}`));
    });
  });
}

