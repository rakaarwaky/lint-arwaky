import sys

def main():
    user_input = sys.argv[1] if len(sys.argv) > 1 else "1+1"
    # DANGEROUS: eval() triggers Bandit B307/B102
    print(f"Result: {eval(user_input)}") 

if __name__ == "__main__":
    main()
