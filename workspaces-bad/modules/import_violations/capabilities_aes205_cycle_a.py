# Fixture: AES205 — circular dependency A side.
from contract.aggregate import ContractAggregate


def process():
    agg = ContractAggregate()
    return agg
