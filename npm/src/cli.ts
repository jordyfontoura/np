import { select, confirm } from '@inquirer/prompts';
import { err, ok } from 'tryless';
import { detectPackageManagers, writePackageManagerToPackageJson } from './config';
import { getPackageManagerInfoByName, listPackageManagerNames } from './managers';

/**
 * Prompts user to select a default package manager
 * @param projectPath - Path to the project
 * @returns Selected package manager or error
 */
async function setDefaultPackageManager(
  projectPath: string
) {
  const names = listPackageManagerNames();

  const choice = await select({
    message: 'Which package manager would you like to use?',
    choices: names,
  }) as string;

  const selected = getPackageManagerInfoByName(choice);
  if (!selected) {
    return err('unexpected-package-manager-chosen', `Unexpected package manager chosen: ${choice}`);
  }

  const canWrite = await confirm({
    message: "Would you like to save this choice in the 'packageManager' field in package.json?",
    default: true,
  });

  if (canWrite) {
    const result = writePackageManagerToPackageJson(projectPath, selected);
    if (!result.success) {
      console.error(`Failed to save 'packageManager' field into package.json: ${result.reason}`);
    }
  }

  return ok(selected);
}

/**
 * Resolves which package manager to use
 * @param cwd - Current working directory
 * @returns Resolved package manager or error
 */
export async function resolvePackageManager(
  cwd: string
) {
  const detected = detectPackageManagers(cwd);

  if (detected.length === 1) {
    return ok(detected[0]!);
  }

  if (detected.length === 0) {
    console.log('\n🤔 Could not determine the package manager.');
    return setDefaultPackageManager(cwd);
  }

  const names = detected.map((pm) => pm.name);
  console.log(`Multiple package managers detected: ${JSON.stringify(names)}`);
  return setDefaultPackageManager(cwd);
}

