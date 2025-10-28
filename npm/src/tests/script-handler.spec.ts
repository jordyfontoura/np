import { describe, it, expect } from 'vitest';
import { tmpdir } from 'os';
import { mkdtempSync } from 'fs';
import { join } from 'path';
import { readFileSync } from 'fs';
import { readPackageManagerFromPackageJson, writePackageManagerToPackageJson } from '../config';
import { getPackageManagerInfoByName } from '../managers';
import * as fs from 'fs';
import * as path from 'path';

/**
 * Writes content to a file
 * @param filePath - Path to the file
 * @param name - File name
 * @param content - Content to write
 */
function writeFile(filePath: string, name: string, content: string): void {
  fs.writeFileSync(path.join(filePath, name), content);
}

function createTempDir(): string {
  return mkdtempSync(join(tmpdir(), 'np-test-'));
}

describe('script-handler', () => {
  it('should read package manager npm', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"packageManager": "npm@10.0.0"}');
    const pm = readPackageManagerFromPackageJson(dir);
    expect(pm?.name).toBe('npm');
  });

  it('should read package manager yarn', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"packageManager": "yarn@1.22.19"}');
    const pm = readPackageManagerFromPackageJson(dir);
    expect(pm?.name).toBe('yarn');
  });

  it('should read package manager pnpm', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"packageManager": "pnpm@9.9.0"}');
    const pm = readPackageManagerFromPackageJson(dir);
    expect(pm?.name).toBe('pnpm');
  });

  it('should read package manager without version', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"packageManager": "npm"}');
    const pm = readPackageManagerFromPackageJson(dir);
    expect(pm?.name).toBe('npm');
  });

  it('should read package manager no field', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"name": "test"}');
    const pm = readPackageManagerFromPackageJson(dir);
    expect(pm).toBeUndefined();
  });

  it('should read package manager no package.json', () => {
    const dir = createTempDir();
    const pm = readPackageManagerFromPackageJson(dir);
    expect(pm).toBeUndefined();
  });

  it('should read package manager invalid json', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"invalid json');
    const pm = readPackageManagerFromPackageJson(dir);
    expect(pm).toBeUndefined();
  });

  it('should read package manager unknown manager', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"packageManager": "bun@1.0.0"}');
    const pm = readPackageManagerFromPackageJson(dir);
    expect(pm).toBeUndefined();
  });

  it('should write package manager npm', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"name": "test"}');
    const npm = getPackageManagerInfoByName('npm')!;
    const result = writePackageManagerToPackageJson(dir, npm);
    expect(result.success).toBe(true);

    const content = readFileSync(join(dir, 'package.json'), 'utf-8');
    expect(content).toContain('packageManager');
    expect(content).toContain('npm');
  });

  it('should write package manager yarn', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"name": "test"}');
    const yarn = getPackageManagerInfoByName('yarn')!;
    const result = writePackageManagerToPackageJson(dir, yarn);
    expect(result.success).toBe(true);

    const content = readFileSync(join(dir, 'package.json'), 'utf-8');
    expect(content).toContain('packageManager');
    expect(content).toContain('yarn');
  });

  it('should write package manager pnpm', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"name": "test"}');
    const pnpm = getPackageManagerInfoByName('pnpm')!;
    const result = writePackageManagerToPackageJson(dir, pnpm);
    expect(result.success).toBe(true);

    const content = readFileSync(join(dir, 'package.json'), 'utf-8');
    expect(content).toContain('packageManager');
    expect(content).toContain('pnpm');
  });

  it('should write package manager overwrites existing', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"packageManager": "npm@10.0.0"}');
    const yarn = getPackageManagerInfoByName('yarn')!;
    const result = writePackageManagerToPackageJson(dir, yarn);
    expect(result.success).toBe(true);

    const content = readFileSync(join(dir, 'package.json'), 'utf-8');
    expect(content).toContain('yarn');
    expect(content).not.toContain('"npm"');
  });

  it('should write package manager preserves other fields', () => {
    const dir = createTempDir();
    writeFile(dir, 'package.json', '{"name": "test", "version": "1.0.0"}');
    const npm = getPackageManagerInfoByName('npm')!;
    const result = writePackageManagerToPackageJson(dir, npm);
    expect(result.success).toBe(true);

    const content = readFileSync(join(dir, 'package.json'), 'utf-8');
    expect(content).toContain('name');
    expect(content).toContain('test');
    expect(content).toContain('version');
    expect(content).toContain('1.0.0');
    expect(content).toContain('packageManager');
  });

  it('should write package manager no package.json', () => {
    const dir = createTempDir();
    const npm = getPackageManagerInfoByName('npm')!;
    const result = writePackageManagerToPackageJson(dir, npm);
    expect(result.success).toBe(false);
  });

  it('should write package manager maintains json structure', () => {
    const dir = createTempDir();
    writeFile(
      dir,
      'package.json',
      JSON.stringify({
        name: 'test',
        version: '1.0.0',
        dependencies: {
          axios: '^1.0.0',
        },
      })
    );

    const pnpm = getPackageManagerInfoByName('pnpm')!;
    const result = writePackageManagerToPackageJson(dir, pnpm);
    expect(result.success).toBe(true);

    const content = readFileSync(join(dir, 'package.json'), 'utf-8');
    JSON.parse(content);
    expect(content).toContain('packageManager');
    expect(content).toContain('pnpm');
  });
});

