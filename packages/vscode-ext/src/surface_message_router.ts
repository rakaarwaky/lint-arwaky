/** Surface: Smart message router — routes extension→webview messages.

Maps message commands to webview posting.
*/

import * as vscode from "vscode";
import { ExtensionToWebviewMessage } from "@lint-arwaky/shared/src/taxonomy_webview_vo";

export class MessageRouter {
  constructor(private readonly webview: vscode.Webview | undefined) {}

  send(message: ExtensionToWebviewMessage): void {
    if (this.webview) {
      this.webview.postMessage(message);
    }
  }
}
