"""architecture_orchestrator — Logic for resolving ArchitectureConfig into LayerDefinitions."""

from __future__ import annotations

from ..taxonomy import (
    ArchitectureConfig,
    LayerDefinition,
    ArchitectureRule,
    LAYER_GLOBAL,
    LayerMapVO,
)
from ..contract import ArchitectureOrchestratorAggregate


class ArchitectureOrchestrator(ArchitectureOrchestratorAggregate):
    """Orchestrates the resolution of architectural configurations."""

    def resolve_effective_layer_map(self, config: ArchitectureConfig) -> LayerMapVO:
        """
        Merges base layer definitions with global and layer-specific rules.
        Replaces the logic previously found in ArchitectureConfig.get_effective_layer_map().
        """
        # Ground mandatory imports
        _arch_aggregate: ArchitectureOrchestratorAggregate = self

        layer_map = self._resolve_layers(config)
        layer_map = self._merge_layer_rules(layer_map, config)
        return LayerMapVO(values=self._finalize_map(layer_map))

    def _resolve_layers(self, config: ArchitectureConfig):
        """Initialize base layer definitions from config — preserve suffix policy & allowed/forbidden suffixes AS-IS."""
        layer_map = {}
        for name, entry in config.layers.items():
            layer_map[str(name)] = entry
        return layer_map

    def _merge_layer_rules(self, layer_map, config: ArchitectureConfig):
        """Merge all rules from config into the layer map, handling global, specialized, and base scopes."""
        result = dict(layer_map)  # shallow copy to avoid mutating caller's dict

        for rule in config.rules:
            scope = str(rule.scope)
            if scope == LAYER_GLOBAL:
                self._apply_global_rule(result, rule)
                continue

            base_name = scope.split("(")[0] if "(" in scope else scope
            base_definition = result.get(base_name)
            if base_definition is None:
                continue  # omit rules targeting unknown layers

            # Clone specialized scope from base definition if not yet present
            if scope not in result and scope != base_name:
                result[scope] = base_definition.model_copy(deep=True)

            if "(" in scope:
                self._apply_specialized_rule(result, scope, rule)
            else:
                self._apply_base_rule(result, scope, rule)

        return result

    def _apply_global_rule(self, result, rule: ArchitectureRule) -> None:
        """Apply rule with scope=LAYER_GLOBAL to all non-specialized layers."""
        for layer_name in list(result.keys()):
            if "(" in layer_name:
                continue  # omit specialized layers for global rules
            result[layer_name] = self._merge_rule_into_definition(result[layer_name], rule)

    def _apply_specialized_rule(self, result, scope, rule: ArchitectureRule) -> None:
        """Apply rule to a single specialized layer scope (e.g. 'service(admin)')."""
        if scope in result:
            result[scope] = self._merge_rule_into_definition(result[scope], rule)

    def _apply_base_rule(self, result, scope, rule: ArchitectureRule) -> None:
        """Apply rule to a single base/named layer (no parentheses in scope)."""
        if scope in result:
            result[scope] = self._merge_rule_into_definition(result[scope], rule)

    def _finalize_map(self, layer_map):
        """Final pass over resolved layer map — ready for return."""
        return layer_map

    def _get_target_layers(self, rule: ArchitectureRule, layer_map):
        scope = str(rule.scope)
        if scope == LAYER_GLOBAL:
            return list(layer_map.keys())

        # Handle list of layers in scope
        if hasattr(rule.scope, 'values'): # PatternList
            return [str(s) for s in rule.scope.values if str(s) in layer_map]

        if scope in layer_map:
            return [scope]

        return []

    def _merge_rule_into_definition(self, definition: LayerDefinition, rule: ArchitectureRule) -> LayerDefinition:
        """Surgically merges rule settings into a LayerDefinition."""
        update_data = {}
        update_data.update(self._apply_rule_overrides(rule))
        update_data.update(self._apply_exceptions(rule))

        # Additively merge exceptions instead of overwriting!
        from ..taxonomy import PatternList
        existing_exceptions = definition.exceptions.values if definition.exceptions else []
        rule_exceptions = rule.exceptions.values if rule.exceptions else []
        if rule_exceptions:
            merged_exceptions = list(sorted(set(existing_exceptions + rule_exceptions)))
            update_data["exceptions"] = PatternList(values=merged_exceptions)

        return definition.model_copy(update=update_data)

    def _apply_rule_overrides(self, rule: ArchitectureRule):
        """Extract scalar override fields from a rule (simple fields, booleans, violation messages)."""
        update_data = {}

        # 1. Simple scalar fields (override if not None)
        simple_fields = [
            "word_count", "suffix_policy",
            "mandatory_import_violation_message", "forbidden_import_violation_message",
            "word_count_violation_message", "suffix_violation_message",
            "no_primitives_violation_message", "min_lines_violation_message",
            "max_lines_violation_message", "barrel_completeness_violation_message",
            "forbid_internal_all_violation_message", "forbidden_bypass_violation_message",
            "mandatory_class_definition_violation_message", "dead_inheritance_bypass_violation_message",
            "orphan_violation_message", "check_unused_mandatory_imports_violation_message",
            "min_lines", "max_lines"
        ]

        # Role-specific violation messages
        role_violation_fields = [
            "no_domain_logic_violation_message", "must_implement_service_container_aggregate_violation_message",
            "lazy_eager_initialization_only_violation_message", "stateless_execution_violation_message",
            "single_execution_goal_violation_message", "high_level_policy_only_violation_message",
            "coordinates_multiple_orchestrators_violation_message", "crud_only_violation_message",
            "no_decision_logic_violation_message", "thread_async_safe_violation_message",
            "no_domain_data_storage_violation_message", "owns_system_health_transitions_violation_message",
            "lifecycle_tracking_only_violation_message",
            "forbid_any_type_violation_message"
        ]

        for field in simple_fields + role_violation_fields:
            val = getattr(rule, field, None)
            if val is not None:
                update_data[field] = val

        # 2. Boolean/Status fields (override if not None)
        boolean_fields = [
            "no_primitives", "barrel_completeness", "forbid_internal_all",
            "mandatory_class_definition", "dead_inheritance_bypass", "check_orphan",
            "check_unused_mandatory_imports", "no_domain_logic",
            "must_implement_service_container_aggregate", "lazy_eager_initialization_only",
            "stateless_execution", "single_execution_goal", "high_level_policy_only",
            "coordinates_multiple_orchestrators", "crud_only", "no_decision_logic",
            "thread_async_safe", "no_domain_data_storage", "owns_system_health_transitions",
            "lifecycle_tracking_only",
            "forbid_any_type"
        ]

        for field in boolean_fields:
            val = getattr(rule, field, None)
            if val is not None:
                update_data[field] = val

        return update_data

    def _apply_exceptions(self, rule: ArchitectureRule):
        """Extract collection/exception fields from a rule (imports, suffixes, bypasses, etc.)."""
        update_data = {}
        collection_fields = [
            "allowed_import", "forbidden_import", "mandatory_import",
            "allowed_suffix", "forbidden_suffix", "exceptions",
            "forbidden_bypass", "orphan_entry_points",
            "mandatory_imports", "forbidden_bypass_custom_messages",
            "dead_inheritance_bypass_custom_messages"
        ]

        for field in collection_fields:
            val = getattr(rule, field, None)
            if val is not None:
                if hasattr(val, 'values') and val.values or isinstance(val, list) and val:
                    update_data[field] = val

        return update_data
