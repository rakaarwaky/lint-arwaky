from auto_linter.capabilities.dependency_cycle_analyzer import _detect_cycle_edges


def test_detect_cycle_edges_no_cycle():
    edges = [
        {"source": "layerA", "target": "layerB"},
        {"source": "layerB", "target": "layerC"},
    ]
    res = _detect_cycle_edges(edges)
    assert len(res.values) == 0


def test_detect_cycle_edges_simple_cycle():
    edges = [
        {"source": "layerA", "target": "layerB"},
        {"source": "layerB", "target": "layerC"},
        {"source": "layerC", "target": "layerA"},
    ]
    res = _detect_cycle_edges(edges)
    assert len(res.values) > 0
    # Any of the back edges is a valid output
    valid_edges = ["layerA->layerB", "layerB->layerC", "layerC->layerA"]
    for val in res.values:
        assert val in valid_edges


def test_detect_cycle_edges_self_loop():
    edges = [
        {"source": "layerA", "target": "layerA"},
    ]
    res = _detect_cycle_edges(edges)
    assert len(res.values) == 1
    assert res.values[0] == "layerA->layerA"
