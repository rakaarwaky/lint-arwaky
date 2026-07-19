import { <VO> } from '../shared/<domain>/taxonomy_<name>_vo';

export interface I<Name>Aggregate {
    execute(request: ScanRequest): LintResult[];
}
