"""domain_type_checker — Capability for enforcing domain type usage over primitives."""

from ..taxonomy import (
    PrimitiveViolation,
    PrimitiveViolationList,
    ColumnNumber,
    FilePath,
    LineNumber,
    PrimitiveTypeList,
    PrimitiveTypeName,
)


from ..contract import ISourceParserPort, IDomainTypeProtocol


class DomainTypeRuleChecker(IDomainTypeProtocol):
    """Business logic for detecting illicit primitive usage in class attributes."""

    def __init__(self, parser: ISourceParserPort):
        self._parser = parser

    def find_primitive_violations(
        self, path: FilePath, primitive_types: PrimitiveTypeList
    ) -> PrimitiveViolationList:
        """
        Analyzes class attributes to ensure they use domain types instead of primitives.
        """
        # Get raw attribute info from infrastructure
        class_data = self._parser.get_class_attributes(path)

        violations: list[PrimitiveViolation] = []

        for class_name, attributes in class_data.value.items():
            for attr in attributes:
                type_name = attr.get("type_name")
                if type_name in primitive_types:
                    violations.append(
                        PrimitiveViolation(
                            line=LineNumber(value=attr.get("line", 0)),
                            column=ColumnNumber(value=attr.get("column", 0)),
                            type_name=PrimitiveTypeName(value=str(type_name))
                            if type_name is not None
                            else PrimitiveTypeName(value=""),
                        )
                    )

        return PrimitiveViolationList(values=violations)
