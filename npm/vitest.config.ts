import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,
    environment: 'node',
    include: ['src/tests/**/*.ts'],
    typecheck: {
      tsconfig: './tsconfig.test.json',
    },
  },
});

