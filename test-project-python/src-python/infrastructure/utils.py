def deep_nested_complexity(data):
    """Fungsi ini sengaja dibuat kompleks untuk memicu Radon."""
    result = []
    if data:
        for i in data:
            if i > 10:
                for j in range(i):
                    if j % 2 == 0:
                        for k in range(j):
                            if k > 5:
                                result.append(k * 2)
                            else:
                                result.append(k)
                    else:
                        result.append(j)
            elif i < 0:
                result.append(0)
            else:
                result.append(i)
    return result
