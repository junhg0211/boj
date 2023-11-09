def gcd(a, b):
    if a < b:
        return gcd(b, a)
    
    while b:
        a, b = b, a%b
    
    return a
    

a, b = map(int, input().split())
print(a * b // gcd(a, b))
