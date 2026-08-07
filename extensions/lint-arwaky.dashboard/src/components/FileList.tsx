import React from 'react';
import type { FileViolation } from '../types';

interface FileListProps {
  files: FileViolation[];
}

export function FileList({ files }: FileListProps) {
  return (
    <section className="files-section">
      <h3>Violations by File</h3>
      <div className="file-list">
        {files.map((file) => {
          const countClass = file.errors > 0 
            ? 'has-errors' 
            : file.warnings > 0 
              ? 'warnings-only' 
              : 'info-only';
          
          return (
            <div key={file.file} className="file-item">
              <span className="file-name">{file.file}</span>
              <span className={`file-count ${countClass}`}>{file.total}</span>
            </div>
          );
        })}
      </div>
    </section>
  );
}
