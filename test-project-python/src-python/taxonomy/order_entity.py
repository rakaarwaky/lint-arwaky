# AES002 — taxonomy entity without mandatory VO import
# Entity must import VOs from taxonomy

class OrderEntity:
    def __init__(self, order_id: str, customer: str, total: float):
        self.order_id = order_id
        self.customer = customer
        self.total = total

    def process(self):
        return f"Processing order {self.order_id}"
