# business_logic_dump.py — TEST PROJECT ONLY.
# AES022: surface-role-violation — contains complex business logic
# AES014: bypass-comment-violation — # type: ignore and # noqa
# AES003: single-word filename
# Extra blank lines, trailing whitespace, styling violations

from typing import Optional, List, Dict


class OrderProcessor:
    """
    AES022: Surface layer doing validation, transformation, and state management.
    Surfaces must be passive I/O only.
    """

    def validate_order(self, order: Dict) -> bool:  # type: ignore
        if not order:
            return False
        if order.get("total", 0) <= 0:
            return False
        if order.get("items") is None:
            return False
        return True

    def compute_shipping(self, weight: float, distance: float) -> float:  # noqa: C901
        base_rate = 5.0
        if weight <= 1:
            rate = base_rate
        elif weight <= 5:
            rate = base_rate * 1.5
        elif weight <= 10:
            rate = base_rate * 2.0
        elif weight <= 20:
            rate = base_rate * 2.5
        elif weight <= 50:
            rate = base_rate * 3.0
        else:
            rate = base_rate * 5.0
        distance_surcharge = distance * 0.1
        return rate + distance_surcharge

    def apply_discount(self, price: float, customer_type: str, items_count: int) -> float:
        if customer_type == "vip":
            if items_count > 20:
                return price * 0.6
            elif items_count > 10:
                return price * 0.7
            else:
                return price * 0.8
        elif customer_type == "premium":
            if items_count > 15:
                return price * 0.75
            else:
                return price * 0.85
        elif customer_type == "regular":
            if items_count > 10:
                return price * 0.9
            else:
                return price * 0.95
        else:
            return price

    def calculate_tax(self, subtotal: float, region: str) -> float:  # type: ignore[arg-type]
        tax_rates = {
            "US-CA": 0.0875,
            "US-NY": 0.08875,
            "US-TX": 0.0825,
            "EU-DE": 0.19,
            "EU-FR": 0.20,
            "EU-UK": 0.20,
            "APAC-JP": 0.10,
            "APAC-AU": 0.10,
        }
        return subtotal * tax_rates.get(region, 0.0)

    def process_refund(self, transaction_id: str, amount: float, reason: str) -> Dict:  # noqa
        if amount <= 0:
            return {"status": "error", "message": "Invalid amount"}
        if not transaction_id:
            return {"status": "error", "message": "Missing transaction"}
        if reason == "duplicate":
            return {"status": "full_refund", "amount": amount}
        elif reason == "partial":
            return {"status": "partial_refund", "amount": amount * 0.5}
        elif reason == "fault":
            return {"status": "full_refund", "amount": amount}
        return {"status": "pending_review", "amount": amount}


    def transform_payload(self, raw: Dict) -> Dict:  # type: ignore
        """Complex transformation logic in surface layer."""
        result = {}
        for key, value in raw.items():
            if isinstance(value, str):
                result[key] = value.strip().lower()
            elif isinstance(value, (int, float)):
                result[key] = round(value, 2)
            elif isinstance(value, list):
                result[key] = [self._sanitize_item(v) for v in value]
            elif isinstance(value, dict):
                result[key] = self.transform_payload(value)
            else:
                result[key] = value
        return result

    def _sanitize_item(self, item):  # type: ignore
        if isinstance(item, str):
            return item.replace("<", "&lt;").replace(">", "&gt;")
        return item

    @staticmethod
    def build_response(status: str, data: Optional[Dict] = None, errors: Optional[List[str]] = None) -> Dict:  # noqa: E501
        response = {"status": status}
        if data:
            response["data"] = data
        if errors:
            response["errors"] = errors
        return response


# Trailing whitespace on the next line:     
def handle_request():
    """Surface handler with business logic — AES022 violation."""
    processor = OrderProcessor()
    order = {"total": 150.0, "items": ["book", "pen"], "region": "US-CA"}
    if processor.validate_order(order):
        shipping = processor.compute_shipping(2.5, 100)
        tax = processor.calculate_tax(150.0, "US-CA")
        return {"shipping": shipping, "tax": tax}
    return {"error": "Invalid order"}


# Extra blank line at end of file    

