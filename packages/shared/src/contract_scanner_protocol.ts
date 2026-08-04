/** Contract: Scanner protocol — implemented by Capabilities layer. */

import { ScanRequestVO } from "./taxonomy_scan_request_vo";
import { ScanResultVO } from "./taxonomy_scan_result_vo";

export interface IScannerProtocol {
  scan(request: ScanRequestVO): Promise<ScanResultVO>;
}
