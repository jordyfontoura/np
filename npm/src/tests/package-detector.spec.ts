import { describe, it, expect } from 'vitest';
import { tmpdir } from 'os';
import { mkdtempSync } from 'fs';
import { join } from 'path';
import { detectPackageManagers } from '../config';
import { getPackageManagerInfoByName } from '../managers';
import * as fs from 'fs';
import * as path from 'path';

/**
 * Writes content to a file
 * @param filePath - Path to the file
 * @param name - File name
 * @param content - Content to write
 */
export function writeFile(filePath: string, name: string, content: string): void {
  fs.writeFileSync(path.join(filePath, name), content);
}

/**
 * Creates an empty file (touch)
 * @param filePath - Path to the directory
 * @param name - File name
 */
export function touch(filePath: string, name: string): void {
  writeFile(filePath, name, '');
}

function createTempDir(): string {
  return mkdtempSync(join(tmpdir(), 'np-test-'));
}

describe('package-detector', () => {
  it('should detect npm', () => {
    const dir = createTempDir();
    touch(dir, 'package-lock.json');
    const detected = detectPackageManagers(dir);
    expect(detected.length).toBe(1);
    expect(detected[0].name).toBe('npm');
  });

  it('should detect yarn', () => {
    const dir = createTempDir();
    touch(dir, 'yarn.lock');
    const detected = detectPackageManagers(dir);
    expect(detected.length).toBe(1);
    expect(detected[0].name).toBe('yarn');
  });

  it('should detect pnpm', () => {
    const dir = createTempDir();
    touch(dir, 'pnpm-lock.yaml');
    const detected = detectPackageManagers(dir);
    expect(detected.length).toBe(1);
    expect(detected[0].name).toBe('pnpm');
  });

  it('should detect none', () => {
    const dir = createTempDir();
    const detected = detectPackageManagers(dir);
    expect(detected.length).toBe(0);
  });

  it('should detect multiple lockfiles', () => {
    const dir = createTempDir();
    touch(dir, 'package-lock.json');
    touch(dir, 'yarn.lock');
    const detected = detectPackageManagers(dir);
    expect(detected.length).toBeGreaterThan(1);
    expect(detected.some((pm) => pm.name === 'npm')).toBe(true);
    expect(detected.some((pm) => pm.name === 'yarn')).toBe(true);
  });

  it('should detect from package.json npm', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"packageManager": "npm@10.0.0"}');
    touch(dir, 'yarn.lock');
    const detected = detectPackageManagers(dir);
    expect(detected.length).toBe(1);
    expect(detected[0].name).toBe('npm');
  });

  it('should detect from package.json yarn', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"packageManager": "yarn@1.22.19"}');
    touch(dir, 'package-lock.json');
    const detected = detectPackageManagers(dir);
    expect(detected.length).toBe(1);
    expect(detected[0].name).toBe('yarn');
  });

  it('should detect from package.json pnpm', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"packageManager": "pnpm@9.9.0"}');
    touch(dir, 'yarn.lock');
    const detected = detectPackageManagers(dir);
    expect(detected.length).toBe(1);
    expect(detected[0].name).toBe('pnpm');
  });

  it('should detect from package.json without version', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"packageManager": "npm"}');
    const detected = detectPackageManagers(dir);
    expect(detected.length).toBe(1);
    expect(detected[0].name).toBe('npm');
  });

  it('should have correct package manager names', () => {
    expect(getPackageManagerInfoByName('npm')?.name).toBe('npm');
    expect(getPackageManagerInfoByName('yarn')?.name).toBe('yarn');
    expect(getPackageManagerInfoByName('pnpm')?.name).toBe('pnpm');
  });
});

