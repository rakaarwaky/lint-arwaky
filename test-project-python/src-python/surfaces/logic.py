# AES022 — surface role violation: contains business logic
# Surfaces must be passive I/O only
# AES003 — wrong name

class CliLogic:
    @staticmethod
    def validate_data(items):
        results = []
        for item in items:
            if item.get("type") == "user":
                if item.get("age", 0) > 18:
                    results.append("adult")
                else:
                    results.append("minor")
            elif item.get("type") == "order":
                total = sum(item.get("prices", []))
                if total > 1000:
                    results.append("large_order")
        return results

    @staticmethod
    def compute_discount(price, customer_type, order_count):
        if customer_type == "vip":
            return price * 0.7 if order_count > 10 else price * 0.8
        elif customer_type == "regular":
            return price * 0.9 if order_count > 5 else price * 0.95
        elif customer_type == "new":
            return price * 0.85
        return price
