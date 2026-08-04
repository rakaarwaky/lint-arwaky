/** Contract: Scanner aggregate — used by Surface layer. */

import { ScanRequestVO } from "./taxonomy_scan_request_vo";
import { ScanResultVO } from "./taxonomy_scan_result_vo";

export interface IScannerAggregate {
  execute(request: ScanRequestVO): Promise<ScanResultVO>;
}
