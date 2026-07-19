import { <VO> } from '../shared/<domain>/taxonomy_<name>_vo';

export interface I<Name>Port {
    methodName(param: <VO>): Result<<VO>, Error>;
}
