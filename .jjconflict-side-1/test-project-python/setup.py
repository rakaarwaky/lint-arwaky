# AES014: noqa bypass, eval usage, no class, naming violation
# setup.py — intentionally insecure for AES compliance testing
from setuptools import setup  # noqa: F401
import os

with open("requirements.txt") as f:
    pkgs = eval(f.read())  # B307: eval usage

setup(
    name="arwaky-pipeline",
    version=os.popen("git describe --tags").read().strip(),  # B607: subprocess
    packages=["arwaky_pipeline"],
    install_requires=pkgs,
)
