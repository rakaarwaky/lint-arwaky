import React, { useRef, useEffect } from 'react';

interface SeverityChartProps {
  errors: number;
  warnings: number;
  info: number;
  isDark: boolean;
}

export function SeverityChart({ errors, warnings, info, isDark }: SeverityChartProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const total = errors + warnings + info;

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Set canvas size with device pixel ratio
    const dpr = window.devicePixelRatio || 1;
    const rect = canvas.getBoundingClientRect();
    canvas.width = rect.width * dpr;
    canvas.height = rect.height * dpr;
    ctx.scale(dpr, dpr);

    const width = rect.width;
    const height = rect.height;
    const centerX = width / 2;
    const centerY = height / 2;
    const radius = Math.min(width, height) / 2 - 16;
    const innerRadius = radius * 0.65;

    // Clear canvas
    ctx.clearRect(0, 0, width, height);

    if (total === 0) {
      // Empty ring
      ctx.beginPath();
      ctx.arc(centerX, centerY, radius, 0, Math.PI * 2);
      ctx.strokeStyle = isDark ? '#3a3a52' : '#e0e0e0';
      ctx.lineWidth = radius - innerRadius;
      ctx.stroke();
      return;
    }

    const slices = [
      { value: errors, color: '#f43f5e' },
      { value: warnings, color: '#f59e0b' },
      { value: info, color: '#3b82f6' },
    ];

    let startAngle = -Math.PI / 2;
    const lineWidth = radius - innerRadius;

    // Draw slices
    ctx.lineCap = 'round';
    
    slices.forEach(({ value, color }) => {
      if (value === 0) return;

      const sliceAngle = (value / total) * Math.PI * 2;
      const endAngle = startAngle + sliceAngle;

      ctx.beginPath();
      ctx.arc(centerX, centerY, (radius + innerRadius) / 2, startAngle + 0.02, endAngle - 0.02);
      ctx.strokeStyle = color;
      ctx.lineWidth = lineWidth;
      ctx.stroke();

      startAngle = endAngle;
    });

    // Center text
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    
    ctx.fillStyle = isDark ? '#e4e4f0' : '#1a1a2e';
    ctx.font = '600 28px -apple-system, BlinkMacSystemFont, sans-serif';
    ctx.fillText(total.toString(), centerX, centerY - 6);
    
    ctx.fillStyle = isDark ? '#a0a0b8' : '#6e6e8a';
    ctx.font = '500 11px -apple-system, BlinkMacSystemFont, sans-serif';
    ctx.fillText('violations', centerX, centerY + 16);
  }, [errors, warnings, info, isDark, total]);

  return (
    <div className="chart-card">
      <h3>Severity Distribution</h3>
      <div className="chart-container">
        <canvas ref={canvasRef} />
      </div>
    </div>
  );
}
