/**
 * Lint Arwaky Dashboard - Main Module
 * Real-time visualization of lint scan results
 */

// Types
interface ScanViolation {
  file: string;
  line: number;
  column: number;
  rule: string;
  severity: 'error' | 'warning' | 'info';
  message: string;
  group?: string;
}

interface ScanResults {
  timestamp: string;
  summary: {
    total: number;
    errors: number;
    warnings: number;
    info: number;
  };
  violations: ScanViolation[];
}

interface RuleGroup {
  name: string;
  count: number;
  color: string;
}

interface FileViolation {
  file: string;
  errors: number;
  warnings: number;
  info: number;
  total: number;
}

// State
let currentResults: ScanResults | null = null;
let themeType: 'dark' | 'light' = 'dark';

// DOM Elements
const elements = {
  emptyState: () => document.getElementById('empty-state'),
  dashboard: () => document.getElementById('dashboard'),
  totalCount: () => document.getElementById('total-count'),
  errorCount: () => document.getElementById('error-count'),
  warningCount: () => document.getElementById('warning-count'),
  infoCount: () => document.getElementById('info-count'),
  workspaceInfo: () => document.getElementById('workspace-info'),
  lastScan: () => document.getElementById('last-scan'),
  ruleGroups: () => document.getElementById('rule-groups'),
  fileList: () => document.getElementById('file-list'),
  importData: () => document.getElementById('import-data') as HTMLTextAreaElement,
  importBtn: () => document.getElementById('import-btn'),
  severityChart: () => document.getElementById('severity-chart') as HTMLCanvasElement,
};

// Rule group colors
const RULE_GROUP_COLORS: Record<string, string> = {
  naming: '#3b82f6',
  import: '#8b5cf6',
  orphan: '#f59e0b',
  quality: '#22c55e',
  role: '#ef4444',
  other: '#6b7280',
};

// Initialize
async function init() {
  // Check if cate API is available
  if (typeof (window as any).cate === 'undefined') {
    console.warn('Cate API not available, running in standalone mode');
    showEmptyState();
    return;
  }

  const cate = (window as any).cate;

  // Get theme
  try {
    const theme = await cate.theme.get();
    themeType = theme.type;
    applyTheme(theme);
  } catch (e) {
    console.warn('Theme not available:', e);
  }

  // Get workspace info
  try {
    const workspace = await cate.workspace.get();
    if (workspace) {
      elements.workspaceInfo()!.textContent = workspace.branch || 'No branch';
    }
  } catch (e) {
    console.warn('Workspace not available:', e);
  }

  // Load stored results
  await loadResults();

  // Set up import button
  elements.importBtn()?.addEventListener('click', handleImport);

  // Listen for storage changes
  try {
    cate.storage.onChange((key: string) => {
      if (key === 'scan-results') {
        loadResults();
      }
    });
  } catch (e) {
    console.warn('Storage onChange not available:', e);
  }
}

// Apply theme
function applyTheme(theme: { type: 'dark' | 'light' }) {
  const root = document.documentElement;
  if (theme.type === 'dark') {
    root.style.setProperty('--bg-primary', '#1a1a1a');
    root.style.setProperty('--bg-secondary', '#242424');
    root.style.setProperty('--bg-tertiary', '#333333');
    root.style.setProperty('--text-primary', '#ffffff');
    root.style.setProperty('--text-secondary', '#999999');
    root.style.setProperty('--border-color', '#404040');
  } else {
    root.style.setProperty('--bg-primary', '#ffffff');
    root.style.setProperty('--bg-secondary', '#f5f5f5');
    root.style.setProperty('--bg-tertiary', '#e8e8e8');
    root.style.setProperty('--text-primary', '#1a1a1a');
    root.style.setProperty('--text-secondary', '#666666');
    root.style.setProperty('--border-color', '#e0e0e0');
  }
}

// Load results from storage
async function loadResults() {
  if (typeof (window as any).cate === 'undefined') {
    // Try to load from localStorage for standalone mode
    const stored = localStorage.getItem('scan-results');
    if (stored) {
      try {
        currentResults = JSON.parse(stored);
        renderDashboard();
      } catch (e) {
        showEmptyState();
      }
    } else {
      showEmptyState();
    }
    return;
  }

  try {
    const results = await (window as any).cate.storage.get('scan-results');
    if (results) {
      currentResults = results;
      renderDashboard();
    } else {
      showEmptyState();
    }
  } catch (e) {
    console.error('Failed to load results:', e);
    showEmptyState();
  }
}

// Show empty state
function showEmptyState() {
  elements.emptyState()?.classList.remove('hidden');
  elements.dashboard()?.classList.add('hidden');
}

// Render dashboard
function renderDashboard() {
  if (!currentResults) {
    showEmptyState();
    return;
  }

  elements.emptyState()?.classList.add('hidden');
  elements.dashboard()?.classList.remove('hidden');

  // Update summary
  elements.totalCount()!.textContent = currentResults.summary.total.toString();
  elements.errorCount()!.textContent = currentResults.summary.errors.toString();
  elements.warningCount()!.textContent = currentResults.summary.warnings.toString();
  elements.infoCount()!.textContent = currentResults.summary.info.toString();

  // Update last scan time
  const scanTime = new Date(currentResults.timestamp);
  elements.lastScan()!.textContent = `Last: ${scanTime.toLocaleTimeString()}`;

  // Render charts
  renderSeverityChart();
  renderRuleGroups();
  renderFileList();
}

// Render severity donut chart
function renderSeverityChart() {
  if (!currentResults) return;

  const canvas = elements.severityChart();
  if (!canvas) return;

  const ctx = canvas.getContext('2d');
  if (!ctx) return;

  // Set canvas size
  const dpr = window.devicePixelRatio || 1;
  const rect = canvas.getBoundingClientRect();
  canvas.width = rect.width * dpr;
  canvas.height = rect.height * dpr;
  ctx.scale(dpr, dpr);

  const width = rect.width;
  const height = rect.height;
  const centerX = width / 2;
  const centerY = height / 2;
  const radius = Math.min(width, height) / 2 - 20;
  const innerRadius = radius * 0.6;

  const { errors, warnings, info } = currentResults.summary;
  const total = errors + warnings + info;

  if (total === 0) {
    // Empty state
    ctx.beginPath();
    ctx.arc(centerX, centerY, radius, 0, Math.PI * 2);
    ctx.fillStyle = themeType === 'dark' ? '#333' : '#e8e8e8';
    ctx.fill();
    return;
  }

  const slices = [
    { value: errors, color: '#ef4444' },
    { value: warnings, color: '#f59e0b' },
    { value: info, color: '#3b82f6' },
  ];

  let startAngle = -Math.PI / 2;

  slices.forEach(({ value, color }) => {
    if (value === 0) return;

    const sliceAngle = (value / total) * Math.PI * 2;

    ctx.beginPath();
    ctx.moveTo(centerX + innerRadius * Math.cos(startAngle), centerY + innerRadius * Math.sin(startAngle));
    ctx.arc(centerX, centerY, radius, startAngle, startAngle + sliceAngle);
    ctx.arc(centerX, centerY, innerRadius, startAngle + sliceAngle, startAngle, true);
    ctx.closePath();
    ctx.fillStyle = color;
    ctx.fill();

    startAngle += sliceAngle;
  });

  // Center text
  ctx.fillStyle = themeType === 'dark' ? '#fff' : '#1a1a1a';
  ctx.font = 'bold 24px -apple-system, sans-serif';
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  ctx.fillText(total.toString(), centerX, centerY - 8);
  ctx.font = '12px -apple-system, sans-serif';
  ctx.fillStyle = themeType === 'dark' ? '#999' : '#666';
  ctx.fillText('violations', centerX, centerY + 12);
}

// Render rule groups
function renderRuleGroups() {
  if (!currentResults) return;

  const container = elements.ruleGroups();
  if (!container) return;

  // Count by rule group
  const groupCounts: Record<string, number> = {};
  currentResults.violations.forEach((v) => {
    const group = v.group || extractGroup(v.rule);
    groupCounts[group] = (groupCounts[group] || 0) + 1;
  });

  // Sort by count
  const groups: RuleGroup[] = Object.entries(groupCounts)
    .map(([name, count]) => ({
      name,
      count,
      color: RULE_GROUP_COLORS[name] || RULE_GROUP_COLORS.other,
    }))
    .sort((a, b) => b.count - a.count);

  const maxCount = Math.max(...groups.map((g) => g.count), 1);

  container.innerHTML = groups
    .map(
      (g) => `
    <div class="rule-group">
      <span class="rule-group-name">${g.name}</span>
      <div class="rule-group-bar">
        <div class="rule-group-fill" style="width: ${(g.count / maxCount) * 100}%; background: ${g.color}"></div>
      </div>
      <span class="rule-group-count">${g.count}</span>
    </div>
  `
    )
    .join('');
}

// Extract rule group from rule name (e.g., "AES101" -> "naming")
function extractGroup(rule: string): string {
  const match = rule.match(/AES(\d)/);
  if (!match) return 'other';
  const prefix = parseInt(match[1], 10);
  if (prefix === 1) return 'naming';
  if (prefix === 2) return 'import';
  if (prefix === 3) return 'quality';
  if (prefix === 4) return 'role';
  if (prefix === 5) return 'orphan';
  return 'other';
}

// Render file list
function renderFileList() {
  if (!currentResults) return;

  const container = elements.fileList();
  if (!container) return;

  // Group by file
  const fileMap: Record<string, FileViolation> = {};
  currentResults.violations.forEach((v) => {
    if (!fileMap[v.file]) {
      fileMap[v.file] = { file: v.file, errors: 0, warnings: 0, info: 0, total: 0 };
    }
    fileMap[v.file][v.severity === 'error' ? 'errors' : v.severity === 'warning' ? 'warnings' : 'info']++;
    fileMap[v.file].total++;
  });

  // Sort by total violations
  const files = Object.values(fileMap).sort((a, b) => b.total - a.total);

  container.innerHTML = files
    .map((f) => {
      const countClass = f.errors > 0 ? 'has-errors' : f.warnings > 0 ? 'warnings-only' : 'info-only';
      return `
      <div class="file-item">
        <span class="file-name">${f.file}</span>
        <span class="file-count ${countClass}">${f.total}</span>
      </div>
    `;
    })
    .join('');
}

// Handle import
async function handleImport() {
  const textarea = elements.importData();
  if (!textarea) return;

  const text = textarea.value.trim();
  if (!text) return;

  try {
    const data = JSON.parse(text) as ScanResults;

    // Validate structure
    if (!data.summary || !Array.isArray(data.violations)) {
      throw new Error('Invalid format: missing summary or violations');
    }

    // Add timestamp if missing
    if (!data.timestamp) {
      data.timestamp = new Date().toISOString();
    }

    currentResults = data;

    // Save to storage
    if (typeof (window as any).cate !== 'undefined') {
      await (window as any).cate.storage.set('scan-results', data);
    } else {
      localStorage.setItem('scan-results', JSON.stringify(data));
    }

    // Render
    renderDashboard();

    // Clear textarea
    textarea.value = '';

    // Notify
    if (typeof (window as any).cate !== 'undefined') {
      await (window as any).cate.ui.notify('Scan results imported successfully', 'info');
    }
  } catch (e) {
    console.error('Import failed:', e);
    if (typeof (window as any).cate !== 'undefined') {
      await (window as any).cate.ui.notify('Invalid JSON format', 'error');
    } else {
      alert('Invalid JSON format');
    }
  }
}

// Start
init();
