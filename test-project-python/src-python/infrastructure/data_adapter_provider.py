"""
Data adapter provider module.
Handles external system communication.
"""
import os

def fetch_external_resource():
    return os.getenv("API_KEY")

# Line 11
# Line 12
