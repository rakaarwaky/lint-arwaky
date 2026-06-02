"""domain_type_checker_protocol — Protocol for enforcing domain types over primitives.

Capabilities implement this (DomainTypeChecker). Agent consumes via DI.
"""

from abc import ABC, abstractmethod
from ..taxonomy import FilePath, PrimitiveTypeList, PrimitiveViolationList


class IDomainTypeProtocol(ABC):
    """Protocol for detecting illicit primitive usage in class attributes."""

    @abstractmethod
    def find_primitive_violations(
        self, path: FilePath, primitive_types: PrimitiveTypeList
    ) -> PrimitiveViolationList:
        """Analyze class attributes to ensure domain types instead of primitives."""
        ...
