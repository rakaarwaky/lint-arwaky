def unsafe():
    pw = "secret_123" # BANDIT: Hardcoded password
    eval("print(1)") # BANDIT: Eval usage
