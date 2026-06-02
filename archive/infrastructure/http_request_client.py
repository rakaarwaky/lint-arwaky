"""http_request_client — Sync HTTP provider implementation."""

from __future__ import annotations

import httpx
from ..taxonomy import ResponseData, TransportUrlVO, Timeout, ContentString
from ..contract import IHttpProviderPort


class SyncHttpProvider(IHttpProviderPort):
    """Synchronous HTTP client implementing IHttpProviderPort."""

    def get(
        self, url: TransportUrlVO, timeout: Timeout = Timeout(value=2000)
    ) -> ResponseData:
        """Performs a synchronous GET request."""
        with httpx.Client(timeout=timeout.value / 1000) as client:
            resp = client.get(url.value)
            resp.raise_for_status()
            return ResponseData(value=resp.text)

    def post(
        self,
        url: TransportUrlVO,
        data: ContentString | None = None,
        timeout: Timeout = Timeout(value=2000),
    ) -> ResponseData:
        """Performs a synchronous POST request."""
        with httpx.Client(timeout=timeout.value / 1000) as client:
            payload = data.value if data else None
            resp = client.post(url.value, content=payload)
            resp.raise_for_status()
            return ResponseData(value=resp.text)
