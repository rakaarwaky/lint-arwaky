import { createServer } from 'node:http';
import { readFile, stat } from 'node:fs/promises';
import { join, extname } from 'node:path';
import { spawn, execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { dirname } from 'node:path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const DIST_DIR = join(__dirname, '..', 'dist');
const WORKSPACE_ROOT = process.env.WORKSPACE_ROOT || process.cwd();

const HOST = process.env.HOST || '127.0.0.1';
const PORT = parseInt(process.env.PORT || '3000', 10);
const CATE_TOKEN = process.env.CATE_TOKEN;

const MIME = {
  '.html': 'text/html',
  '.js': 'application/javascript',
  '.css': 'text/css',
  '.json': 'application/json',
  '.svg': 'image/svg+xml',
  '.png': 'image/png',
  '.ico': 'image/x-icon',
  '.woff': 'font/woff',
  '.woff2': 'font/woff2',
};

function requireAuth(req, res) {
  if (!CATE_TOKEN) return true;
  // Allow direct browser access (no auth header = standalone mode)
  const auth = req.headers['authorization'];
  if (!auth) return true;
  if (auth === `Bearer ${CATE_TOKEN}`) return true;
  res.writeHead(401, { 'Content-Type': 'application/json' });
  res.end(JSON.stringify({ error: 'unauthorized' }));
  return false;
}

function json(res, status, data) {
  const body = JSON.stringify(data);
  res.writeHead(status, {
    'Content-Type': 'application/json',
    'Content-Length': Buffer.byteLength(body),
  });
  res.end(body);
}

function sendFile(res, filePath) {
  readFile(filePath)
    .then((data) => {
      const mime = MIME[extname(filePath)] || 'application/octet-stream';
      res.writeHead(200, { 'Content-Type': mime, 'Content-Length': data.length });
      res.end(data);
    })
    .catch(() => {
      res.writeHead(404);
      res.end('Not found');
    });
}

function parseBody(req) {
  return new Promise((resolve, reject) => {
    const chunks = [];
    req.on('data', (c) => chunks.push(c));
    req.on('end', () => {
      try {
        resolve(JSON.parse(Buffer.concat(chunks).toString()));
      } catch {
        resolve({});
      }
    });
    req.on('error', reject);
  });
}

const SEVERITY_MAP = { critical: 'error', high: 'error', medium: 'warning', low: 'info' };

function mapCliOutput(raw) {
  const violations = (raw.results || []).map((r) => ({
    file: r.file,
    line: r.line,
    column: r.column || 0,
    rule: r.code || 'UNKNOWN',
    severity: SEVERITY_MAP[r.severity] || 'info',
    message: r.message || '',
    group: r.member || undefined,
  }));

  let errors = 0;
  let warnings = 0;
  let info = 0;
  for (const v of violations) {
    if (v.severity === 'error') errors++;
    else if (v.severity === 'warning') warnings++;
    else info++;
  }

  return {
    summary: {
      total: raw.total_violations ?? violations.length,
      errors,
      warnings,
      info,
    },
    violations,
  };
}

function findCliBinary() {
  // 1. Check PATH (global install via cargo install)
  try {
    const path = execFileSync('which', ['lint-arwaky-cli'], { encoding: 'utf-8' }).trim();
    if (path) return path;
  } catch {}

  // 2. Fallback to local target directories
  const candidates = [
    join(WORKSPACE_ROOT, 'target', 'release', 'lint-arwaky-cli'),
    join(WORKSPACE_ROOT, 'target', 'debug', 'lint-arwaky-cli'),
  ];
  return candidates[0];
}

async function handleScan(req, res) {
  if (!requireAuth(req, res)) return;

  const body = await parseBody(req);
  const scanPath = body.path || '.';
  const cliPath = findCliBinary();

  // Resolve scanPath: absolute paths used as-is, relative paths resolved from workspace root
  const resolvedPath = scanPath.startsWith('/') ? scanPath : join(WORKSPACE_ROOT, scanPath);

  const child = spawn(cliPath, ['scan', resolvedPath, '--format', 'json'], {
    cwd: WORKSPACE_ROOT,
    stdio: ['ignore', 'pipe', 'pipe'],
    env: { ...process.env, RUST_BACKTRACE: '1' },
    timeout: 300_000,
  });

  let stdout = '';
  let stderr = '';

  child.stdout.on('data', (d) => (stdout += d));
  child.stderr.on('data', (d) => (stderr += d));

  child.on('error', (err) => {
    json(res, 500, {
      error: 'cli-not-found',
      message: `Could not run lint-arwaky-cli: ${err.message}. Build it first: cargo build --release -p lint-arwaky-cli`,
      stderr,
    });
  });

  child.on('close', (code) => {
    if (code !== 0 && !stdout.trim()) {
      json(res, 500, {
        error: 'cli-error',
        message: `lint-arwaky-cli exited with code ${code}`,
        stderr: stderr.slice(0, 2000),
      });
      return;
    }

    try {
      const raw = JSON.parse(stdout);
      const data = mapCliOutput(raw);
      json(res, 200, data);
    } catch {
      // CLI might not support --json; return raw output
      json(res, 200, {
        summary: { total: 0, errors: 0, warnings: 0, info: 0 },
        violations: [],
        raw: stdout.slice(0, 10_000),
        note: 'CLI did not return valid JSON. Ensure --json flag is supported.',
      });
    }
  });
}

async function handleStatus(req, res) {
  if (!requireAuth(req, res)) return;

  const cliPath = findCliBinary();
  let cliAvailable = false;
  try {
    await stat(cliPath);
    cliAvailable = true;
  } catch {}

  json(res, 200, { cliAvailable, cliPath, workspaceRoot: WORKSPACE_ROOT });
}

function serveStatic(req, res) {
  const url = new URL(req.url, `http://${HOST}:${PORT}`);
  let filePath = join(DIST_DIR, url.pathname === '/' ? 'index.html' : url.pathname);

  // SPA fallback: if file doesn't exist, serve index.html
  stat(filePath)
    .then((s) => {
      if (s.isFile()) {
        sendFile(res, filePath);
      } else {
        sendFile(res, join(DIST_DIR, 'index.html'));
      }
    })
    .catch(() => {
      sendFile(res, join(DIST_DIR, 'index.html'));
    });
}

const server = createServer(async (req, res) => {
  const url = new URL(req.url, `http://${HOST}:${PORT}`);

  if (url.pathname === '/api/scan' && req.method === 'POST') {
    return handleScan(req, res);
  }
  if (url.pathname === '/api/status' && req.method === 'GET') {
    return handleStatus(req, res);
  }

  return serveStatic(req, res);
});

server.listen(PORT, HOST, () => {
  console.log(`lint-arwaky-dashboard server on http://${HOST}:${PORT}`);
});
