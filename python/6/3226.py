def tick():
    time, delta = input().split()
    delta = int(delta)
    hour, minute = map(int, time.split(':'))
    
    result = 0
    for i in range(delta):
        h, m = hour, minute
        m += i
        
        if m >= 60:
            m -= 60
            h += 1
        
        if h >= 24:
            h -= 24
        
        
        if 7 <= h < 19:
            result += 10
        else:
            result += 5
        
    
    
    return result


def main():
    count = int(input())
    
    result = 0
    for _ in range(count):
        result += tick()
    
    
    print(result)



if __name__ == '__main__':
    main()


