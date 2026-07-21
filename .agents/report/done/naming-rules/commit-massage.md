
fix(naming-rules): correct surface layer mapping and improve checker reliability

- Fix surface_ prefix mapping: surface_ -> surface, not surfaces.
- Split INamingCheckerProtocol into INamingConventionChecker and ISuffixPrefixChecker.
- Make run_audit return Result<Vec<LintResult></lintresult>, ScanError>.
- Centralize layer detection and lint-result construction.
- Use config-driven AES101 minimum word count.
- Default AES101 word_count to 3.
- Replace infrastructure_ guidance with utility_ in violation messages.
