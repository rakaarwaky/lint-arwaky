import { <VO> } from '../shared/<domain>/taxonomy_<name>_vo';
import { I<Name>Aggregate } from '../shared/<domain>/contract_<name>_aggregate';

export class Surface<Name> {
    constructor(private readonly aggregate: I<Name>Aggregate) {}

    handle(event: TuiEvent): Result<UiState, SurfaceError> {
        // orchestration only
        return Ok(UiState.idle());
    }
}
