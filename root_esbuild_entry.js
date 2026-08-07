/** esbuild config — dual bundle: extension (Node.js) + webview (browser). */

const esbuild = require('esbuild');
const path = require('path');
const fs = require('fs');

const production = process.argv.includes('--production');
const watch = process.argv.includes('--watch');

/** @type {import('esbuild').Plugin} */
const esbuildProblemMatcherPlugin = {
  name: 'esbuild-problem-matcher',
  setup(build) {
    build.onStart(() => console.log('[watch] build started'));
    build.onEnd((result) => {
      result.errors.forEach(({ text, location }) => {
        console.error(`✘ [ERROR] ${text}`);
        if (location) {
          console.error(`    ${location.file}:${location.line}:${location.column}:`);
        }
      });
      console.log('[watch] build finished');
    });
  },
};

async function main() {
  // Bundle 1: Extension host (Node.js)
  const extensionConfig = {
    entryPoints: ['packages/root_extension_entry.ts'],
    bundle: true,
    outfile: 'dist/extension.js',
    external: ['vscode'],
    format: 'cjs',
    platform: 'node',
    target: 'node18',
    sourcemap: !production,
    minify: production,
    define: { 'process.env.NODE_ENV': production ? '"production"' : '"development"' },
    plugins: [esbuildProblemMatcherPlugin],
    logLevel: 'info',
  };

  // Bundle 2: Webview (browser)
  const webviewConfig = {
    entryPoints: ['packages/webview/src/index.tsx'],
    bundle: true,
    outfile: 'dist/webview.js',
    format: 'iife',
    platform: 'browser',
    target: 'es2020',
    sourcemap: !production,
    minify: production,
    define: { 'process.env.NODE_ENV': production ? '"production"' : '"development"' },
    plugins: [esbuildProblemMatcherPlugin],
    logLevel: 'info',
    jsx: 'automatic',
    loader: { '.tsx': 'tsx' },
  };

  if (watch) {
    const [extCtx, webCtx] = await Promise.all([
      esbuild.context(extensionConfig),
      esbuild.context(webviewConfig),
    ]);
    await Promise.all([extCtx.watch(), webCtx.watch()]);
    console.log('[watch] watching for changes...');
  } else {
    await Promise.all([
      esbuild.build(extensionConfig),
      esbuild.build(webviewConfig),
    ]);
    console.log('✓ Build complete');
  }
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
