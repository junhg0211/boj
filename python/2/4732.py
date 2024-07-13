keys = dict([
('A', 0),

('Bb', 1),
('A#', 1),

('Cb', 2),
('B', 2),

('C', 3),
('B#', 3),

('Db', 4),
('C#', 4),

('D', 5),

('Eb', 6),
('D#', 6),

('Fb', 7),
('E', 7),

('E#', 8),
('F', 8),

('Gb', 9),
('F#', 9),

('G', 10),

('Ab', 11),
('G#', 11),
])

keys_rev = dict((v, k) for k, v in keys.items())

def main():
    while True:
        notes = input().split()
        
        if notes[0] == '***':
            break
        
        
        key_diff = int(input())
        
        for note in notes:
            print(keys_rev[(keys[note] + key_diff) % 12], end=' ')
        
        print()
    



if __name__ == '__main__':
    main()


