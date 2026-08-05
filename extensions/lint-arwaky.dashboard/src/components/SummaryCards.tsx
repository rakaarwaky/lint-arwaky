import React from 'react';

interface SummaryCardsProps {
  total: number;
  errors: number;
  warnings: number;
  info: number;
}

export function SummaryCards({ total, errors, warnings, info }: SummaryCardsProps) {
  return (
    <section className="summary">
      <div className="card card-total">
        <span className="card-value">{total}</span>
        <span className="card-label">Total</span>
      </div>
      <div className="card card-errors">
        <span className="card-value">{errors}</span>
        <span className="card-label">Errors</span>
      </div>
      <div className="card card-warnings">
        <span className="card-value">{warnings}</span>
        <span className="card-label">Warnings</span>
      </div>
      <div className="card card-info">
        <span className="card-value">{info}</span>
        <span className="card-label">Info</span>
      </div>
    </section>
  );
}
