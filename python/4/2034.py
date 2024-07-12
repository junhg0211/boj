from sys import stdin

input = stdin.readline


possibles = [0, 2, 4, 5, 7, 9, 11]


def main():
    count = int(input())
    deltas = [int(input()) for _ in range(count)]
    
    result = list()
    for i, start in enumerate(possibles):
        value = start
        impossible = False
        for delta in deltas:
            value += delta
            value %= 12
            
            if value not in possibles:
                impossible = True
                break
            
        
        
        if impossible:
            continue
        
        
        result.append((start, value))
    
    
    prints = list()
    for resulton in result:
        a, b = resulton
        prints.append('CDEFGAB'[possibles.index(a)] + ' ' + 'CDEFGAB'[possibles.index(b)])
    
    
    print('\n'.join(sorted(prints)))



if __name__ == '__main__':
    main()


