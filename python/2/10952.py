while True:
    a, b = input().split()
    a, b = int(a), int(b)

    '''
    if a != 0 or b != 0:
        print(a + b)
    else:
        break
        '''

    if a == 0 and b == 0:
        break
    else:
        print(a + b)

