"""
bare_except_more.py — TEST PROJECT ONLY.
Intentionally filled with bare except violations (E722) and mypy issues.
BUKAN KODE PRODUKSI.
"""

import json
from typing import Optional, List, Dict


def fetch_data(url: str) -> Optional[Dict]:
    """Bare except with pass — classic E722."""
    try:
        import requests
        resp = requests.get(url, timeout=5)
        return resp.json()
    except:
        pass
    return None


def parse_config(path: str) -> Dict:
    """Bare except with print — still E722."""
    try:
        with open(path, 'r') as f:
            return json.load(f)
    except:
        print(f"Failed to parse {path}")
    return {}


def execute_query(query: str) -> List:
    """Bare except with return default."""
    try:
        import sqlite3
        conn = sqlite3.connect(':memory:')
        cursor = conn.execute(query)
        return cursor.fetchall()
    except:
        return []

