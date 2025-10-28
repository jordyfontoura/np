/**
 * Package manager information and detection logic
 */

export interface PackageManagerInfo {
  name: string;
  lockfiles: readonly string[];
}

const PACKAGE_MANAGERS: readonly PackageManagerInfo[] = [
  {
    name: 'npm',
    lockfiles: ['package-lock.json'],
  },
  {
    name: 'yarn',
    lockfiles: ['yarn.lock'],
  },
  {
    name: 'pnpm',
    lockfiles: ['pnpm-lock.yaml'],
  },
];

/**
 * Gets all possible lockfile names
 * @returns Array of lockfile names
 */
export function getPossibleLockfiles(): string[] {
  return PACKAGE_MANAGERS.flatMap((pm) => pm.lockfiles);
}

/**
 * Gets package manager info by lockfile name
 * @param file - The lockfile name
 * @returns Package manager info if found
 */
export function getFromLockfile(file: string): PackageManagerInfo | undefined {
  return PACKAGE_MANAGERS.find((pm) => pm.lockfiles.includes(file));
}

/**
 * Gets list of all package manager names
 * @returns Array of package manager names
 */
export function listPackageManagerNames(): string[] {
  return PACKAGE_MANAGERS.map((pm) => pm.name);
}

/**
 * Gets package manager info by name
 * @param name - The package manager name
 * @returns Package manager info if found
 */
export function getPackageManagerInfoByName(
  name: string
): PackageManagerInfo | undefined {
  return PACKAGE_MANAGERS.find((pm) => pm.name === name);
}

