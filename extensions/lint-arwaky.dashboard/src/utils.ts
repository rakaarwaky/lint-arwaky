import type { FileViolation, RuleGroup, ScanViolation } from './types';

// Rule group colors
export const RULE_GROUP_COLORS: Record<string, string> = {
  naming: '#7c5cfc',
  import: '#3b82f6',
  orphan: '#f59e0b',
  quality: '#22c55e',
  role: '#f43f5e',
  other: '#6e6e8a',
};

// Extract rule group from rule name (e.g., "AES101" -> "naming")
export function extractGroup(rule: string): string {
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

// Group violations by rule group
export function groupByRule(violations: ScanViolation[]): RuleGroup[] {
  const groupCounts: Record<string, number> = {};
  
  violations.forEach((v) => {
    const group = v.group || extractGroup(v.rule);
    groupCounts[group] = (groupCounts[group] || 0) + 1;
  });

  return Object.entries(groupCounts)
    .map(([name, count]) => ({
      name,
      count,
      color: RULE_GROUP_COLORS[name] || RULE_GROUP_COLORS.other,
    }))
    .sort((a, b) => b.count - a.count);
}

// Group violations by file
export function groupByFile(violations: ScanViolation[]): FileViolation[] {
  const fileMap: Record<string, FileViolation> = {};
  
  violations.forEach((v) => {
    if (!fileMap[v.file]) {
      fileMap[v.file] = { file: v.file, errors: 0, warnings: 0, info: 0, total: 0 };
    }
    const key = v.severity === 'error' ? 'errors' : v.severity === 'warning' ? 'warnings' : 'info';
    fileMap[v.file][key]++;
    fileMap[v.file].total++;
  });

  return Object.values(fileMap).sort((a, b) => b.total - a.total);
}

// Format timestamp
export function formatTime(timestamp: string): string {
  const date = new Date(timestamp);
  return date.toLocaleTimeString();
}
