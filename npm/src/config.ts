import * as fs from 'fs';
import * as path from 'path';
import * as childProcess from 'child_process';
import { err, ok } from 'tryless';
import { PackageManagerInfo, getFromLockfile, getPackageManagerInfoByName, getPossibleLockfiles } from './managers';

/**
 * Reads package manager from package.json
 * @param cwd - Current working directory
 * @returns Package manager info if found in package.json
 */
export function readPackageManagerFromPackageJson(
  cwd: string
): PackageManagerInfo | undefined {
  const packageJsonPath = path.join(cwd, 'package.json');

  if (!fs.existsSync(packageJsonPath)) return undefined;

  try {
    const content = fs.readFileSync(packageJsonPath, 'utf-8');
    const pkg = JSON.parse(content);

    const pmStr = pkg.packageManager;
    if (!pmStr || typeof pmStr !== 'string') return undefined;

    const name = pmStr.split('@')[0];
    return getPackageManagerInfoByName(name);
  } catch {
    return undefined;
  }
}

/**
 * Detects package managers in the current directory
 * @param cwd - Current working directory
 * @returns Array of detected package managers
 */
export function detectPackageManagers(cwd: string): PackageManagerInfo[] {
  const pmFromPackageJson = readPackageManagerFromPackageJson(cwd);
  if (pmFromPackageJson) {
    return [pmFromPackageJson];
  }

  const lockfiles = getPossibleLockfiles();
  const detected: PackageManagerInfo[] = [];

  for (const lockfile of lockfiles) {
    const lockfilePath = path.join(cwd, lockfile);
    if (fs.existsSync(lockfilePath)) {
      const pm = getFromLockfile(lockfile);
      if (pm) {
        detected.push(pm);
      }
    }
  }

  return detected;
}

/**
 * Writes package manager to package.json
 * @param cwd - Current working directory
 * @param managerInfo - Package manager to write
 * @returns Result indicating success or failure
 */
export function writePackageManagerToPackageJson(
  cwd: string,
  managerInfo: PackageManagerInfo
) {
  const packageJsonPath = path.join(cwd, 'package.json');

  if (!fs.existsSync(packageJsonPath)) {
    return err('package.json-not-found', `package.json not found at ${packageJsonPath}`);
  }

  const content = fs.readFileSync(packageJsonPath, 'utf-8');
  const pkg = JSON.parse(content);

  const name = managerInfo.name;
  if (!name) {
    return ok();
  }

  const version = detectPackageManagerVersion(managerInfo);
  const pmValue = version ? `${name}@${version}` : name;

  pkg.packageManager = pmValue;

  const serialized = JSON.stringify(pkg, null, 2);
  fs.writeFileSync(packageJsonPath, serialized + '\n');

  return ok();
}

/**
 * Detects package manager version by running its binary
 * @param manager - Package manager info
 * @returns Version string if detected
 */
function detectPackageManagerVersion(manager: PackageManagerInfo): string | undefined {
  const bin = manager.name;

  const run = (flag: string): string | undefined => {
    try {
      const output = childProcess.execSync(`${bin} ${flag}`, {
        encoding: 'utf-8',
        stdio: ['ignore', 'pipe', 'pipe'],
      });
      const text = output.trim();
      return text || undefined;
    } catch {
      return undefined;
    }
  };

  return run('--version') || run('-v');
}

