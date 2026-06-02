def complex_af(n):
    for i in range(n):
        for j in range(i):
            if j % 2 == 0:
                for k in range(j):
                    if k > 5:
                        print(k)
                    else:
                        print("low")
            else:
                print("odd")
# RADON: High complexity due to deep nesting
