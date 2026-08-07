import React from 'react';
import type { RuleGroup } from '../types';

interface RuleGroupsChartProps {
  groups: RuleGroup[];
}

export function RuleGroupsChart({ groups }: RuleGroupsChartProps) {
  const maxCount = Math.max(...groups.map((g) => g.count), 1);

  return (
    <div className="chart-card">
      <h3>By Rule Group</h3>
      <div className="rule-groups">
        {groups.map((group) => (
          <div key={group.name} className="rule-group">
            <span className="rule-group-name">{group.name}</span>
            <div className="rule-group-bar">
              <div
                className="rule-group-fill"
                style={{
                  width: `${(group.count / maxCount) * 100}%`,
                  background: group.color,
                }}
              />
            </div>
            <span className="rule-group-count">{group.count}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
