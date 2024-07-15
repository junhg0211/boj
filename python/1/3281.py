t9 = dict([
('A', 2), ('B', 2), ('C', 2),
('D', 3), ('E', 3), ('F', 3),
('G', 4), ('H', 4), ('I', 4),
('J', 5), ('K', 5), ('L', 5),
('M', 6), ('N', 6), ('O', 6),
('P', 7), ('Q', 7), ('R', 7), ('S', 7),
('T', 8), ('U', 8), ('V', 8),
('W', 9), ('X', 9), ('Y', 9), ('Z', 9),
])

def main():
    word_count = int(input())
    words = [input() for _ in range(word_count)]
    
    key_count = int(input())
    keys = list(map(int, input().split()))
    
    buffer = list()
    j = 0
    while j <= len(keys):
        if j < len(keys):
            key = keys[j]
        elif keys[-1] != 1:
            key = 1
        else:
            break
        
        
        if key != 1:
            buffer.append(key)
            j += 1
            continue
        
        
        good = False
        for word in words:
            if len(word) != len(buffer):
                continue
            
            
            wrong = False
            for i, letter in enumerate(word):
                if buffer[i] != t9[letter]:
                    wrong = True
                    break
                
            
            
            if not wrong:
                print(word, end=' ')
                good = True
                break
            
        
        
        if not good:
            print('*' * len(buffer), end=' ')
        
        
        buffer.clear()
        j += 1
    
    
    print()



if __name__ == '__main__':
    main()


